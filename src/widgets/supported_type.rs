use gtk_estate::{gtk4::DropDown, RcWidgetAdapter, StoredWidgetObject, WidgetAdapter, WidgetStateContainer};

use strum::{EnumCount, IntoEnumIterator};

use std::rc::Rc;

use std::cell::LazyCell;
use std::sync::LazyLock;

use crate::SupportedType;

//thread_local!
//{

static SUPPORTED_TYPE_VARIANT_STRS: LazyLock<Vec<&'static str>> = LazyLock::new(|| { //LazyCell<Vec<&'static str>> = LazyCell::new(|| {

    //: Vec<&'static str>
    
    let mut supported_type_strs = Vec::with_capacity(SupportedType::COUNT + 1);

    supported_type_strs.push("*");

    for item in SupportedType::iter()
    {

        supported_type_strs.push(item.into());

    }

    supported_type_strs

});

//}

pub fn new_supported_type_strs_dropdown() -> DropDown
{

    DropDown::from_strings(&SUPPORTED_TYPE_VARIANT_STRS)

}

/*
pub struct SupportedTypeState
{

    dropdown_adapter: RcWidgetAdapter<DropDown, SupportedTypeState>

}

impl SupportedTypeState
{

    pub fn new() -> Rc<Self>
    {

        Rc::new_cyclic(|weak_self|
        {

            let dd = DropDown::from_strings(&SUPPORTED_TYPE_STRS); //(&[&"strings"]);

            Self
            {
    
                dropdown_adapter: WidgetAdapter::new(&dd, weak_self)
    
            }
            
        })

    }

    pub fn dropdown_adapter(&self) -> RcWidgetAdapter<DropDown, SupportedTypeState>
    {

        self.dropdown_adapter.clone()

    }

    pub fn dropdown_adapter_ref(&self) -> &WidgetAdapter<DropDown, SupportedTypeState>
    {

        &self.dropdown_adapter

    }

}

impl_widget_state_container!(dropdown_adapter, SupportedTypeState);
*/
