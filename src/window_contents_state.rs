use std::cell::RefCell;

use std::rc::{Rc, Weak};

use std::any::Any;
use std::str::FromStr;
use std::time::Duration;

use act_rs::enter;
use corlib::cell::borrow_mut;

use corlib::inc_dec::IncDecSelf;
use corlib::upgrading::{try_up_rc, try_up_rc_pt, up_rc, up_rc_pt};

use corlib::value::HasValueGetter;
use corlib::NonOption;

use gtk_estate::adw::gdk::Display;

//use gtk_estate::adw::glib::{self, clone};

//use gtk_estate::adw::glib;

//use gtk_estate::adw::glib_macros::clone;

//use gtk_estate::adw::glib::macros;

use gtk_estate::adw::prelude::{BoxExt, ButtonExt, Cast, ObjectExt, WidgetExt};

use gtk_estate::adw::{Application, ApplicationWindow, HeaderBar, WindowTitle};

use gtk_estate::corlib::{impl_as_any_ref, convert::AsAnyRef, weak_self::WeakSelf};

use gtk_estate::gtk::{Align, Label};
use gtk_estate::helpers::widget_ext::set_hvexpand_t;

use gtk_estate::{impl_widget_state_container_traits, scs_add, widget_upgrade_error_debug_println, DynWidgetStateContainer, StateContainers, TimeOut, TimeOutRunType, WidgetAdapter, WidgetContainer, WidgetObject, WidgetStateContainer};

//impl_weak_self_methods, 

use gtk_estate::gtk::{Box, Button, CenterBox, DropDown, Orientation, Paned, ScrolledWindow, StringObject, TextView, prelude::{TextViewExt, TextBufferExt}};

use gtk_estate::gtk::glib;

use gtk_estate::gtk::glib::clone;

use libsync::crossbeam::mpmc::tokio::array_queue::io_channels::IOClient;
use libsync::BoundedSendError;
use serde::ser::Error;
use serde_json::to_string_pretty;

use crate::actors::{MapageTypeActorInputMessage, MapageTypeActorOutputMessage, MapageTypeActorState};

use crate::{AllOrNot, AllOrNotSupportedTypeSubContents, AllOrNotTypeInstanceSubContents, AllOrNotWhateverSubContents, ApplicationState, CommandErrorSubContents, CommandResultSubContents, CommandSubContents, StreamedMessageSubContents, SupportedType};

use crate::widgets::{new_mapage_type_strs_dropdown, new_supported_type_strs_dropdown, output_format_strs_dropdown, MapageType, OutputFormat};

use corlib::cell::RefCellStore;

//use gtk::glib;

//rust-analyzer version: 0.3.2249-standalone

//When using glib clone!( #[strong] this,... old error underlines don't get removed. 

#[derive(Debug)]
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

#[derive(Debug)]
pub struct WindowContentsState
{

    widget_adapter: Rc<WidgetAdapter<ApplicationWindow, Self>>,
    //widget_adapter: Rc<WidgetAdapter<Box, Self>>,
    mapage_types_dropdown: DropDown,
    text_output: TextView,
    mut_state: RefCellStore<WindowContentsMutState>,
    io_client: IOClient<MapageTypeActorInputMessage, MapageTypeActorOutputMessage>,
    actor_poller: TimeOut<Self>,
    output_format_dropdown: DropDown,
    run_button: Button,
    supported_type_sub_contents: Rc<AllOrNotSupportedTypeSubContents<Self>>,
    new_window_button: Button,
    whatever_sub_contents: Rc<AllOrNotWhateverSubContents<Self>>,
    type_instance_sub_contents: Rc<AllOrNotTypeInstanceSubContents<Self>>,
    command_sub_contents: Rc<CommandSubContents>,
    command_result_sub_contents: Rc<CommandResultSubContents>,
    command_error_sub_contents: Rc<CommandErrorSubContents>,
    streamed_message_sub_contents: Rc<StreamedMessageSubContents>

}

impl WindowContentsState
{

