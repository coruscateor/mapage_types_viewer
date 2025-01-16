use std::{cell::Cell, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::clone::Downgrade, prelude::{BoxExt, Cast, WidgetExt}}, gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, Widget}};

use crate::{widgets::new_supported_type_strs_dropdown, AllOrNot, SupportedType, WindowContentsState};

use corlib::{events::PubSingleSubEvent, impl_pub_single_sub_event_method, upgrading::try_up_rc};

use corlib::events::SingleSubEvent; 

pub struct SupportedTypeSubContents
{

    supported_type_strs_dropdown: DropDown,
    supported_type_box: Box,
    all_or_not_supported_type: Cell<AllOrNot<SupportedType>>,
    on_supported_type_str_selected: SingleSubEvent<Self, WindowContentsState>

}

impl SupportedTypeSubContents
{

    pub fn new() -> Rc<Self>
    {

        let supported_type_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        //

        let label = Label::builder().label("SupportedType").halign(Align::Start).build();

        supported_type_box.append(&label);

        //

        let supported_type_strs_dropdown = new_supported_type_strs_dropdown();

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
                all_or_not_supported_type: Cell::new(AllOrNot::All),
                on_supported_type_str_selected: SingleSubEvent::new(weak_self)

            }
        
        });

        let weak = this.downgrade();

        this.supported_type_strs_dropdown.connect_selected_item_notify(move |supported_type_strs_dropdown|
        {

            try_up_rc(&weak, |this|
            {

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

            });

        });

        this

    }

    pub fn widget_ref(&self) -> &Box
    {

        &self.supported_type_box

    }

    pub fn all_or_not_supported_type(&self) -> AllOrNot<SupportedType>
    {

        self.all_or_not_supported_type.get()

    }

    impl_pub_single_sub_event_method!(on_supported_type_str_selected, WindowContentsState);

}

/*
impl Deref for SupportedTypeSubContents
{

    type Target = Widget;

    fn deref(&self) -> &Self::Target
    {

        self.supported_type_box.upcast_ref::<Widget>()

    }

}
*/



