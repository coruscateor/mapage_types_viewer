use std::{cell::Cell, ops::Deref, process::Command, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::gtk4::{prelude::{BoxExt, Cast, WidgetExt}, CheckButton, Text};

use crate::{widgets::new_supported_type_strs_dropdown, AllOrNot, SupportedType, SupportedTypeSubContents, WindowContentsState};

use corlib::{cell::RefCellStore, events::PubSingleSubEvent, impl_pub_single_sub_event_method, upgrading::try_up_rc};

use corlib::events::SingleSubEvent; 

use gtk_estate::gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, Widget};

use gtk_estate::gtk4::glib::clone;

pub struct CommandSubContents
{

    result_command: RefCellStore<Result<Command, String>>,
    id_text: Text,
    type_name_check_button: CheckButton,
    type_name_sub_contents: SupportedTypeSubContents<Self>,
    params_check_button: CheckButton,

}

impl CommandSubContents
{

    pub fn new() -> Rc<Self>
    {

        let command_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        //

        let id_text_label = Label::builder().label("id").halign(Align::Start).build();

        command_box.append(&id_text_label);

        //

        let id_text = Text::new();

        command_box.append(&id_text);

        //

        let supported_type_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(true).build();

        supported_type_strs_dropdown_box.append(&supported_type_strs_dropdown);

        command_box.append(&supported_type_strs_dropdown_box);

        //

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                supported_type_strs_dropdown,
                command_box,
                all_or_not_supported_type: Cell::new(AllOrNot::All),
                on_supported_type_str_selected: SingleSubEvent::new(weak_self)

            }
        
        });

        //let weak = this.downgrade();

        this.supported_type_strs_dropdown.connect_selected_item_notify(clone!( #[strong] this, move |supported_type_strs_dropdown|
        {

            //try_up_rc(&weak, |this|
            //{

            if let Some(item) = supported_type_strs_dropdown.selected_item()
            {

                if let Some(item) = item.downcast_ref::<StringObject>()
                {

                    let item_string = item.string();

                    if item_string == "*"
                    {

                        this.all_or_not_supported_type.set(AllOrNot::All);

                        this.on_supported_type_str_selected.raise();

                    }
                    else
                    {

                        let from_str_res = SupportedType::from_str(&item_string);

                        match from_str_res
                        {

                            Ok(res) =>
                            {

                                this.all_or_not_supported_type.set(AllOrNot::NotAll(res));

                                this.on_supported_type_str_selected.raise();

                            }
                            Err(err) =>
                            {

                                panic!("{}", err)

                            }

                        }
                        
                    }

                }

            }

            //});

        }));

        this

    }

    pub fn widget_ref(&self) -> &Box
    {

        &self.command_box

    }

    pub fn all_or_not_supported_type(&self) -> AllOrNot<SupportedType>
    {

        self.all_or_not_supported_type.get()

    }

    impl_pub_single_sub_event_method!(on_supported_type_str_selected, WindowContentsState);

}



