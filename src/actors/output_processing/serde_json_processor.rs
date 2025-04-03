use std::any::type_name_of_val;

use crate::{actors::MapageTypeActorOutputMessage, AllOrNot, TabIndenter};

use super::{check_for_nulls, ClientOutputter};

use async_recursion::async_recursion;
use corlib::inc_dec::IncDecSelf;
use libsync::{crossbeam::mpmc::tokio::array_queue::Sender, BoundedSendError};
use serde::Serialize;
use serde_json::{to_string_pretty, to_value, Value};
use strum::IntoEnumIterator;

use mapage_lib::{Command, CommandError, CommandResult, StreamedMessage, SupportedType, TypeInstance, Whatever};


pub struct SerdeJsonProcessor
{

    client_outputter: ClientOutputter

}

impl SerdeJsonProcessor
{

    pub fn new(sender: &Sender<MapageTypeActorOutputMessage>) -> Self
    {

        Self
        {
            
            client_outputter: ClientOutputter::new(sender)
        
        }

    }

    async fn send_serde_json_value_heading(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.client_outputter.send_str("serde_json::Value:: ...\n\n\n\n").await

    }

    #[async_recursion]
    async fn send_serde_json_value_enum_string_parts<'a>(&'a self, value: &Value, tab_indenter: &TabIndenter<'a>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        match value
        {

            serde_json::Value::Null =>
            {

                self.client_outputter.send_str("Null").await?;

            },
            serde_json::Value::Bool(val) =>
            {

                self.client_outputter.send_str("Bool(").await?;

                self.client_outputter.send_string(val.to_string()).await?;

                self.client_outputter.send_str(")").await?;

            }
            serde_json::Value::Number(number) =>
            {

                self.client_outputter.send_str("Number(").await?;

                self.client_outputter.send_string(number.to_string()).await?;

                self.client_outputter.send_str(")").await?;

            }
            serde_json::Value::String(string) =>
            {

                self.client_outputter.send_str("String(\"").await?;

                match check_for_nulls(string)
                {
                    Some(new_string) => self.client_outputter.send_string(new_string).await?,
                    None => self.client_outputter.send_string_clone(string).await?
                }

                self.client_outputter.send_str("\")").await?;

            }
            serde_json::Value::Array(vec) =>
            {

                self.client_outputter.send_str("Vec([\n\n").await?;

                let mut len = vec.len();

                let tab_indenter_elements = tab_indenter.next();

                for item in vec
                {

                    tab_indenter_elements.send_indentation().await?;

                    self.send_serde_json_value_enum_string_parts(item, &tab_indenter_elements).await?;

                    len.mm();

                    if len > 0
                    {

                        self.client_outputter.send_str(",\n\n").await?;

                    }
                    else
                    {

                        self.client_outputter.send_2_newlines().await?;
                        
                    }

                }

                tab_indenter.send_indentation().await?;

                self.client_outputter.send_str("])").await?;                     

            }
            serde_json::Value::Object(map) =>
            {

                self.client_outputter.send_str("Object({\n\n").await?;

                let mut len = map.len();

                let tab_indenter_fields = tab_indenter.next();

                for item in map
                {

                    tab_indenter_fields.send_indentation().await?;

                    self.client_outputter.send_str("\"").await?;

                    self.client_outputter.send_string_clone(item.0).await?;

                    self.client_outputter.send_str("\": ").await?;

                    self.send_serde_json_value_enum_string_parts(item.1, &tab_indenter_fields).await?;

                    len.mm();

                    if len > 0
                    {

                        self.client_outputter.send_str(",\n\n").await?;

                    }
                    else
                    {

                        self.client_outputter.send_2_newlines().await?;
                        
                    }

                }

                tab_indenter.send_indentation().await?;

                self.client_outputter.send_str("})").await?;

            }

        }

