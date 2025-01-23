use std::{cell::Cell, fmt::Display, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::{clone::Downgrade, property::PropertyGet}, prelude::{BoxExt, Cast, EditableExt, IsA, TextBufferExt, TextViewExt, WidgetExt}}, gtk4::{Align, Box, DropDown, Label, Orientation, ScrolledWindow, StringObject, Text, TextView, Widget}, impl_contents_box_ref, WidgetContainer};

use crate::{widgets::new_whatever_strs_dropdown, AllOrNot, Whatever, WindowContentsState};

use corlib::{cell::RefCellStore, events::{PubSingleSubEvent, SingleSubArgsEvent}, impl_pub_single_sub_args_event_method, impl_pub_single_sub_event_method, inc_dec::IncDecSelf, upgrading::try_up_rc, value::HasValueGetter};

use corlib::events::{SingleSubEvent, PubSingleSubArgsEvent}; 

use gtk_estate::helpers::text_view::get_text_view_string;

use gtk_estate::gtk4::glib::clone;

use crate::{try_set_specific_whatever, parse_error_at_index, parse_array};

pub struct AllOrNotWhateverSubContents<P>
    where P: 'static
{

    whatever_strs_dropdown: DropDown,
    contents_box: Box,
    all_or_not_whatever_result: RefCellStore<Result<AllOrNot<Whatever>, String>>, //RefCellStore<AllOrNot<Whatever>>,
    on_whatever_str_selected: SingleSubEvent<Self, WindowContentsState>, //SingleSubArgsEvent<Self, AllOrNot<Whatever>, WindowContentsState>,
    value_input: TextView,
    on_value_input_parse_error: SingleSubArgsEvent<Self, String, WindowContentsState>,
    detected_whatever_variant: TextView

}

