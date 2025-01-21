use std::{cell::Cell, fmt::Display, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::{clone::Downgrade, property::PropertyGet}, prelude::{BoxExt, Cast, EditableExt, IsA, TextBufferExt, TextViewExt, WidgetExt}}, gtk4::{Align, Box, DropDown, Label, Orientation, ScrolledWindow, StringObject, Text, TextView, Widget}};

use crate::{widgets::{new_type_instance_strs_dropdown, new_type_instance_strs_no_all_dropdown, new_whatever_strs_no_all_dropdown}, AllOrNot, TypeInstance, Whatever, WindowContentsState};

use corlib::{cell::RefCellStore, events::{PubSingleSubEvent, SingleSubArgsEvent}, impl_pub_single_sub_args_event_method, impl_pub_single_sub_event_method, inc_dec::IncDecSelf, upgrading::try_up_rc};

use corlib::events::{SingleSubEvent, PubSingleSubArgsEvent}; 

use gtk_estate::helpers::text_view::get_text_view_string;

use gtk_estate::gtk4::glib::clone;

use crate::{try_set_specific_whatever, parse_error_at_index, parse_array};

pub struct TypeInstanceSubContents<P>
    where P: 'static
{

    type_instance_strs_dropdown: DropDown,
    whatever_strs_dropdown_box: Box,
    whatever_strs_dropdown: DropDown,
    type_instance_box: Box,
    type_instance_result: RefCellStore<Result<TypeInstance, String>>,
    on_type_instance_str_selected: SingleSubEvent<Self, P>,
    value_input: TextView,
    on_value_input_parse_error: SingleSubArgsEvent<Self, String, P>,
    detected_type_instance_variant: TextView

}

