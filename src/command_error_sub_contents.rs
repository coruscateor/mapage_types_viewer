use std::{cell::Cell, num::ParseIntError, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::property::PropertyGet, prelude::EntryBufferExtManual}, gtk4::{prelude::{BoxExt, Cast, WidgetExt}, CheckButton, Text}, helpers::text_view::get_text_view_string, WidgetContainer};

use crate::{try_get_id, try_get_usize_index, widgets::{new_bool_strs_dropdown, new_supported_type_strs_dropdown}, AllOrNot, CommandError, CommandResult, ParamsSubContents, SupportedType, SupportedTypeSubContents, TypeInstanceSubContents, WindowContentsState};

use corlib::{cell::RefCellStore, events::PubSingleSubEvent, impl_pub_single_sub_event_method, text::SendableText, upgrading::try_up_rc, value::{HasOptionalValueGetter, HasValueGetter}};

use corlib::events::SingleSubEvent; 

use gtk_estate::gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, Widget};

use gtk_estate::gtk4::glib::clone;

use crate::{OptionalValueSubContents, Command};

pub struct CommandErrorSubContents
{

    id_text: Text,
    message_text: Text,
    index_text: Text,
    found_type_text: Text,
    contents_box: Box

}

impl CommandErrorSubContents
{

    pub fn new() -> Rc<Self>
    {

        let contents_box = Box::builder().orientation(Orientation::Vertical).spacing(4).visible(true).build();

        //

        let command_result_text_label = Label::builder().label("Command Result").halign(Align::Start).build();

        contents_box.append(&command_result_text_label);

        //

        let id_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let id_text_label = Label::builder().label("id").halign(Align::Start).build();

        id_box.append(&id_text_label);

        //

        let id_text = Text::new();

        id_box.append(&id_text);

        //

        contents_box.append(&id_box);

        //

        let message_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let message_label = Label::builder().label("message").halign(Align::Start).build();

        message_box.append(&message_label);

        //

        let message_text = Text::new();

        message_box.append(&message_text);

        //

        contents_box.append(&message_box);

        //

        let index_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let index_label = Label::builder().label("index").halign(Align::Start).build();

        index_box.append(&index_label);

        //

        let index_text = Text::new();

        index_box.append(&index_text);

        //

        contents_box.append(&index_box);


        //

        let found_type_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let found_type_label = Label::builder().label("found_type").halign(Align::Start).build();

        found_type_box.append(&found_type_label);

        //

        let found_type_text = Text::new();

        found_type_box.append(&found_type_text);

        //

        contents_box.append(&found_type_box);

        //

        let this = Rc::new_cyclic(|_weak_self|
        {

            Self
            {

                id_text,
                message_text,
                index_text,
                found_type_text,
                contents_box

            }
        
        });

        this

    }

}

impl WidgetContainer for CommandErrorSubContents
{

    fn widget(&self) -> Widget
    {

        self.contents_box.upcast_ref::<Widget>().clone()
        
    }

    fn widget_ref(&self) -> &Widget
    {

        self.contents_box.upcast_ref::<Widget>()
        
    }

}

impl HasValueGetter for CommandErrorSubContents
{

    type HasValueType = Result<CommandError, String>;

    fn value(&self) -> Self::HasValueType
    {

        //id

        let id_text_string = self.id_text.buffer().text();

        let id= try_get_id(&id_text_string, "id")?;

        //message

        let message = SendableText::String(self.message_text.buffer().text().into());

        //index

        let index_text_string = self.index_text.buffer().text();

        let index= try_get_usize_index(&index_text_string, "index")?;

        //found_type

        let found_type;

        let found_type_text_buffer_text = self.found_type_text.buffer().text();

        if found_type_text_buffer_text.is_empty()
        {

            found_type = None;

        }
        else
        {

            found_type = Some(SendableText::String(found_type_text_buffer_text.into()));
            
        }

        //

        let command_error = CommandError { id, message, index, found_type };

        Ok(command_error)

    }

}
