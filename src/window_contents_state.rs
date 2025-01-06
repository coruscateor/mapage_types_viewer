use std::cell::RefCell;

use std::rc::{Rc, Weak};

use std::any::Any;
use std::str::FromStr;
use std::time::Duration;

use act_rs::enter;
use corlib::cell::borrow_mut;

use corlib::upgrading::{try_up_rc, try_up_rc_pt, up_rc, up_rc_pt};

use corlib::NonOption;

use gtk_estate::adw::gdk::Display;
use gtk_estate::adw::glib::clone;

use gtk_estate::adw::prelude::{BoxExt, ButtonExt, Cast, ObjectExt, WidgetExt};

use gtk_estate::adw::{HeaderBar, WindowTitle};

use gtk_estate::corlib::{impl_as_any_ref, convert::AsAnyRef};

use gtk_estate::helpers::widget_ext::set_hvexpand_t;

use gtk_estate::{impl_weak_self_methods, scs_add, RcWidgetAdapter, StateContainers, StoredWidgetObject, WidgetAdapter, WidgetStateContainer, DynWidgetStateContainer, impl_widget_state_container_traits, RcSimpleTimeOut, SimpleTimeOut};

use gtk_estate::gtk4::{Box, Button, CenterBox, DropDown, Orientation, Paned, ScrolledWindow, StringObject, TextView, prelude::{TextViewExt, TextBufferExt}};

use gtk_estate::gtk4::glib;
use libsync::crossbeam::mpmc::tokio::array_queue::io_channels::IOClient;
use serde::ser::Error;
use serde_json::to_string_pretty;

use crate::actors::{MapageTypeActorInputMessage, MapageTypeActorOutputMessage, MapageTypeActorState};

use crate::{AllOrNot, ApplicationState, SupportedType, SupportedTypeSubContents};

use crate::widgets::{new_mapage_type_strs_dropdown, new_supported_type_strs_dropdown, output_format_strs_dropdown, MapageType, OutputFormat};

use corlib::cell::RefCellStore;

struct WindowContentsMutState
{

    output_format: OutputFormat,
    all_or_not_mapage_type: AllOrNot<MapageType>,

}

impl WindowContentsMutState
{

    pub fn new() -> Self
    {

        Self
        {

            output_format: OutputFormat::Json,
            all_or_not_mapage_type: AllOrNot::All

        }

    }

}

pub struct WindowContentsState
{

    widget_adapter: Rc<WidgetAdapter<Box, WindowContentsState>>,
    mapage_types_dropdown: DropDown,
    text_output: TextView,
    mut_state: RefCellStore<WindowContentsMutState>,
    io_client: IOClient<MapageTypeActorInputMessage, MapageTypeActorOutputMessage>,
    actor_poller: RcSimpleTimeOut<Weak<WindowContentsState>>,
    output_format_dropdown: DropDown,
    run_button: Button,
    supported_type_sub_contents: Rc<SupportedTypeSubContents>

}

impl WindowContentsState
{

