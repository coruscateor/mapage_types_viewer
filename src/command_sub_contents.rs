use std::{cell::Cell, num::ParseIntError, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::property::PropertyGet, prelude::EntryBufferExtManual}, gtk4::{prelude::{BoxExt, Cast, WidgetExt}, CheckButton, Text}, helpers::text_view::get_text_view_string, WidgetContainer};

use crate::{try_get_id, widgets::new_supported_type_strs_dropdown, AllOrNot, ParamsSubContents, SupportedType, SupportedTypeSubContents, WindowContentsState};

use corlib::{cell::RefCellStore, events::PubSingleSubEvent, impl_pub_single_sub_event_method, upgrading::try_up_rc, value::{HasOptionalValueGetter, HasValueGetter}};

use corlib::events::SingleSubEvent; 

use gtk_estate::gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, Widget};

use gtk_estate::gtk4::glib::clone;

use crate::{OptionalValueSubContents, Command};

pub type RcCommandSubContents = Rc<CommandSubContents>;

//#[derive(Debug)]
pub struct CommandSubContents
{

    //command_result: RefCellStore<Result<Command, String>>,
    id_text: Text,
    command_text: Text,
    //type_name_check_button: CheckButton,
    optional_type_name_sub_contents: Rc<OptionalValueSubContents<SupportedTypeSubContents<Self>>>,
    //params_check_button: CheckButton,
    optional_params_sub_contents: Rc<OptionalValueSubContents<ParamsSubContents<Self>>>,
    contents_box: Box

}

impl CommandSubContents
{

    pub fn new() -> Rc<Self>
    {

        let contents_box = Box::builder().orientation(Orientation::Vertical).spacing(4).visible(true).build();

        //

        let command_text_label = Label::builder().label("Command").halign(Align::Start).build();

        contents_box.append(&command_text_label);

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

        let command_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let command_text_label = Label::builder().label("command").halign(Align::Start).build();

        command_box.append(&command_text_label);

        //

        let command_text = Text::new();

        command_box.append(&command_text);

        //

        contents_box.append(&command_box);

        //

        let type_name_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let type_name_label  = Label::builder().label("type name").halign(Align::Start).build();

        type_name_box.append(&type_name_label);

        //

        let optional_type_name_sub_contents = OptionalValueSubContents::new(SupportedTypeSubContents::new());

        type_name_box.append(optional_type_name_sub_contents.widget_ref());

        //

        contents_box.append(&type_name_box);

        //

        let params_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let params_label  = Label::builder().label("params").halign(Align::Start).build();

        params_box.append(&params_label);

        //

        let optional_params_sub_contents = OptionalValueSubContents::new(ParamsSubContents::new());

        params_box.append(optional_params_sub_contents.widget_ref());

        //

        contents_box.append(&params_box);

        //

        let this = Rc::new_cyclic(|_weak_self|
        {

            Self
            {

                //command_result,
                id_text,
                command_text,
                optional_type_name_sub_contents,
                optional_params_sub_contents,
                contents_box

            }
        
        });

        this

    }

    /*
    pub fn widget_ref(&self) -> &Box
    {

        &self.command_box

    }
    */

    //impl_pub_single_sub_event_method!(on_supported_type_str_selected, WindowContentsState);

}


impl WidgetContainer for CommandSubContents
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

impl HasValueGetter for CommandSubContents
{

    type HasValueType = Result<Command, String>;

    fn value(&self) -> Self::HasValueType
    {

        //id

        let id_text_string = self.id_text.buffer().text(); //get_text_view_string(&self.id_text);

        let id= try_get_id(&id_text_string, "id")?;

        /*
        let trimmed_id_text_string = id_text_string.trim();

        if trimmed_id_text_string.is_empty()
        {

            id = None;

        }
        else
        {

            let id_number_result = u32::from_str(trimmed_id_text_string);

            match id_number_result
            {

                Ok(res) =>
                {

                    id = Some(res);

                }
                Err(err) =>
                {

                    return Err(err.to_string());

                }

            }
        
        }
        */

        //command

        let command_text_string = self.command_text.buffer().text().to_string();

        //type_name

        let type_name = self.optional_type_name_sub_contents.value();

        //params

        let params;

        let params_opt_result = self.optional_params_sub_contents.value();

        match params_opt_result
        {
            
            Some(res) =>
            {

                params = Some(res?);

            }
            None =>
            {

                params = None;

            }

        }

        //

        let command = Command::new(id, command_text_string, type_name, params);

        Ok(command)

    }

}

/*
only traits defined in the current crate can be implemented for types defined outside of the crate
impl doesn't have any local type before any uncovered type parameters
for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
define and implement a trait or new type insteadrustcClick for full compiler diagnostic
command_sub_contents.rs(153, 23): `ParseIntError` is not defined in the current crate
command_sub_contents.rs(153, 6): `std::string::String` is not defined in the current crate
*/

/*
impl From<String> for ParseIntError
{



}
*/

