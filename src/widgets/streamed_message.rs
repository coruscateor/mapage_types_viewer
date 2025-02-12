use std::sync::LazyLock;

use gtk_estate::gtk::DropDown;

use serde::{Serialize, Deserialize};

use strum::{EnumCount, IntoEnumIterator};

use strum_macros::{AsRefStr, EnumCount, EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::StreamedMessage;

pub static STREAMED_MESSAGE_STRS: LazyLock<Vec<&'static str>> = LazyLock::new(||
{
    
    let mut streamed_message_strs = Vec::with_capacity(StreamedMessage::COUNT); // + 1);

    //streamed_message_strs.push("*");

    for item in StreamedMessage::iter()
    {

        streamed_message_strs.push(item.into());

    }

    streamed_message_strs

});

pub fn new_streamed_message_strs_dropdown() -> DropDown
{

    DropDown::from_strings(&STREAMED_MESSAGE_STRS)

}
