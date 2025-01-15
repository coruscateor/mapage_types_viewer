use gtk_estate::{gtk4::DropDown, RcWidgetAdapter, StoredWidgetObject, WidgetAdapter, WidgetStateContainer};

use strum::{EnumCount, IntoEnumIterator};

use std::rc::Rc;

use std::cell::LazyCell;
use std::sync::LazyLock;

use crate::Whatever;

static WHATEVER_VARIANT_STRS: LazyLock<Vec<&'static str>> = LazyLock::new(||
{
    
    let mut whatever_strs = Vec::with_capacity(Whatever::COUNT + 1);

    whatever_strs.push("*");

    for item in Whatever::iter()
    {

        whatever_strs.push(item.into());

    }

    whatever_strs

});

pub fn new_whatever_strs_dropdown() -> DropDown
{

    DropDown::from_strings(&WHATEVER_VARIANT_STRS)

}
