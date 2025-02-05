use std::{cell::Cell, num::ParseIntError, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::property::PropertyGet, prelude::EntryBufferExtManual}, gtk4::{prelude::{BoxExt, Cast, WidgetExt}, CheckButton, Text}, helpers::text_view::get_text_view_string, WidgetContainer};

use crate::{try_get_id, widgets::{new_streamed_message_strs_dropdown, new_supported_type_strs_dropdown, STREAMED_MESSAGE_STRS}, AllOrNot, CommandErrorSubContents, CommandResultSubContents, CommandSubContents, ParamsSubContents, RcCommandErrorSubContents, RcCommandResultSubContents, RcCommandSubContents, StreamedMessage, SupportedType, SupportedTypeSubContents, WindowContentsState};

use corlib::{cell::RefCellStore, events::PubSingleSubEvent, impl_pub_single_sub_event_method, text::SendableText, upgrading::try_up_rc, value::{HasOptionalValueGetter, HasValueGetter}};

use corlib::events::SingleSubEvent; 

use gtk_estate::gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, Widget};

use gtk_estate::gtk4::glib::clone;

use crate::{OptionalValueSubContents, Command};

#[derive(Debug)]
pub enum StreamedMessageSubContentsContents
{

    Command(RcCommandSubContents),
    CommandResult(RcCommandResultSubContents),
    CommandError(RcCommandErrorSubContents),
    Error(Text),

}

#[derive(Debug)]
pub struct StreamedMessageSubContents
{

    streamed_message_sub_contents_contents: RefCellStore<StreamedMessageSubContentsContents>,
    contents_box: Box,
    streamed_message_strs_dropdown: DropDown

}

impl StreamedMessageSubContents
{

    pub fn new() -> Rc<Self>
    {

        let contents_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        //

        let streamed_message_label = Label::builder().label("StreamedMessage").halign(Align::Start).build();

        contents_box.append(&streamed_message_label);

        //

        let streamed_message_label = Label::builder().label(STREAMED_MESSAGE_STRS[0]).halign(Align::Start).build();

        contents_box.append(&streamed_message_label);

        //

        let streamed_message_strs_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).build();

        let streamed_message_strs_dropdown = new_streamed_message_strs_dropdown();

        streamed_message_strs_box.append(&streamed_message_strs_dropdown);

        streamed_message_strs_dropdown.set_width_request(160);

        contents_box.append(&streamed_message_strs_box);

        //

        let command_sub_contents = CommandSubContents::new();

        contents_box.append(command_sub_contents.widget_ref());

        let streamed_message_sub_contents_contents = RefCellStore::new(StreamedMessageSubContentsContents::Command(command_sub_contents));

        //

        let this = Rc::new_cyclic(|_weak_self|
        {

            Self
            {

                streamed_message_sub_contents_contents,
                contents_box,
                streamed_message_strs_dropdown

            }
        
        });

        this.streamed_message_strs_dropdown.connect_selected_notify(clone!( #[strong] this, move |streamed_message_strs_dropdown|
        {

            //Remove the current StreamedMessageSubContents object.

            this.streamed_message_sub_contents_contents.borrow_with_param(this.clone(), |state, this|
            {

                match &*state
                {

                    StreamedMessageSubContentsContents::Command(command_sub_contents) =>
                    {

                        this.contents_box.remove(command_sub_contents.widget_ref());

                    }
                    StreamedMessageSubContentsContents::CommandResult(command_result_sub_contents) =>
                    {
                        
                        this.contents_box.remove(command_result_sub_contents.widget_ref());

                    }
                    StreamedMessageSubContentsContents::CommandError(command_error_sub_contents) =>
                    {

                        this.contents_box.remove(command_error_sub_contents.widget_ref());

                    }
                    StreamedMessageSubContentsContents::Error(text) =>
                    {

                        this.contents_box.remove(text);

                    }

                }

            });

            let selected_item = streamed_message_strs_dropdown.selected_item().expect("Error: Nothing selected");

            let selected_string_object = selected_item.downcast_ref::<StringObject>().expect("Error: selected_item must be a StringObject.");

            let streamed_message = StreamedMessage::from_str(&selected_string_object.string()).expect("Error: ParsingError");

            this.streamed_message_sub_contents_contents.borrow_mut_with_param((streamed_message, this.clone()), |mut state, (streamed_message, this)|
            {

                match streamed_message
                {

                    StreamedMessage::Command(_command) =>
                    {

                        let command_sub_contents = CommandSubContents::new();

                        this.contents_box.append(command_sub_contents.widget_ref());

                        *state = StreamedMessageSubContentsContents::Command(command_sub_contents);

                    }
                    StreamedMessage::CommandResult(_command_result) =>
                    {

                        let command_result_sub_contents = CommandResultSubContents::new();

                        this.contents_box.append(command_result_sub_contents.widget_ref());

                        *state = StreamedMessageSubContentsContents::CommandResult(command_result_sub_contents);

                    }
                    StreamedMessage::CommandError(_command_error) =>
                    {

                        let command_error_sub_contents = CommandErrorSubContents::new();

                        this.contents_box.append(command_error_sub_contents.widget_ref());

                        *state = StreamedMessageSubContentsContents::CommandError(command_error_sub_contents);

                    }
                    StreamedMessage::Error(_sendable_text) =>
                    {

                        let error_text = Text::new();

                        this.contents_box.append(&error_text);

                        *state = StreamedMessageSubContentsContents::Error(error_text);

                    }

                }

            });

        }));

        this

    }

}


impl WidgetContainer for StreamedMessageSubContents
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

impl HasValueGetter for StreamedMessageSubContents
{

    type HasValueType = Result<StreamedMessage, String>;

    fn value(&self) -> Self::HasValueType
    {

        self.streamed_message_sub_contents_contents.borrow(|state|
        {

            match &*state
            {

                StreamedMessageSubContentsContents::Command(command_sub_contents) => 
                {
                    
                    Ok(StreamedMessage::Command(command_sub_contents.value()?))

                }
                StreamedMessageSubContentsContents::CommandResult(command_result_sub_contents) =>
                {

                    Ok(StreamedMessage::CommandResult(command_result_sub_contents.value()?))

                }
                StreamedMessageSubContentsContents::CommandError(command_error_sub_contents) =>
                {

                    Ok(StreamedMessage::CommandError(command_error_sub_contents.value()?))

                }
                StreamedMessageSubContentsContents::Error(text) =>
                {

                    Ok(StreamedMessage::Error(SendableText::String(text.buffer().text().to_string())))

                }

            }

        })

    }

}