    pub fn new(app: &Application) -> Rc<Self>
    {

        //Layout

        let contents_box = Box::new(Orientation::Vertical, 0);

        contents_box.set_vexpand(true);

        let window_title = WindowTitle::new("Mapage Types Viewer", "");

        let hb = HeaderBar::builder().title_widget(&window_title).build();

        contents_box.append(&hb);

        //New Window

        let new_window_button = Button::builder().label("New Window").build();

        contents_box.append(&new_window_button);

        //

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

        //

        let mapage_types_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let mapage_types_label = Label::builder().label("MapageType").halign(Align::Start).build();

        //input_contents_box.append(&mapage_types_label);

        mapage_types_box.append(&mapage_types_label);

        //

        let mapage_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(true).build();

        let mapage_types_dropdown = new_mapage_type_strs_dropdown();

        mapage_types_dropdown.set_width_request(180);

        mapage_strs_dropdown_box.append(&mapage_types_dropdown);

        //mapage_types_dropdown.set_hexpand_set(true);

        //mapage_types_dropdown.set_hexpand(false);

        //input_contents_box.append(&mapage_types_dropdown);

        mapage_types_box.append(&mapage_strs_dropdown_box);

        //

        input_contents_box.append(&mapage_types_box);

        //SupportedType

        let supported_type_sub_contents = AllOrNotSupportedTypeSubContents::new();

        //input_contents_box.append(&supported_type_sub_contents);

        input_contents_box.append(supported_type_sub_contents.widget_ref());

        //Whatever

        let whatever_sub_contents = AllOrNotWhateverSubContents::new();

        input_contents_box.append(whatever_sub_contents.widget_ref());

        //TypeInstance

        let type_instance_sub_contents = AllOrNotTypeInstanceSubContents::new();

        input_contents_box.append(type_instance_sub_contents.widget_ref());

        //Command

        let command_sub_contents = CommandSubContents::new();

        input_contents_box.append(command_sub_contents.widget_ref());

        //CommandResult

        let command_result_sub_contents = CommandResultSubContents::new();

        input_contents_box.append(command_result_sub_contents.widget_ref());

        //CommandError

        let command_error_sub_contents = CommandErrorSubContents::new();

        input_contents_box.append(command_error_sub_contents.widget_ref());

        //StreamedMessage

        let streamed_message_sub_contents = StreamedMessageSubContents::new();

        input_contents_box.append(streamed_message_sub_contents.widget_ref());

        //

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

        //ApplicationWindow

        let builder = ApplicationWindow::builder();

        //ApplicationWindow strong reference is in the Application.

        let window = builder.application(app)
            .default_width(1000)
            .default_height(1000)

            //Make sure to set the content of the ApplicationWindow.

            .content(&contents_box)
            .visible(true)
            //.hide_on_close(false)
            .build();

        //Tokio

        let scs = StateContainers::get();

        let tokio_rt_handle;

        {

            tokio_rt_handle = scs.application_state_ref_func(|app_state_ref: &ApplicationState|
            {

                app_state_ref.tokio_rt_handle()

            }) //.expect("Error: Not ApplicationState!");

        }

        let io_client = enter!(tokio_rt_handle, MapageTypeActorState::spawn());

        let actor_poller_duration = Duration::from_micros(1);

        //let time_out_run_type = TimeOutRunType::Milliseconds(actor_poller_duration);

        //Duration::new(1, 0)

        //Content state initialisation

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                widget_adapter: WidgetAdapter::new(&window, weak_self),
                //widget_adapter: WidgetAdapter::new(&contents_box, weak_self),
                mapage_types_dropdown,
                text_output,
                mut_state: RefCellStore::new(WindowContentsMutState::new()),
                io_client,
                actor_poller: TimeOut::milliseconds(actor_poller_duration, weak_self), //TimeOut::new(time_out_run_type, weak_self),
                output_format_dropdown,
                run_button,
                supported_type_sub_contents,
                new_window_button,
                whatever_sub_contents,
                type_instance_sub_contents,
                command_sub_contents,
                command_result_sub_contents,
                command_error_sub_contents,
                streamed_message_sub_contents

            }

        });

        let _ = scs.widget_state_ref().add(&this);

        //scs_add!(this);
        
        let weak_self = this.weak_self();

        //type_instance_sub_contents

        this.type_instance_sub_contents.on_type_instance_str_selected().subscribe(&weak_self, |_sender, this|
        {

            this.clear_text_output();

        });

        this.type_instance_sub_contents.on_value_input_parse_error().subscribe(&weak_self, |_sender, message, this|
        {

            this.set_text_output_text(message)

        });

        //whatever_sub_contents

        this.whatever_sub_contents.on_whatever_str_selected().subscribe(&weak_self, |_sender, this| //all_or_not, 
        {

            this.clear_text_output();

            //parent.text_output.buffer().set_text("");

        });

        this.whatever_sub_contents.on_value_input_parse_error().subscribe(&weak_self, |_sender, message, this| //event_arg,
        {

            this.set_text_output_text(message)

            /*
            if let Err(message) = &this.whatever_sub_contents.all_or_not_whatever()
            {

                this.text_output.buffer().set_text(message); //event_arg);

            }
            */

        });

        //supported_type_sub_contents.

        this.supported_type_sub_contents.on_supported_type_str_selected().subscribe(&weak_self, |_sender, this|
        {

            this.clear_text_output();

            //parent.text_output.buffer().set_text("");

        });

        //clone!( #[strong] this, move 

        this.new_window_button.connect_clicked(|_button|
        {

            StateContainers::get().application_state_ref_func(|application_state: &ApplicationState|
            {

                widget_upgrade_error_debug_println(application_state.new_window());

            });

            //this.

        }); //);

        //Signal connections

        //this.

        //let weak_self_moved = weak_self.clone();

        //let this_moved = this.clone();

        //let this2 = this.clone();

        //clone!( #[strong] this,

        this.output_format_dropdown.connect_selected_notify(clone!( #[weak] this, move |format_dropdown|
        {

            //let this = this_moved;

            //try_up_rc(&weak_self_moved, |this|
            //

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

                                //this.text_output.buffer().set_text("");

                                this.clear_text_output();

                            })

                        }
                        Err(err) =>
                        {

                            this.output_error(err);

                        }

                    }

                }

            }

            //} //);

        }));

        //

        //let this_moved = this.clone(); 

        //let weak_self_moved = weak_self.clone();

        //clone!( #[strong] this, 

        //let this2 = this.clone();

        this.mapage_types_dropdown.connect_selected_notify(clone!( #[weak] this, move |mapage_types_dropdown|
        {

            //try_up_rc(&weak_self_moved, |this|
            //{

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

                            //this.text_output.buffer().set_text("");

                            this.clear_text_output();

                            //Make all type input widgets visible.

                            this.supported_type_sub_contents.widget_ref().set_visible(true);

                            this.whatever_sub_contents.widget_ref().set_visible(true);

                            this.type_instance_sub_contents.widget_ref().set_visible(true);

                            this.command_sub_contents.widget_ref().set_visible(true);

                            this.command_result_sub_contents.widget_ref().set_visible(true);

                            this.command_error_sub_contents.widget_ref().set_visible(true);

                            this.streamed_message_sub_contents.widget_ref().set_visible(true);

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

                                    //this.text_output.buffer().set_text("");

                                    this.clear_text_output();

                                    match res
                                    {

                                        MapageType::SupportedType =>
                                        {

                                            this.supported_type_sub_contents.widget_ref().set_visible(true);

                                            this.whatever_sub_contents.widget_ref().set_visible(false);

                                            this.type_instance_sub_contents.widget_ref().set_visible(false);

                                            this.command_sub_contents.widget_ref().set_visible(false);

                                            this.command_result_sub_contents.widget_ref().set_visible(false);

                                            this.command_error_sub_contents.widget_ref().set_visible(false);

                                            this.streamed_message_sub_contents.widget_ref().set_visible(false);

                                        }
                                        MapageType::Whatever =>
                                        {

                                            this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            this.whatever_sub_contents.widget_ref().set_visible(true);

                                            this.type_instance_sub_contents.widget_ref().set_visible(false);

                                            this.command_sub_contents.widget_ref().set_visible(false);

                                            this.command_result_sub_contents.widget_ref().set_visible(false);

                                            this.command_error_sub_contents.widget_ref().set_visible(false);

                                            this.streamed_message_sub_contents.widget_ref().set_visible(false);

                                        }
                                        MapageType::TypeInstance =>
                                        {

                                            this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            this.whatever_sub_contents.widget_ref().set_visible(false);

                                            this.type_instance_sub_contents.widget_ref().set_visible(true);

                                            this.command_sub_contents.widget_ref().set_visible(false);

                                            this.command_result_sub_contents.widget_ref().set_visible(false);

                                            this.command_error_sub_contents.widget_ref().set_visible(false);

                                            this.streamed_message_sub_contents.widget_ref().set_visible(false);

                                        }
                                        MapageType::Command =>
                                        {

                                            this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            this.whatever_sub_contents.widget_ref().set_visible(false);

                                            this.type_instance_sub_contents.widget_ref().set_visible(false);

                                            this.command_sub_contents.widget_ref().set_visible(true);

                                            this.command_result_sub_contents.widget_ref().set_visible(false);

                                            this.command_error_sub_contents.widget_ref().set_visible(false);

                                            this.streamed_message_sub_contents.widget_ref().set_visible(false);

                                        }
                                        MapageType::CommandResult =>
                                        {

                                            this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            this.whatever_sub_contents.widget_ref().set_visible(false);

                                            this.type_instance_sub_contents.widget_ref().set_visible(false);

                                            this.command_sub_contents.widget_ref().set_visible(false);

                                            this.command_result_sub_contents.widget_ref().set_visible(true);

                                            this.command_error_sub_contents.widget_ref().set_visible(false);

                                            this.streamed_message_sub_contents.widget_ref().set_visible(false);

                                        }
                                        MapageType::CommandError =>
                                        {

                                            this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            this.whatever_sub_contents.widget_ref().set_visible(false);

                                            this.type_instance_sub_contents.widget_ref().set_visible(false);

                                            this.command_sub_contents.widget_ref().set_visible(false);

                                            this.command_result_sub_contents.widget_ref().set_visible(false);

                                            this.command_error_sub_contents.widget_ref().set_visible(true);

                                            this.streamed_message_sub_contents.widget_ref().set_visible(false);

                                        }
                                        MapageType::StreamedMessage =>
                                        {

                                            this.supported_type_sub_contents.widget_ref().set_visible(false);

                                            this.whatever_sub_contents.widget_ref().set_visible(false);

                                            this.type_instance_sub_contents.widget_ref().set_visible(false);

                                            this.command_sub_contents.widget_ref().set_visible(false);

                                            this.command_result_sub_contents.widget_ref().set_visible(false);

                                            this.command_error_sub_contents.widget_ref().set_visible(false);

                                            this.streamed_message_sub_contents.widget_ref().set_visible(true);

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

            //} //);

        }));

        //

        //let this_moved = this.clone(); 

        //let weak_self_moved = weak_self.clone();

        //clone!(#[strong] this,

        //let this2 = this.clone();
        
        this.run_button.connect_clicked(clone!( #[weak] this, move |run_button|
        {

            //try_up_rc(&weak_self_moved, |this|
            //{

            if this.actor_poller.is_active()
            {

                return;

            }

            let mut should_run = true;

            //let try_send_res;

            //let mut sent_messages_count = 0;

            this.mut_state.borrow(|state|
            {

                match state.all_or_not_mapage_type
                {

                    AllOrNot::All =>
                    {

                        this.output_when_error(this.send_process_all_message(&state));

                        //this.output_when_error(this.send_process_supported_type_message(&state));

                        //sent_messages_count.pp();

                        //this.output_when_error(this.send_process_whatever_message(&state));

                        //sent_messages_count.pp();

                    }
                    AllOrNot::NotAll(mapage_type) =>
                    {

                        match mapage_type
                        {

                            MapageType::SupportedType =>
                            {

                                this.output_when_error(this.send_process_supported_type_message(&state));

                                //sent_messages_count.pp();

                            }
                            MapageType::Whatever =>
                            {

                                should_run = this.output_when_error(this.send_process_whatever_message(&state));

                                //sent_messages_count.pp();

                            }
                            MapageType::TypeInstance =>
                            {

                                should_run = this.output_when_error(this.send_process_type_instance_message(&state));

                            }
                            MapageType::Command =>
                            {

                                should_run = this.output_when_error(this.send_process_command_message(&state));

                            }
                            MapageType::CommandResult =>
                            {

                                should_run = this.output_when_error(this.send_process_command_result_message(&state));

                            }
                            MapageType::CommandError =>
                            {

                                should_run = this.output_when_error(this.send_process_command_error_message(&state));

                            }
                            MapageType::StreamedMessage =>
                            {

                                should_run = this.output_when_error(this.send_process_streamed_message_message(&state));

                            }
                            
                        }

                    }

                }

                //let input_message = MapageTypeActorInputMessage::ProcessSupportedType(state.output_format, this.supported_type_sub_contents.all_or_not_supported_type()); //state.supported_type); //state.mapage_type,

                //let try_send_res = this.io_client.input_sender_ref().try_send(input_message);

                /*
                if let Err(err) = try_send_res
                {

                    this.text_output.buffer().set_text(&err.to_string());

                }
                */

                //if sent_messages_count > 0
                //{
                
                if should_run
                {

                    this.clear_text_output();

                    this.actor_poller.start();
    
                    run_button.set_sensitive(false);

                }

                //}
                    
            })

            //} //);

        }));

        this.actor_poller.set_time_out_fn(Rc::new(|this: Rc<Self>|
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

                                //print!("received:\n\n");

                                //print!("{}", res);

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

        }));

        this

    }

    //impl_weak_self_methods!(widget_adapter);

    fn set_text_output_text(&self, text: &str)
    {

        self.text_output.buffer().set_text(text);

    }

    pub fn output_error<E>(&self, error: E)
        where E: std::error::Error
    {

        self.text_output.buffer().set_text(&error.to_string());

    }

    pub fn output_when_error<T>(&self, result: Result<T, BoundedSendError<MapageTypeActorInputMessage>>) -> T
        where T: Default
    {

        match result
        {

            Ok(res) =>
            {

                res

            }
            Err(err) =>
            {

                self.text_output.buffer().set_text(&err.to_string());

                T::default()

            }

        }

        /*
        if let Err(err) = result
        {

            self.text_output.buffer().set_text(&err.to_string());

        }
        */

    }

    fn send_process_all_message(&self, state: &WindowContentsMutState) -> Result<(), BoundedSendError<MapageTypeActorInputMessage>>
    {

        let all_or_not_supported_type = self.supported_type_sub_contents.value();

        let all_or_not_whatever;

        let all_or_not_type_instance;

        if let Ok(res) = self.whatever_sub_contents.value()
        {

            all_or_not_whatever = res;

        }
        else
        {

            //Output error message

            return Ok(());
            
        }

        if let Ok(res) = self.type_instance_sub_contents.value()
        {

            all_or_not_type_instance = res;

        }
        else
        {

            //Output error message

            return Ok(());
            
        }

        let input_message = MapageTypeActorInputMessage::ProcessAll(state.output_format, all_or_not_supported_type, all_or_not_whatever, all_or_not_type_instance);

        self.io_client.input_sender_ref().try_send(input_message)

    }

    fn send_process_all_default_message(&self, state: &WindowContentsMutState) -> Result<(), BoundedSendError<MapageTypeActorInputMessage>>
    {

        let input_message = MapageTypeActorInputMessage::ProcessAllDefault(state.output_format);

        self.io_client.input_sender_ref().try_send(input_message)

    }


    fn send_process_supported_type_message(&self, state: &WindowContentsMutState) -> Result<(), BoundedSendError<MapageTypeActorInputMessage>>
    {

        let input_message = MapageTypeActorInputMessage::ProcessSupportedType(state.output_format, self.supported_type_sub_contents.value());

        self.io_client.input_sender_ref().try_send(input_message)

    }

    fn send_process_whatever_message(&self, state: &WindowContentsMutState) -> Result<bool, BoundedSendError<MapageTypeActorInputMessage>>
    {

        match self.whatever_sub_contents.value()
        {

            Ok(res) =>
            {

                let input_message = MapageTypeActorInputMessage::ProcessWhatever(state.output_format, res);

                if let Err(err) = self.io_client.input_sender_ref().try_send(input_message)
                {

                    Err(err)

                }
                else
                {

                    Ok(true)

                }

            }
            Err(err) =>
            {

                self.set_text_output_text(&err);

                Ok(false)

            }

        }

        //let input_message = MapageTypeActorInputMessage::ProcessWhatever(state.output_format, self.whatever_sub_contents.all_or_not_whatever());

        //self.io_client.input_sender_ref().try_send(input_message)

    }

    fn send_process_type_instance_message(&self, state: &WindowContentsMutState) -> Result<bool, BoundedSendError<MapageTypeActorInputMessage>>
    {

        match self.type_instance_sub_contents.value()
        {

            Ok(res) =>
            {

                let input_message = MapageTypeActorInputMessage::ProcessTypeInstance(state.output_format, res);

                if let Err(err) = self.io_client.input_sender_ref().try_send(input_message)
                {

                    Err(err)

                }
                else
                {

                    Ok(true)

                }

            }
            Err(err) =>
            {

                self.set_text_output_text(&err);

                Ok(false)

            }

        }

    }

    fn send_process_command_message(&self, state: &WindowContentsMutState) -> Result<bool, BoundedSendError<MapageTypeActorInputMessage>>
    {

        match self.command_sub_contents.value()
        {

            Ok(res) =>
            {

                let input_message = MapageTypeActorInputMessage::ProcessCommand(state.output_format, res);

                if let Err(err) = self.io_client.input_sender_ref().try_send(input_message)
                {

                    Err(err)

                }
                else
                {

                    Ok(true)

                }

            }
            Err(err) =>
            {

                self.set_text_output_text(&err);

                Ok(false)

            }

        }

    }

    fn send_process_command_result_message(&self, state: &WindowContentsMutState) -> Result<bool, BoundedSendError<MapageTypeActorInputMessage>>
    {

        match self.command_result_sub_contents.value()
        {

            Ok(res) =>
            {

                let input_message = MapageTypeActorInputMessage::ProcessCommandResult(state.output_format, res);

                if let Err(err) = self.io_client.input_sender_ref().try_send(input_message)
                {

                    Err(err)

                }
                else
                {

                    Ok(true)

                }

            }
            Err(err) =>
            {

                self.set_text_output_text(&err);

                Ok(false)

            }

        }

    }

    fn send_process_command_error_message(&self, state: &WindowContentsMutState) -> Result<bool, BoundedSendError<MapageTypeActorInputMessage>>
    {

        match self.command_error_sub_contents.value()
        {

            Ok(res) =>
            {

                let input_message = MapageTypeActorInputMessage::ProcessCommandError(state.output_format, res);

                if let Err(err) = self.io_client.input_sender_ref().try_send(input_message)
                {

                    Err(err)

                }
                else
                {

                    Ok(true)

                }

            }
            Err(err) =>
            {

                self.set_text_output_text(&err);

                Ok(false)

            }

        }

    }

    fn send_process_streamed_message_message(&self, state: &WindowContentsMutState) -> Result<bool, BoundedSendError<MapageTypeActorInputMessage>>
    {

        match self.streamed_message_sub_contents.value()
        {

            Ok(res) =>
            {

                let input_message = MapageTypeActorInputMessage::ProcessStreamedMessage(state.output_format, res);

                if let Err(err) = self.io_client.input_sender_ref().try_send(input_message)
                {

                    Err(err)

                }
                else
                {

                    Ok(true)

                }

            }
            Err(err) =>
            {

                self.set_text_output_text(&err);

                Ok(false)

            }

        }

    }

    fn clear_text_output(&self)
    {

        self.text_output.buffer().set_text("");

    }

    /*
    fn output_todo(&self)
    {

        self.text_output.buffer().set_text("todo");

    }

    fn output_unrecognised_selection_error(&self)
    {

        self.text_output.buffer().set_text("Error: Unrecognised Selection Error");

    }
    */

    /*
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
    */
    
}

impl Drop for WindowContentsState
{

    fn drop(&mut self)
    {
        
        println!("WindowContentsState Dropped!")

    }

}

impl_widget_state_container_traits!(ApplicationWindow, WindowContentsState); //Box, 