impl<P> AllOrNotWhateverSubContents<P>
    where P: 'static
{

    pub fn new() -> Rc<Self>
    {

        let contents_box = Box::builder().orientation(Orientation::Vertical).spacing(6).visible(true).build();

        //

        let label = Label::builder().label("Whatever").halign(Align::Start).build();

        contents_box.append(&label);

        //

        let whatever_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(true).build();

        let whatever_strs_dropdown = new_whatever_strs_dropdown();

        whatever_strs_dropdown.set_width_request(120);

        whatever_strs_dropdown_box.append(&whatever_strs_dropdown);

        contents_box.append(&whatever_strs_dropdown_box);

        //

        let value_input_label = Label::builder().label("Value Input").halign(Align::Start).build();

        contents_box.append(&value_input_label);

        //

        let value_input = TextView::builder().accepts_tab(false).build();

        let value_input_sw = ScrolledWindow::builder().child(&value_input).build();

        contents_box.append(&value_input_sw);

        //

        let detected_whatever_variant_label = Label::builder().label("Detected Variant Or Variants").halign(Align::Start).build();

        contents_box.append(&detected_whatever_variant_label);

        //

        //What is in all_or_not_whatever?

        let detected_whatever_variant = TextView::builder().editable(false).build(); //.text("All Variants")

        let detected_whatever_variant_sw = ScrolledWindow::builder().child(&detected_whatever_variant).build();

        contents_box.append(&detected_whatever_variant_sw);

        detected_whatever_variant.buffer().set_text("All Variants");

        //
    
        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                whatever_strs_dropdown,
                contents_box,
                all_or_not_whatever_result: RefCellStore::new(Ok(AllOrNot::All)),
                on_whatever_str_selected: SingleSubEvent::new(weak_self), //SingleSubArgsEvent::new(weak_self),
                value_input,
                on_value_input_parse_error: SingleSubArgsEvent::new(weak_self),
                detected_whatever_variant

            }
        
        });

        //let weak = this.downgrade();

        //clone!( #[strong] this,

        //let this2 = this.clone();

        //Try set the whatever variant, or all, when is a String is selected.

        this.whatever_strs_dropdown.connect_selected_item_notify(clone!( #[strong] this, move |whatever_strs_dropdown|
        {

            //try_up_rc(&weak, |this|
            //{

                if let Some(item) = whatever_strs_dropdown.selected_item()
                {

                    if let Some(item) = item.downcast_ref::<StringObject>()
                    {

                        let item_string = item.string();

                        this.try_set_whatever(&item_string);
                        
                    }

                }

            //});

        }));

        this.value_input.connect_move_focus(clone!( #[strong] this, move |_value_input, _|
        {

            if let Some(item) = this.whatever_strs_dropdown.selected_item()
            {

                if let Some(item) = item.downcast_ref::<StringObject>()
                {

                    let item_string = item.string();

                    this.try_set_whatever(&item_string);
                    
                }

            }

        }));
        
        this

    }

    impl_contents_box_ref!();

    /*
    fn parse_error_at_index(&self, index: usize, inner_message: String) //-> Result<Whatever, String>
    {

        let error_message = format!("Parsing Error: {{ index: {}, message: {} }}", index, inner_message);

        self.on_value_input_parse_error.raise(&error_message);

    }
    */

    //Implement impl_pub_single_sub_event_methods

    //impl_pub_single_sub_args_event_method!(on_whatever_str_selected, AllOrNot<Whatever>, WindowContentsState);

    impl_pub_single_sub_event_method!(on_whatever_str_selected, WindowContentsState);

    //impl_pub_single_sub_event_method!(on_value_input_parse_error, WindowContentsState);

    impl_pub_single_sub_args_event_method!(on_value_input_parse_error, String, WindowContentsState);

    /*
    pub fn widget_ref(&self) -> &Box
    {

        &self.whatever_box

    }

    pub fn all_or_not_whatever_result(&self) -> Result<AllOrNot<Whatever>, String>
    {

        self.all_or_not_whatever_result.get()

    }
    */

    /*
    fn raise_on_whatever_str_selected(&self)
    {

        self.all_or_not_whatever.borrow(|state|
        {

            if let Ok(all_or_not) = &*state
            {

                self.on_whatever_str_selected.raise(all_or_not);

            }

        });

    }
    */

    fn try_set_whatever(&self, variant_str: &str)
    {

        if variant_str == "*"
        {

            self.all_or_not_whatever_result.set(Ok(AllOrNot::All));

            //this.all_or_not_whatever.borrow_mut(|state| { *state = AllOrNot::All; } ); //.set(AllOrNot::All);

            //self.raise_on_whatever_str_selected();

            self.on_whatever_str_selected.raise();

            self.detected_whatever_variant.buffer().set_text("All Variants");

            //Clear the value input buffer.

            self.value_input.buffer().set_text("");

        }
        else
        {

            let buffer = self.value_input.buffer();

            let start = buffer.start_iter();

            let end = buffer.end_iter();

            let buffer_text = buffer.text(&start, &end, false);

            let value_input_str = buffer_text.as_str();

            let the_res = try_set_specific_whatever(variant_str, value_input_str);

            match the_res
            {

                Ok((whatever, variant_string)) =>
                {

                    self.all_or_not_whatever_result.set(Ok(AllOrNot::NotAll(whatever)));

                    self.detected_whatever_variant.buffer().set_text(&variant_string);

                    self.on_whatever_str_selected.raise();

                }
                Err(error_message) =>
                {

                    self.detected_whatever_variant.buffer().set_text(&error_message);

                    self.all_or_not_whatever_result.set(Err(error_message));

                    self.all_or_not_whatever_result.borrow(|store|
                    {

                        if let Err(message) = &*store
                        {

                            self.on_value_input_parse_error.raise(message);

                        }

                    })

                }

            }

            //let from_str_res = Whatever::from_str(variant_str);

            /*
            match from_str_res
            {

                Ok(res) =>
                {

                    let buffer = self.value_input.buffer();

                    let start = buffer.start_iter();

                    let end = buffer.end_iter();

                    let buffer_text = buffer.text(&start, &end, false);

                    let value_input_str = buffer_text.as_str();

                    //let value_input_string = get_text_view_string(&this.value_input);

                    //this.all_or_not_whatever.set(AllOrNot::NotAll(res));

                    //let whatever_res;

                    /*
                    let the_res;

                    match res
                    {

                        Whatever::Bool(_) =>
                        {

                            let res = bool::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::Bool({})", val));

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

                            let res = char::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::Char(\'{}\')", val));

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

                            let res = f32::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::F32({})", val));

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

                            let res = f64::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::F64({})", val));

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

                            let res = i8::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::I8({})", val));

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

                            let res = i16::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::I16({})", val));

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

                            let res = i32::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::I32({})", val));

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

                            let res = i64::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::I64({})", val));

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

                            let res = i128::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::I128({})", val));

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

                            let res = u8::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::U8({})", val));

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

                            let res = u16::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::U16({})", val));

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

                            let res = u32::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::U32({})", val));

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

                            let res = u64::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::U64({})", val));

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

                            let res = u128::from_str(value_input_str);

                            match res
                            {

                                Ok(val) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::U128({})", val));

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

                            self.detected_whatever_variant.buffer().set_text(&format!("Whatever::String(\"{}\")", value_input_str));

                            the_res = Ok(Whatever::String(value_input_str.to_string()));

                        }
                        Whatever::VecBool(mut vec) =>
                        {

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecBool({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecF32({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecF64({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecI8({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecI16({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecI32({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecI64({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecI128({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecU8({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecU16({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecU32({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecU64({:?})", vec));

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

                            let res = parse_array(value_input_str, &mut vec);

                            match res
                            {

                                Ok(_) =>
                                {

                                    self.detected_whatever_variant.buffer().set_text(&format!("Whatever::VecU128({:?})", vec));

                                    the_res = Ok(Whatever::VecU128(vec));

                                }
                                Err(err) =>
                                {

                                    the_res = Err(err);

                                }
                                
                            }

                        }

                    }
                    */

                    //let the_res = try_set_specific_whatever()

                    match the_res
                    {

                        Ok(res) =>
                        {

                            self.all_or_not_whatever.set(Ok(AllOrNot::NotAll(res)));

                            //self.raise_on_whatever_str_selected();

                            self.on_whatever_str_selected.raise();

                        }
                        Err(error_message) =>
                        {

                            self.detected_whatever_variant.buffer().set_text(&error_message);

                            self.all_or_not_whatever.set(Err(error_message));

                            self.all_or_not_whatever.borrow(|store|
                            {

                                if let Err(message) = &*store
                                {

                                    self.on_value_input_parse_error.raise(message);

                                }

                                //self.on_value_input_parse_error.raise(); //&error_message);

                            })

                        }

                    }            

                }
                Err(err) =>
                {

                    //parent.output_error(err);

                    panic!("{}", err)

                }

            }
            */

        }

    }

}

impl<P> WidgetContainer for AllOrNotWhateverSubContents<P>
{

    fn widget(&self) -> Widget
    {

        self.contents_box.upcast_ref::<Widget>().clone()
        
    }

    fn widget_ref(&self) -> &Widget
    {

        self.contents_box.upcast_ref::<Widget>()
        
    }

}

impl<P> HasValueGetter for AllOrNotWhateverSubContents<P>
{

    type HasValueType = Result<AllOrNot<Whatever>, String>;

    fn value(&self) -> Self::HasValueType
    {

        self.all_or_not_whatever_result.get()

    }

}

/*
fn parse_error_at_index<T>(index: usize, inner_message: String) -> Result<T, String>
{

    Err(format!("Parsing Error: {{ index: {}, message: {} }}", index, inner_message))

}

fn parse_array<T>(value_input_str: &str, vec: &mut Vec<T>) -> Result<(), String>
    where T: FromStr,
          T::Err: Display + ToString
{


    let mut index: usize = 0;

    let split_value_input = value_input_str.split(',');

    for item in split_value_input
    {

        let trimmed_item = item.trim();

        let res = T::from_str(trimmed_item);

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
*/

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
