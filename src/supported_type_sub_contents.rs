use std::{cell::Cell, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{gtk4::prelude::{BoxExt, Cast, WidgetExt}};

use crate::{widgets::{new_supported_type_strs_dropdown, new_supported_type_strs_no_all_dropdown}, SupportedType};

use corlib::{events::PubSingleSubEvent, impl_pub_single_sub_event_method, upgrading::try_up_rc};

use corlib::events::SingleSubEvent; 

use gtk_estate::gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, Widget};

use gtk_estate::gtk4::glib::clone;

pub struct SupportedTypeSubContents<P>
    where P: 'static
{

    supported_type_strs_dropdown: DropDown,
    supported_type_box: Box,
    supported_type: Cell<SupportedType>,
    on_supported_type_str_selected: SingleSubEvent<Self, P>

}

impl<P> SupportedTypeSubContents<P>
    where P: 'static
{

    pub fn new() -> Rc<Self>
    {

        let supported_type_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        //

        let label = Label::builder().label("SupportedType").halign(Align::Start).build();

        supported_type_box.append(&label);

        //

        let supported_type_strs_dropdown = new_supported_type_strs_no_all_dropdown();

        supported_type_strs_dropdown.set_width_request(120);

        //

        let supported_type_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(true).build();

        supported_type_strs_dropdown_box.append(&supported_type_strs_dropdown);

        supported_type_box.append(&supported_type_strs_dropdown_box);

        //

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                supported_type_strs_dropdown,
                supported_type_box,
                supported_type: Cell::new(SupportedType::Bool),
                on_supported_type_str_selected: SingleSubEvent::new(weak_self)

            }
        
        });

        this.supported_type_strs_dropdown.connect_selected_item_notify(clone!( #[strong] this, move |supported_type_strs_dropdown|
        {

            if let Some(item) = supported_type_strs_dropdown.selected_item()
            {

                if let Some(item) = item.downcast_ref::<StringObject>()
                {

                    let item_string = item.string();

                    let from_str_res = SupportedType::from_str(&item_string);

                    match from_str_res
                    {

                        Ok(res) =>
                        {

                            this.supported_type.set(res);

                            this.on_supported_type_str_selected.raise();

                        }
                        Err(err) =>
                        {

                            panic!("{}", err)

                        }

                    }

                }

            }

        }));

        this

    }

    pub fn widget_ref(&self) -> &Box
    {

        &self.supported_type_box

    }

    pub fn supported_type(&self) -> SupportedType
    {

        self.supported_type.get()

    }

    impl_pub_single_sub_event_method!(on_supported_type_str_selected, P);

}