impl<P> TypeInstanceSubContents<P>
    where P: 'static
{

    pub fn new() -> Rc<Self>
    {

        let type_instance_box = Box::builder().orientation(Orientation::Vertical).spacing(6).visible(true).build();

        //

        let label = Label::builder().label("TypeInstance").halign(Align::Start).build();

        type_instance_box.append(&label);

        //

        let type_instance_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(true).build();

        let type_instance_strs_dropdown = new_type_instance_strs_no_all_dropdown();

        type_instance_strs_dropdown.set_width_request(120);

        type_instance_strs_dropdown_box.append(&type_instance_strs_dropdown);

        type_instance_box.append(&type_instance_strs_dropdown_box);

        //Whatever

        let whatever_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(false).build();

        let whatever_strs_dropdown = new_whatever_strs_no_all_dropdown();

        whatever_strs_dropdown.set_width_request(120);

        whatever_strs_dropdown_box.append(&whatever_strs_dropdown);

        type_instance_box.append(&whatever_strs_dropdown_box);

        //

        let value_input_label = Label::builder().label("Value Input").halign(Align::Start).build();

        type_instance_box.append(&value_input_label);

        //

        let value_input = TextView::builder().accepts_tab(false).build();

        let value_input_sw = ScrolledWindow::builder().child(&value_input).build();

        type_instance_box.append(&value_input_sw);

        //

        let detected_whatever_variant_label = Label::builder().label("Detected Variant Or Variants").halign(Align::Start).build();

        type_instance_box.append(&detected_whatever_variant_label);

        //

        //What is in all_or_not_type_instance?

        let detected_type_instance_variant = TextView::builder().editable(false).build();

        let detected_type_instance_variant_sw = ScrolledWindow::builder().child(&detected_type_instance_variant).build();

        type_instance_box.append(&detected_type_instance_variant_sw);

        detected_type_instance_variant.buffer().set_text("All Variants");

        //
    
        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {
                
                type_instance_strs_dropdown,
                whatever_strs_dropdown_box,
                whatever_strs_dropdown,
                type_instance_box,
                type_instance_result: RefCellStore::new(Ok(TypeInstance::default())),
                on_type_instance_str_selected: SingleSubEvent::new(weak_self),
                value_input,
                on_value_input_parse_error: SingleSubArgsEvent::new(weak_self),
                detected_type_instance_variant

            }
        
        });

        //Try set the TypeInstance variant, or all, when is a String is selected.

        this.type_instance_strs_dropdown.connect_selected_item_notify(clone!( #[strong] this, move |type_instance_strs_dropdown|
        {

            if let Some(item) = type_instance_strs_dropdown.selected_item()
            {

                if let Some(item) = item.downcast_ref::<StringObject>()
                {

                    let item_string = item.string();

                    this.try_set_type_instance(&item_string);
                    
                }

            }

        }));

        this.value_input.connect_move_focus(clone!( #[strong] this, move |_value_input, _|
        {

            if let Some(item) = this.type_instance_strs_dropdown.selected_item()
            {

                if let Some(item) = item.downcast_ref::<StringObject>()
                {

                    let item_string = item.string();

                    this.try_set_type_instance(&item_string);
                    
                }

            }

        }));

        /*
        this.whatever_strs_dropdown.connect_selected_item_notify(clone!( #[strong] this, move |type_instance_strs_dropdown|
        {



        }));
        */
        
        this

    }

    impl_pub_single_sub_event_method!(on_type_instance_str_selected, P);

    impl_pub_single_sub_args_event_method!(on_value_input_parse_error, String, P);

    pub fn widget_ref(&self) -> &Box
    {

        &self.type_instance_box

    }

    pub fn type_instance_result(&self) -> Result<TypeInstance, String>
    {

        self.type_instance_result.get()

    }

    fn set_whatever_strs_dropdown_box_invisible(&self)
    {

        self.whatever_strs_dropdown_box.set_visible(false);

    }

    fn set_whatever_strs_dropdown_box_visible(&self)
    {

        self.whatever_strs_dropdown_box.set_visible(true);

    }

    fn try_set_type_instance(&self, variant_str: &str)
    {


        let from_str_res = TypeInstance::from_str(variant_str);

        match from_str_res
        {

            Ok(res) =>
            {

                let buffer = self.value_input.buffer();

                let start = buffer.start_iter();

                let end = buffer.end_iter();

                let buffer_text = buffer.text(&start, &end, false);

                let value_input_str = buffer_text.as_str();

                let the_res;

                let mut detected_type_instance_variant = String::new();

                match res
                {

                    TypeInstance::Bool(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = bool::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::Bool({})", val);

                                the_res = Ok(TypeInstance::Bool(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::Char(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = char::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::Char(\'{}\')", val);

                                the_res = Ok(TypeInstance::Char(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::F32(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = f32::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::F32({})", val);

                                the_res = Ok(TypeInstance::F32(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::F64(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = f64::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::F64({})", val);

                                the_res = Ok(TypeInstance::F64(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::I8(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = i8::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::I8({})", val);

                                the_res = Ok(TypeInstance::I8(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::I16(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = i16::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::I16({})", val);

                                the_res = Ok(TypeInstance::I16(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::I32(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = i32::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::I32({})", val);

                                the_res = Ok(TypeInstance::I32(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::I64(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = i64::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::I64({})", val);

                                the_res = Ok(TypeInstance::I64(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::I128(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = i128::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::I128({})", val);

                                the_res = Ok(TypeInstance::I128(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::U8(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = u8::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::U8({})", val);

                                the_res = Ok(TypeInstance::U8(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::U16(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = u16::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::U16({})", val);

                                the_res = Ok(TypeInstance::U16(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::U32(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = u32::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::U32({})", val);

                                the_res = Ok(TypeInstance::U32(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::U64(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = u64::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::U64({})", val);

                                the_res = Ok(TypeInstance::U64(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::U128(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = u128::from_str(value_input_str);

                        match res
                        {

                            Ok(val) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::U128({})", val);

                                the_res = Ok(TypeInstance::U128(val));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::String(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        detected_type_instance_variant = format!("TypeInstance::String(\"{}\")", value_input_str);

                        the_res = Ok(TypeInstance::String(value_input_str.to_string()));

                    }
                    TypeInstance::Whatever(_) =>
                    {

                        self.set_whatever_strs_dropdown_box_visible();

                        //let res: Result<Whatever, String> = Ok(Whatever::default());

                        //let whatever_string;

                        let whatever_object = self.whatever_strs_dropdown.selected_item().expect("Error: Unexpected Item");

                        let whatever_string_object = whatever_object.downcast_ref::<StringObject>().expect("Error: Cannot cast to StringObject");

                        let whatever_string = whatever_string_object.string();

                        let res = try_set_specific_whatever( &whatever_string, value_input_str);

                        match res
                        {

                            Ok((whatever, detected_variant)) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::Whatever({})", detected_variant);

                                the_res = Ok(TypeInstance::Whatever(whatever));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err.to_string());

                            }

                        }

                    }
                    TypeInstance::VecBool(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecBool({:?})", vec);

                                the_res = Ok(TypeInstance::VecBool(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }

                        }

                    }
                    TypeInstance::VecF32(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecF32({:?})", vec);

                                the_res = Ok(TypeInstance::VecF32(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecF64(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecF64({:?})", vec);

                                the_res = Ok(TypeInstance::VecF64(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecI8(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecI8({:?})", vec);

                                the_res = Ok(TypeInstance::VecI8(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecI16(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecI16({:?})", vec);

                                the_res = Ok(TypeInstance::VecI16(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecI32(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecI32({:?})", vec);

                                the_res = Ok(TypeInstance::VecI32(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecI64(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecI64({:?})", vec);

                                the_res = Ok(TypeInstance::VecI64(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecI128(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecI128({:?})", vec);

                                the_res = Ok(TypeInstance::VecI128(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecU8(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecU8({:?})", vec);

                                the_res = Ok(TypeInstance::VecU8(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecU16(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecU16({:?})", vec);

                                the_res = Ok(TypeInstance::VecU16(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecU32(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecU32({:?})", vec);

                                the_res = Ok(TypeInstance::VecU32(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecU64(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecU64({:?})", vec);

                                the_res = Ok(TypeInstance::VecU64(vec));

                            }
                            Err(err) =>
                            {

                                the_res = Err(err);

                            }
                            
                        }

                    }
                    TypeInstance::VecU128(mut vec) =>
                    {

                        self.set_whatever_strs_dropdown_box_invisible();

                        let res = parse_array(value_input_str, &mut vec);

                        match res
                        {

                            Ok(_) =>
                            {

                                detected_type_instance_variant = format!("TypeInstance::VecU128({:?})", vec);

                                the_res = Ok(TypeInstance::VecU128(vec));

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

                        self.detected_type_instance_variant.buffer().set_text(&detected_type_instance_variant);

                        self.type_instance_result.set(Ok(res));

                        self.on_type_instance_str_selected.raise();

                    }
                    Err(error_message) =>
                    {

                        self.detected_type_instance_variant.buffer().set_text(&error_message);

                        self.type_instance_result.set(Err(error_message));

                        self.type_instance_result.borrow(|store|
                        {

                            if let Err(message) = &*store
                            {

                                self.on_value_input_parse_error.raise(message);

                            }

                        })

                    }        

                }
      
            }
            Err(err) =>
            {

                panic!("{}", err)

            }
            
        }

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
