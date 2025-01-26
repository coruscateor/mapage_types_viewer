use std::str::FromStr;

pub fn try_get_id(str_value: &str, field: &str) -> Result<Option<u32>, String>
{

    let trimmed_str_value = str_value.trim();

    if trimmed_str_value.is_empty()
    {

        Ok(None)

    }
    else
    {

        let id_number_result = u32::from_str(trimmed_str_value);

        match id_number_result
        {

            Ok(res) =>
            {

                Ok(Some(res))

            }
            Err(err) =>
            {

                Err(format!("Error: {}: {}", field, err.to_string()))

            }

        }
    
    }

}

pub fn try_get_usize_index(str_value: &str, field: &str) -> Result<Option<usize>, String>
{

    let trimmed_str_value = str_value.trim();

    if trimmed_str_value.is_empty()
    {

        Ok(None)

    }
    else
    {

        let id_number_result = usize::from_str(trimmed_str_value);

        match id_number_result
        {

            Ok(res) =>
            {

                Ok(Some(res))

            }
            Err(err) =>
            {

                Err(format!("Error: {}: {}", field, err.to_string()))

            }

        }
    
    }

}