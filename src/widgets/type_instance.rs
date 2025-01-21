use gtk_estate::{gtk4::DropDown, RcWidgetAdapter, StoredWidgetObject, WidgetAdapter, WidgetStateContainer};

use strum::{EnumCount, IntoEnumIterator};

use std::rc::Rc;

use std::cell::LazyCell;
use std::sync::LazyLock;

use crate::TypeInstance;

static TYPE_INSTANCE_VARIANT_STRS: LazyLock<Vec<&'static str>> = LazyLock::new(||
{
    
    let mut type_instance_strs = Vec::with_capacity(TypeInstance::COUNT + 1);

    type_instance_strs.push("*");

    for item in TypeInstance::iter()
    {

        type_instance_strs.push(item.into());

    }

    type_instance_strs

});

static TYPE_INSTANCE_VARIANT_STRS_NO_ALL: LazyLock<Vec<&'static str>> = LazyLock::new(||
{
    
    let mut type_instance_strs = Vec::with_capacity(TypeInstance::COUNT);

    for item in TypeInstance::iter()
    {

        type_instance_strs.push(item.into());

    }

    type_instance_strs

});

pub fn new_type_instance_strs_dropdown() -> DropDown
{

    DropDown::from_strings(&TYPE_INSTANCE_VARIANT_STRS)

}

pub fn new_type_instance_strs_no_all_dropdown() -> DropDown
{

    DropDown::from_strings(&TYPE_INSTANCE_VARIANT_STRS_NO_ALL)

}
