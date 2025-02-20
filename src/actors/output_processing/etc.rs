
//The GTK TextView Buffer doesn't like it when you try to append null characters to it.

pub fn check_for_nulls(string: &String) -> Option<String>
{

    if string.contains('\0')
    {

        Some(string.replace('\0', "\\0"))

    }
    else
    {

        None
        
    }

}


