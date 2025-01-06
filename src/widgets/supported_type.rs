use gtk_estate::{gtk4::DropDown, RcWidgetAdapter, StoredWidgetObject, WidgetAdapter, WidgetStateContainer};

use strum::{EnumCount, IntoEnumIterator};

use std::rc::Rc;

use std::cell::LazyCell;
use std::sync::LazyLock;

use crate::SupportedType;

static SUPPORTED_TYPE_VARIANT_STRS: LazyLock<Vec<&'static str>> = LazyLock::new(||
{

    let mut supported_type_strs = Vec::with_capacity(SupportedType::COUNT + 1);

    supported_type_strs.push("*");

    for item in SupportedType::iter()
    {

        supported_type_strs.push(item.into());

    }

    supported_type_strs

});

pub fn new_supported_type_strs_dropdown() -> DropDown
{

    DropDown::from_strings(&SUPPORTED_TYPE_VARIANT_STRS)

}