    pub fn new() -> Rc<Self>
    {

        //Layout

        let contents_box = Box::new(Orientation::Vertical, 0);

        contents_box.set_vexpand(true);

        let window_title = WindowTitle::new("Mapage Types Viewer", "");

        let hb = HeaderBar::builder().title_widget(&window_title).build();

        contents_box.append(&hb);

        let tool_cb = CenterBox::new();

        //Left
        
        let tool_left_box = Box::new(Orientation::Horizontal, 5);

        tool_left_box.set_margin_end(10);

        let output_format_dropdown = output_format_strs_dropdown();

        tool_left_box.append(&output_format_dropdown);

        tool_cb.set_start_widget(Some(&tool_left_box));

        //Center

        let tool_center_box = Box::new(Orientation::Horizontal, 2);

        let run_button = Button::builder().label("Run").build();

        tool_center_box.append(&run_button);

        tool_cb.set_center_widget(Some(&tool_center_box));

        //Right

        let tool_right_box = Box::new(Orientation::Horizontal, 5);

        tool_cb.set_end_widget(Some(&tool_right_box));

        //

        contents_box.append(&tool_cb);

        //
        
        let contents_paned = Paned::new(Orientation::Horizontal);

        contents_paned.set_vexpand(true);

        //

        let input_contents_box = Box::new(Orientation::Vertical, 20);

        input_contents_box.set_margin_top(10);

        let mapage_types_dropdown = new_mapage_type_strs_dropdown();

        //mapage_types_dropdown.set_hexpand_set(true);

        //mapage_types_dropdown.set_hexpand(false);

        //input_contents_box.append(&mapage_types_dropdown);

        let mapage_types_box = Box::builder().orientation(Orientation::Horizontal).spacing(2).build();

        mapage_types_box.append(&mapage_types_dropdown);

        mapage_types_dropdown.set_width_request(180);

        input_contents_box.append(&mapage_types_box);

        //SupportedType

        let supported_type_sub_contents = SupportedTypeSubContents::new();

        //input_contents_box.append(&supported_type_sub_contents);

        input_contents_box.append(supported_type_sub_contents.widget_ref());

        let input_contents_box_sw = ScrolledWindow::builder().child(&input_contents_box).build();

        contents_paned.set_start_child(Some(&input_contents_box_sw));

        let output_contents_box = Box::new(Orientation::Vertical, 0);

        let text_output = TextView::builder().editable(false).build();

        text_output.set_vexpand(true);

        output_contents_box.append(&text_output);

        let output_contents_box_sw = ScrolledWindow::builder().child(&output_contents_box).build();

        output_contents_box_sw.set_child(Some(&output_contents_box));

        contents_paned.set_end_child(Some(&output_contents_box_sw));

        //

        contents_box.append(&contents_paned);

        let scs = StateContainers::get();

        let tokio_rt_handle;

        {

            tokio_rt_handle = scs.dyn_application_state_ref(|app_state_ref: &ApplicationState|
            {

                app_state_ref.tokio_rt_handle()

            }).expect("Error: Not ApplicattionState!");

        }

        let io_client = enter!(tokio_rt_handle, MapageTypeActorState::spawn());

        let actor_poller_duration = Duration::from_micros(1);

        //Duration::new(1, 0)

        //Content state initialisation

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                widget_adapter: WidgetAdapter::new(&contents_box, weak_self),
                mapage_types_dropdown,
                text_output,
                mut_state: RefCellStore::new(WindowContentsMutState::new()),
                io_client,
                actor_poller: SimpleTimeOut::with_state_ref(actor_poller_duration, weak_self),
                output_format_dropdown,
                run_button,
                supported_type_sub_contents

            }

        });

        scs_add!(this);

        let weak_self = this.weak_self();

        this.supported_type_sub_contents.on_supported_type_str_selected().subscribe(&weak_self, |_sender, parent|
        {

            parent.text_output.buffer().set_text("");

        });

        //Signal connection

        let weak_self_moved = weak_self.clone();

        this.output_format_dropdown.connect_selected_notify(move |format_dropdown|
        {

            try_up_rc(&weak_self_moved, |this|
            {

                if let Some(item) = format_dropdown.selected_item()
                {

                    if let Some(item) = item.downcast_ref::<StringObject>()
                    {

                        let item_string = item.string();

                        let from_str_res = OutputFormat::from_str(&item_string);

                        match from_str_res
                        {

                            Ok(res) =>
                            {

                                this.mut_state.borrow_mut(|mut state|
                                {

                                    state.output_format = res;

                                    this.text_output.buffer().set_text("");

                                })

                            }
                            Err(err) =>
                            {

                                this.output_error(err);

                            }

                        }

                    }

                }

            });

        });

        //

        let weak_self_moved = weak_self.clone();

        this.mapage_types_dropdown.connect_selected_notify(move |mapage_types_dropdown|
        {

            try_up_rc(&weak_self_moved, |this|
            {

                if let Some(item) = mapage_types_dropdown.selected_item()
                {

                    if let Some(item) = item.downcast_ref::<StringObject>()
                    {

                        let item_string = item.string();

                        if item_string == "*"
                        {

                            this.mut_state.borrow_mut(|mut state|
                            {

                                state.all_or_not_mapage_type = AllOrNot::All;

                                this.text_output.buffer().set_text("");

                            });

                        }
                        else
                        {

                            let from_str_res = MapageType::from_str(&item_string);

                            match from_str_res
                            {
    
                                Ok(res) =>
                                {
    
                                    this.mut_state.borrow_mut(|mut state|
                                    {
    
                                        state.all_or_not_mapage_type = AllOrNot::NotAll(res);
    
                                        this.text_output.buffer().set_text("");

                                        match res
                                        {

                                            MapageType::SupportedType =>
                                            {

                                                this.supported_type_sub_contents.widget_ref().set_visible(true);

                                            }
                                            MapageType::Whatever =>
                                            {

                                                this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            }
                                            MapageType::TypeInstance =>
                                            {

                                                this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            }
                                            MapageType::Command =>
                                            {

                                                this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            }
                                            MapageType::CommandResult =>
                                            {

                                                this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            }
                                            MapageType::CommandError =>
                                            {

                                                this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            }
                                            MapageType::StreamedMessage =>
                                            {

                                                this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            }

                                        }
    
                                    })
    
                                }
                                Err(err) =>
                                {
    
                                    this.output_error(err);
    
                                }
    
                            }
                            
                        }

                    }

                }

            });

        });

        //

        let weak_self_moved = weak_self.clone();

        this.run_button.connect_clicked(move |run_button|
        {

            try_up_rc(&weak_self_moved, |this|
            {

                if this.actor_poller.is_active()
                {

                    return;

                }

                this.mut_state.borrow(|state|
                {

                    match state.all_or_not_mapage_type
                    {

                        AllOrNot::All =>
                        {



                        }
                        AllOrNot::NotAll(mapage_type) =>
                        {

                            match mapage_type
                            {

                                MapageType::SupportedType => todo!(),
                                MapageType::Whatever => todo!(),
                                MapageType::TypeInstance => todo!(),
                                MapageType::Command => todo!(),
                                MapageType::CommandResult => todo!(),
                                MapageType::CommandError => todo!(),
                                MapageType::StreamedMessage => todo!(),
                                
                            }

                        }

                    }

                    let input_message = MapageTypeActorInputMessage::ProcessSupportedType(state.output_format, this.supported_type_sub_contents.all_or_not_supported_type()); //state.supported_type); //state.mapage_type,

                    let try_send_res = this.io_client.input_sender_ref().try_send(input_message);

                    if let Err(err) = try_send_res
                    {

                        this.text_output.buffer().set_text(&err.to_string());

                    }

                    this.actor_poller.start();

                    run_button.set_sensitive(false);
                    
                })

            });

        });

        this.actor_poller.set_on_time_out_fn(|sto|
        {

            try_up_rc_pt(sto.state(), |this|
            {

                let receiver = this.io_client.output_receiver_ref();

                match receiver.try_recv()
                {

                    Ok(res) =>
                    {

                        match res
                        {

                            MapageTypeActorOutputMessage::WorkInProgressTextResult(work_in_progress_result) =>
                            {

                                if let Some(res) = work_in_progress_result.result()
                                {

                                    let mut end_iter = this.text_output.buffer().end_iter();

                                    this.text_output.buffer().insert(&mut end_iter, res);

                                }

                                let is_done = work_in_progress_result.is_done();

                                if is_done
                                {

                                    this.run_button.set_sensitive(true);

                                }

                                !is_done

                            }

                        }

                    }
                    Err(err) =>
                    {

                        let err_string = format!("\n\n{err:?}\n\n");

                        let mut end_iter = this.text_output.buffer().end_iter();

                        this.text_output.buffer().insert(&mut end_iter, &err_string);

                        true

                    }
                    
                }

            })

        });

        this

    }

    impl_weak_self_methods!(widget_adapter);

    fn output_todo(&self)
    {

        self.text_output.buffer().set_text("todo");

    }

    fn output_unrecognised_selection_error(&self)
    {

        self.text_output.buffer().set_text("Error: Unrecognised Selection Error");

    }

    pub fn output_error<E>(&self, error: E)
        where E: std::error::Error
    {

        self.text_output.buffer().set_text(&error.to_string());

    }

    pub fn show_supported_type_widget(&self)
    {



    }

    pub fn show_whatever_widget(&self)
    {



    }

    pub fn show_type_instance_widget(&self)
    {



    }

    pub fn show_command_widget(&self)
    {



    }

    pub fn show_command_result_widget(&self)
    {



    }

    pub fn show_command_error_widget(&self)
    {



    }

    pub fn show_streamed_message_widget(&self)
    {



    }

}

impl_widget_state_container_traits!(Box, WindowContentsState);
