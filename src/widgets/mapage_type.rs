use std::sync::LazyLock;

use gtk_estate::gtk4::DropDown;

use serde::{Serialize, Deserialize};

use strum::{EnumCount, IntoEnumIterator};

use strum_macros::{AsRefStr, EnumCount, EnumIter, EnumString, FromRepr, IntoStaticStr};

static MAPAGE_TYPE_VARIANT_STRS: LazyLock<Vec<&'static str>> = LazyLock::new(||
{
    
    let mut mapage_type_strs = Vec::with_capacity(MapageType::COUNT + 1);

    mapage_type_strs.push("*");

    for item in MapageType::iter()
    {

        mapage_type_strs.push(item.into());

    }

    mapage_type_strs

});

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, EnumString, FromRepr, EnumIter, AsRefStr, EnumCount, IntoStaticStr)]
pub enum MapageType
{

    #[default]
    SupportedType,
    Whatever,
    TypeInstance,
    Command,
    CommandResult,
    CommandError,
    StreamedMessage

}

pub fn new_mapage_type_strs_dropdown() -> DropDown
{

    DropDown::from_strings(&MAPAGE_TYPE_VARIANT_STRS)

}