        Ok(())

    }

    async fn process_all(&self, all_or_not_supported_type: AllOrNot<SupportedType>, all_or_not_whatever: AllOrNot<Whatever>, all_or_not_type_instance: AllOrNot<TypeInstance>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.client_outputter.send_str("Process All:\n\n\n\n").await?;

        self.process_all_or_not_enum_input(all_or_not_supported_type).await?;

        self.process_all_or_not_enum_input(all_or_not_whatever).await?;

        self.process_all_or_not_enum_input( all_or_not_type_instance).await?;

        Ok(())

    }

    pub async fn process_all_message(&self, all_or_not_supported_type: AllOrNot<SupportedType>, all_or_not_whatever: AllOrNot<Whatever>, all_or_not_type_instance: AllOrNot<TypeInstance>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.process_all(all_or_not_supported_type, all_or_not_whatever, all_or_not_type_instance).await?;

        self.client_outputter.send_done().await

    }

    async fn process_all_default(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.client_outputter.send_str("Process All Default:\n\n\n\n").await?;

        self.process_all_or_not_enum_input::<SupportedType>(AllOrNot::All).await?;

        self.process_all_or_not_enum_input::<Whatever>(AllOrNot::All).await?;

        self.process_all_or_not_enum_input::<TypeInstance>(AllOrNot::All).await?;

        Ok(())

    }

    pub async fn process_all_default_message(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.process_all_default().await?;

        self.client_outputter.send_done().await

    }

    async fn process_input<T>(&self, item_instance: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Serialize + Clone
    {

        let item_value_res = to_value(item_instance);

        match item_value_res
        {

            Ok(res) =>
            {

                let tab_indenter = TabIndenter::new(self.client_outputter.sender()); //self.io_server.output_sender_ref()); 

                self.send_serde_json_value_enum_string_parts(&res, &tab_indenter).await?;

                self.client_outputter.send_2_newlines().await?;

                let item_string_res = to_string_pretty(&res);

                match item_string_res
                {

                    Ok(res) =>
                    {

                        self.client_outputter.send_string(res).await?;

                    }
                    Err(err) =>
                    {

                        self.client_outputter.send_error(err).await?;

                    }

                }

            }
            Err(err) =>
            {

                self.client_outputter.send_error(err).await?;

                self.client_outputter.send_2_newlines().await?;

                return Ok(());

            }

        }

        self.client_outputter.send_4_newlines().await?;

        Ok(())

    }

    async fn process_enum_input<T>(&self, item_instance: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Into<&'static str> + Serialize + Clone
    {

        self.client_outputter.send_str(type_name_of_val(&item_instance)).await?;
        
        self.client_outputter.send_str(":\n\n\n\n").await?;

        self.process_enum_input_variant( item_instance).await?;

        Ok(())

    }

    async fn process_enum_input_variant<T>(&self, item_instance: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Into<&'static str> + Serialize + Clone
    {

        self.client_outputter.send_str(item_instance.clone().into()).await?;

        self.client_outputter.send_2_newlines().await?;

        self.process_input( item_instance).await?;

        Ok(())

    }

    async fn process_struct_input<T>(&self, item_instance: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Serialize + Clone
    {

        self.client_outputter.send_str(type_name_of_val(&item_instance)).await?;
        
        self.client_outputter.send_str(":\n\n\n\n").await?;

        self.process_input(item_instance).await?;

        Ok(())

    }

    //Process All Supported Types or just one.

    async fn process_all_or_not_enum_input<T>(&self, all_or_not_input: AllOrNot<T>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Into<&'static str> + Serialize + Clone + IntoEnumIterator
    {

        self.client_outputter.send_str(type_name_of_val(&all_or_not_input)).await?;
        
        self.client_outputter.send_str(":\n\n\n\n").await?;

        match all_or_not_input
        {

            AllOrNot::All =>
            {

                self.client_outputter.send_str("AllOrNot::All\n\n\n\n").await?;

                for item in T::iter()
                {

                    self.process_enum_input_variant(item).await?;
            
                }

            }
            AllOrNot::NotAll(all_or_not_input_variant) =>
            {

                self.client_outputter.send_str("AllOrNot::NotAll\n\n\n\n").await?;

                self.process_enum_input_variant(all_or_not_input_variant).await?;

            }

        }

        Ok(())

    }

    pub async fn process_all_or_not_enum_input_message<T>(&self, all_or_not_input: AllOrNot<T>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Into<&'static str> + Serialize + Clone + IntoEnumIterator
    {

        self.send_serde_json_value_heading().await?;

        self.process_all_or_not_enum_input(all_or_not_input).await?;

        self.client_outputter.send_done().await

    }

    pub async fn process_command_message(&self, command: Command) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_serde_json_value_heading().await?;

        self.process_struct_input(command).await?;
        
        self.client_outputter.send_done().await

    }

    pub async fn process_command_result_message(&self, command_result: CommandResult) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_serde_json_value_heading().await?;

        self.process_struct_input(command_result).await?;
        
        self.client_outputter.send_done().await

    }

    pub async fn process_command_error_message(&self, command_error: CommandError) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_serde_json_value_heading().await?;

        self.process_struct_input(command_error).await?;
        
        self.client_outputter.send_done().await
 
    }

    pub async fn process_streamed_message_message(&self, streamed_message: StreamedMessage) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_serde_json_value_heading().await?;

        self.process_enum_input(streamed_message).await?;
        
        self.client_outputter.send_done().await

    }

}