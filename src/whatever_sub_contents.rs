use std::{cell::Cell, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::clone::Downgrade, prelude::{BoxExt, Cast, WidgetExt}}, gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, TextView}};

use crate::{widgets::{new_whatever_strs_dropdown}, AllOrNot, Whatever, WindowContentsState};

use corlib::{events::PubSingleSubEvent, impl_pub_single_sub_event_method, upgrading::try_up_rc};

use corlib::events::SingleSubEvent; 

pub struct WhateverSubContents
{

    whatever_strs_dropdown: DropDown,
    whatever_box: Box,
    all_or_not_whatever: Cell<AllOrNot<Whatever>>,
    on_whatever_str_selected: SingleSubEvent<Self, WindowContentsState>,
    value_input: TextView

}

impl WhateverSubContents
{

    pub fn new() -> Rc<Self>
    {

        let label = Label::builder().label("Whatever").halign(Align::Start).build();

        let whatever_strs_dropdown = new_whatever_strs_dropdown();

        whatever_strs_dropdown.set_width_request(120);

        //

        let whatever_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(true).build();

        whatever_strs_dropdown_box.append(&whatever_strs_dropdown);

        //

        let value_input_label = Label::builder().label("Value Input").halign(Align::Start).build();

        let value_input = TextView::builder().build();

        //

        let whatever_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        whatever_box.append(&label);

        whatever_box.append(&whatever_strs_dropdown_box);

        whatever_box.append(&value_input_label);

        whatever_box.append(&value_input);

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                whatever_strs_dropdown,
                whatever_box,
                all_or_not_whatever: Cell::new(AllOrNot::All),
                on_whatever_str_selected: SingleSubEvent::new(weak_self),
                value_input

            }
        
        });

        let weak = this.downgrade();

        this.whatever_strs_dropdown.connect_selected_item_notify(move |whatever_strs_dropdown|
        {

            try_up_rc(&weak, |this|
            {

                if let Some(item) = whatever_strs_dropdown.selected_item()
                {

                    if let Some(item) = item.downcast_ref::<StringObject>()
                    {

                        let item_string = item.string();

                        if item_string == "*"
                        {

                            this.all_or_not_whatever.set(AllOrNot::All);



                            this.on_whatever_str_selected.raise();

                        }
                        else
                        {

                            let from_str_res = Whatever::from_str(&item_string);

                            match from_str_res
                            {
    
                                Ok(res) =>
                                {

                                    this.all_or_not_whatever.set(AllOrNot::NotAll(res));

                                    match res
                                    {
                                        Whatever::Bool(_) => todo!(),
                                        Whatever::Char(_) => todo!(),
                                        Whatever::F32(_) => todo!(),
                                        Whatever::F64(_) => todo!(),
                                        Whatever::I8(_) => todo!(),
                                        Whatever::I16(_) => todo!(),
                                        Whatever::I32(_) => todo!(),
                                        Whatever::I64(_) => todo!(),
                                        Whatever::I128(_) => todo!(),
                                        Whatever::U8(_) => todo!(),
                                        Whatever::U16(_) => todo!(),
                                        Whatever::U32(_) => todo!(),
                                        Whatever::U64(_) => todo!(),
                                        Whatever::U128(_) => todo!(),
                                        Whatever::String(_) => todo!(),
                                        Whatever::VecBool(vec) => todo!(),
                                        Whatever::VecF32(vec) => todo!(),
                                        Whatever::VecF64(vec) => todo!(),
                                        Whatever::VecI8(vec) => todo!(),
                                        Whatever::VecI16(vec) => todo!(),
                                        Whatever::VecI32(vec) => todo!(),
                                        Whatever::VecI64(vec) => todo!(),
                                        Whatever::VecI128(vec) => todo!(),
                                        Whatever::VecU8(vec) => todo!(),
                                        Whatever::VecU16(vec) => todo!(),
                                        Whatever::VecU32(vec) => todo!(),
                                        Whatever::VecU64(vec) => todo!(),
                                        Whatever::VecU128(vec) => todo!(),
                                    }


                                    this.on_whatever_str_selected.raise();
    
                                }
                                Err(err) =>
                                {
    
                                    //parent.output_error(err);
    
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

}