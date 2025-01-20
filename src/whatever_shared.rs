use std::{fmt::Display, str::FromStr};

use corlib::inc_dec::IncDecSelf;

use crate::Whatever;

pub fn try_set_specific_whatever(variant_str: &str, value_input_str: &str) -> Result<(Whatever, String), String>
{

    let from_str_res = Whatever::from_str(variant_str);
    
    let detected_whatever_variant;

    let whatever;

    match from_str_res
    {

        Ok(res) =>
        {

            match res
            {
        
                Whatever::Bool(_) =>
                {
        
                    let res = bool::from_str(value_input_str);
        
                    match res
                    {
        
                        Ok(val) =>
                        {
        
                            detected_whatever_variant = format!("Whatever::Bool({})", val);
        
                            whatever = Whatever::Bool(val);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::Char(\'{}\')", val);
        
                            whatever = Whatever::Char(val);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::F32({})", val);
        
                            whatever = Whatever::F32(val);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::F64({})", val);
        
                            whatever = Whatever::F64(val);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::I8({})", val);
        
                            whatever = Whatever::I8(val);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::I16({})", val);
        
                            whatever = Whatever::I16(val);
        
                        }
                        Err(err) =>
                        {

                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::I32({})", val);
        
                            whatever = Whatever::I32(val);
        
                        }
                        Err(err) =>
                        {

                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::I64({})", val);
        
                            whatever = Whatever::I64(val);
        
                        }
                        Err(err) =>
                        {
        
                            return  Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::I128({})", val);
        
                            whatever = Whatever::I128(val);
        
                        }
                        Err(err) =>
                        {
                            
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::U8({})", val);
        
                            whatever = Whatever::U8(val);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::U16({})", val);
        
                            whatever = Whatever::U16(val);
        
                        }
                        Err(err) =>
                        {
                            
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::U32({})", val);
        
                            whatever = Whatever::U32(val);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::U64({})", val);
        
                            whatever =Whatever::U64(val);
        
                        }
                        Err(err) =>
                        {
                            
                            return Err(err.to_string());
        
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
        
                            detected_whatever_variant = format!("Whatever::U128({})", val);
        
                            whatever = Whatever::U128(val);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err.to_string());
        
                        }
        
                    }
        
                }
                Whatever::String(_) =>
                {
        
                    detected_whatever_variant = format!("Whatever::String(\"{}\")", value_input_str);
        
                    whatever = Whatever::String(value_input_str.to_string());
        
                }
                Whatever::VecBool(mut vec) =>
                {
        
                    let res = parse_array(value_input_str, &mut vec);
        
                    match res
                    {
        
                        Ok(_) =>
                        {
        
                            detected_whatever_variant = format!("Whatever::VecBool({:?})", vec);
        
                            whatever = Whatever::VecBool(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
                        }
        
                    }
        
                }
                Whatever::VecF32(mut vec) =>
                {
        
                    let res = parse_array(value_input_str, &mut vec);
        
                    match res
                    {
        
                        Ok(_) =>
                        {
        
                            detected_whatever_variant = format!("Whatever::VecF32({:?})", vec);
        
                            whatever = Whatever::VecF32(vec);
        
                        }
                        Err(err) =>
                        {

                            return Err(err);
        
                        }
                        
                    }
        
                }
                Whatever::VecF64(mut vec) =>
                {
        
                    let res = parse_array(value_input_str, &mut vec);
        
                    match res
                    {
        
                        Ok(_) =>
                        {
        
                            detected_whatever_variant = format!("Whatever::VecF64({:?})", vec);
        
                            whatever = Whatever::VecF64(vec);
        
                        }
                        Err(err) =>
                        {
                            
                            return Err(err);
        
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

                            detected_whatever_variant = format!("Whatever::VecI8({:?})", vec);
        
                            whatever = Whatever::VecI8(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecI16({:?})", vec);
        
                            whatever = Whatever::VecI16(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecI32({:?})", vec);
        
                            whatever = Whatever::VecI32(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecI64({:?})", vec);
        
                            whatever = Whatever::VecI64(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecI128({:?})", vec);
        
                            whatever = Whatever::VecI128(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecU8({:?})", vec);
        
                            whatever = Whatever::VecU8(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecU16({:?})", vec);
        
                            whatever =Whatever::VecU16(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecU32({:?})", vec);
        
                            whatever = Whatever::VecU32(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecU64({:?})", vec);
        
                            whatever = Whatever::VecU64(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
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
        
                            detected_whatever_variant = format!("Whatever::VecU128({:?})", vec);
        
                            whatever = Whatever::VecU128(vec);
        
                        }
                        Err(err) =>
                        {
        
                            return Err(err);
        
                        }
                        
                    }
        
                }
        
            }

        }
        Err(err) =>
        {

            return Err(err.to_string());

        }

    }

    //whatever

    Ok((whatever, detected_whatever_variant))

}

pub fn parse_error_at_index<T>(index: usize, inner_message: String) -> Result<T, String>
{

    Err(format!("Parsing Error: {{ index: {}, message: {} }}", index, inner_message))

}

pub fn parse_array<T>(value_input_str: &str, vec: &mut Vec<T>) -> Result<(), String>
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
