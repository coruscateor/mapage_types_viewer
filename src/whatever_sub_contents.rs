use std::{cell::Cell, fmt::Display, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::{clone::Downgrade, property::PropertyGet}, prelude::{BoxExt, Cast, IsA, TextViewExt, WidgetExt}}, gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, TextView, Widget}};

use crate::{widgets::{new_whatever_strs_dropdown}, AllOrNot, Whatever, WindowContentsState};

use corlib::{events::{PubSingleSubEvent, SingleSubArgsEvent}, impl_pub_single_sub_args_event_method, impl_pub_single_sub_event_method, inc_dec::IncDecSelf, upgrading::try_up_rc};

use corlib::events::{SingleSubEvent, PubSingleSubArgsEvent}; 

use gtk_estate::helpers::text_view::get_text_view_string;

pub struct WhateverSubContents
{

    whatever_strs_dropdown: DropDown,
    whatever_box: Box,
    all_or_not_whatever: Cell<AllOrNot<Whatever>>,
    on_whatever_str_selected: SingleSubEvent<Self, WindowContentsState>,
    value_input: TextView,
    on_value_input_parse_error: SingleSubArgsEvent<Self, String, WindowContentsState>

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
                value_input,
                on_value_input_parse_error: SingleSubArgsEvent::new(weak_self),

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

                                    let value_input_string = get_text_view_string(&this.value_input);

                                    //this.all_or_not_whatever.set(AllOrNot::NotAll(res));

                                    //let whatever_res;

                                    let the_res;

                                    match res
                                    {
                                        Whatever::Bool(_) =>
                                        {

                                            let res = bool::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::Bool(val));

                                                    //whatever_res = Whatever::Bool(val);

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                    //this.on_value_input_parse_error.raise(&err.to_string());

                                                    //return;

                                                }

                                            }

                                        }
                                        Whatever::Char(_) =>
                                        {

                                            let res = char::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::Char(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::F32(_) =>
                                        {

                                            let res = f32::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::F32(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::F64(_) =>
                                        {

                                            let res = f64::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::F64(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::I8(_) =>
                                        {

                                            let res = i8::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::I8(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::I16(_) =>
                                        {

                                            let res = i16::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::I16(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::I32(_) =>
                                        {

                                            let res = i32::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::I32(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::I64(_) =>
                                        {

                                            let res = i64::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::I64(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::I128(_) =>
                                        {

                                            let res = i128::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::I128(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::U8(_) =>
                                        {

                                            let res = u8::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::U8(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::U16(_) =>
                                        {

                                            let res = u16::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::U16(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::U32(_) =>
                                        {

                                            let res = u32::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::U32(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::U64(_) =>
                                        {

                                            let res = u64::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::U64(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::U128(_) =>
                                        {

                                            let res = u128::from_str(&value_input_string);

                                            match res
                                            {

                                                Ok(val) =>
                                                {

                                                    the_res = Ok(Whatever::U128(val));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err.to_string());

                                                }

                                            }

                                        }
                                        Whatever::String(_) =>
                                        {

                                            the_res = Ok(Whatever::String(value_input_string));

                                        }
                                        Whatever::VecBool(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecBool(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }

                                            }
                                            
                                            /*
                                            let mut index: usize = 0;

                                            let split_value_input = value_input_string.split(',');

                                            for item in split_value_input
                                            {

                                                let res = bool::from_str(item);

                                                match res
                                                {

                                                    Ok(val) =>
                                                    {

                                                        vec.push(val);

                                                    }
                                                    Err(err) =>
                                                    {

                                                        this.parse_error_at_index(index, err.to_string());

                                                        return;

                                                        //the_res = parse_error_at_index(index, err.to_string());

                                                        //break;

                                                    }

                                                }

                                                index.pp();

                                            }

                                            the_res = Ok(Whatever::VecBool(vec));
                                            */

                                        }
                                        Whatever::VecF32(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecF32(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                            /*
                                            let mut index: usize = 0;

                                            let split_value_input = value_input_string.split(',');

                                            for item in split_value_input
                                            {

                                                let res = f32::from_str(item);

                                                match res
                                                {

                                                    Ok(val) =>
                                                    {

                                                        vec.push(val);

                                                    }
                                                    Err(err) =>
                                                    {

                                                        this.parse_error_at_index(index, err.to_string());

                                                        return;

                                                        //the_res = parse_error_at_index(index, err.to_string());

                                                        //break;

                                                    }

                                                }

                                                index.pp();

                                            }

                                            the_res = Ok(Whatever::VecBool(vec));
                                            */

                                        }
                                        Whatever::VecF64(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecF64(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecI8(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecI8(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecI16(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecI16(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecI32(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecI32(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecI64(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecI64(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecI128(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecI128(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecU8(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecU8(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecU16(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecU16(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecU32(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecU32(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecU64(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecU64(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }
                                        Whatever::VecU128(mut vec) =>
                                        {

                                            let res = parse_array(value_input_string, &mut vec);

                                            match res
                                            {

                                                Ok(_) =>
                                                {

                                                    the_res = Ok(Whatever::VecU128(vec));

                                                }
                                                Err(err) =>
                                                {

                                                    the_res = Err(err);

                                                }
                                                
                                            }

                                        }

                                    }

                                    match the_res
                                    {

                                        Ok(res) =>
                                        {

                                            this.all_or_not_whatever.set(AllOrNot::NotAll(res));

                                            this.on_whatever_str_selected.raise();

                                        }
                                        Err(error_message) =>
                                        {

                                            //Pass parameter by move.

                                            this.on_value_input_parse_error.raise(&error_message);

                                        }

                                    }

                                    
    
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

    /*
    fn parse_error_at_index(&self, index: usize, inner_message: String) //-> Result<Whatever, String>
    {

        let error_message = format!("Parsing Error: {{ index: {}, message: {} }}", index, inner_message);

        self.on_value_input_parse_error.raise(&error_message);

    }
    */

    //Implement impl_pub_single_sub_event_methods

    impl_pub_single_sub_event_method!(on_whatever_str_selected, WindowContentsState);

    impl_pub_single_sub_args_event_method!(on_value_input_parse_error, String, WindowContentsState);

    pub fn widget_ref(&self) -> &Box
    {

        &self.whatever_box

    }

}

fn parse_error_at_index<T>(index: usize, inner_message: String) -> Result<T, String>
{

    Err(format!("Parsing Error: {{ index: {}, message: {} }}", index, inner_message))

}

fn parse_array<T>(value_input_string: String, vec: &mut Vec<T>) -> Result<(), String>
    where T: FromStr,
          T::Err: Display + ToString
{


    let mut index: usize = 0;

    let split_value_input = value_input_string.split(',');

    for item in split_value_input
    {

        let res = T::from_str(item);

        match res
        {

            Ok(val) =>
            {

                vec.push(val);

            }
            Err(err) =>
            {

                return parse_error_at_index(index, err.to_string());

            }

        }

        index.pp();

    }

    Ok(())

}

//Add to GTK Estate

/*
impl Deref for WhateverSubContents
{

    type Target = Box;

    fn deref(&self) -> &Self::Target
    {

        &self.whatever_box

    }

}
*/

/*
impl Deref for WhateverSubContents
{

    type Target = Widget;

    fn deref(&self) -> &Self::Target
    {

        self.whatever_box.upcast_ref::<Widget>()

    }

}
*/

/*
impl Deref for WhateverSubContents
{

    type Target = dyn IsA<Widget>;

    fn deref(&self) -> &Self::Target
    {

        self.whatever_box.upcast_ref::<IsA<Widget>>()

    }

}
*/
