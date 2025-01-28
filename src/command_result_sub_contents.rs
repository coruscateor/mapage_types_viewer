use std::{cell::Cell, num::ParseIntError, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::property::PropertyGet, prelude::EntryBufferExtManual}, gtk4::{prelude::{BoxExt, Cast, WidgetExt}, CheckButton, Text}, helpers::text_view::get_text_view_string, WidgetContainer};

use crate::{try_get_id, widgets::{new_bool_strs_dropdown, new_supported_type_strs_dropdown}, AllOrNot, CommandResult, ParamsSubContents, SupportedType, SupportedTypeSubContents, TypeInstanceSubContents, WindowContentsState};

use corlib::{cell::RefCellStore, events::PubSingleSubEvent, impl_pub_single_sub_event_method, upgrading::try_up_rc, value::{HasOptionalValueGetter, HasValueGetter}};

use corlib::events::SingleSubEvent; 

use gtk_estate::gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, Widget};

use gtk_estate::gtk4::glib::clone;

use crate::{OptionalValueSubContents, Command};

pub type RcCommandResultSubContents = Rc<CommandResultSubContents>;

pub struct CommandResultSubContents
{

    id_text: Text,
    result_optional_type_name_sub_contents: Rc<OptionalValueSubContents<TypeInstanceSubContents<Self>>>,
    done_dropdown: DropDown,
    contents_box: Box

}

impl CommandResultSubContents
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

        let result_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let result_label = Label::builder().label("result").halign(Align::Start).build();

        result_box.append(&result_label);

        //

        let result_optional_type_name_sub_contents = OptionalValueSubContents::new(TypeInstanceSubContents::new());

        result_box.append(result_optional_type_name_sub_contents.widget_ref());

        //

        contents_box.append(&result_box);

        //

        let done_box = Box::builder().orientation(Orientation::Vertical).spacing(2).build();

        //

        let done_label  = Label::builder().label("done").halign(Align::Start).build();

        done_box.append(&done_label);

        //

        let done_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).build();

        let done_dropdown = new_bool_strs_dropdown();

        done_dropdown.set_width_request(80);

        done_dropdown_box.append(&done_dropdown);

        done_box.append(&done_dropdown_box);

        //

        contents_box.append(&done_box);

        //

        let this = Rc::new_cyclic(|_weak_self|
        {

            Self
            {

                id_text,
                result_optional_type_name_sub_contents,
                done_dropdown,
                contents_box

            }
        
        });

        this

    }

}


impl WidgetContainer for CommandResultSubContents
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

impl HasValueGetter for CommandResultSubContents
{

    type HasValueType = Result<CommandResult, String>;

    fn value(&self) -> Self::HasValueType
    {

        //id

        let id_text_string = self.id_text.buffer().text();

        let id= try_get_id(&id_text_string, "id")?;

        //result

        let result;

        let opt_processing_result = self.result_optional_type_name_sub_contents.value();

        match opt_processing_result
        {
            
            Some(res) =>
            {

                result = Some(res?);

            }
            None =>
            {

                result = None;

            }

        }

        //done

        let done;

        let obj = self.done_dropdown.selected_item().expect("Error: An item ust be selected");

        let str_obj = obj.downcast_ref::<StringObject>().expect("Error: Selected Item ust be StringObject");

        match bool::from_str(&str_obj.string())
        {

            Ok(res) =>
            {

                done = res;

            }
            Err(err) =>
            {

                return Err(err.to_string());

            }

        }

        //

        let command_result = CommandResult::new(id, result, done);

        Ok(command_result)

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

