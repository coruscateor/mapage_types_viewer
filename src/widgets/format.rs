use std::sync::LazyLock;

use gtk_estate::gtk4::DropDown;

use serde::{Serialize, Deserialize};

use strum::{EnumCount, IntoEnumIterator};

use strum_macros::{AsRefStr, EnumCount, EnumIter, EnumString, FromRepr, IntoStaticStr};

static OUTPUT_FORMAT_STRS: LazyLock<Vec<&'static str>> = LazyLock::new(||
{
    
    let mut supported_type_strs = Vec::with_capacity(OutputFormat::COUNT);

    for item in OutputFormat::iter()
    {

        supported_type_strs.push(item.into());

    }

    supported_type_strs

});

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, EnumString, FromRepr, EnumIter, AsRefStr, EnumCount, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
pub enum OutputFormat
{

    #[default]
    Json

}

pub fn output_format_strs_dropdown() -> DropDown
{

    DropDown::from_strings(&OUTPUT_FORMAT_STRS)

}

