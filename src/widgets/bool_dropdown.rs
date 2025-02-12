use gtk_estate::gtk::DropDown;

static BOOL_STRS: &[&'static str] = &["false", "true"];

pub fn new_bool_strs_dropdown() -> DropDown
{

    DropDown::from_strings(BOOL_STRS)

}