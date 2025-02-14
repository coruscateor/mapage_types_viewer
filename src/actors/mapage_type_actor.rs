use std::{error::Error, fmt::{Debug, Display}};

use act_rs::{impl_on_started_and_ending_async, impl_mac_task_actor};

use corlib::{inc_dec::IncDecSelf, text::SendableText, WorkInProgressResult};
use paste::paste;

//use act_rs::{impl_default_end_async, impl_default_start_async}; //Remove impl_default_start_and_end_async macro dependancies.

use serde::Serialize;
use serde_json::{to_string, to_string_pretty, to_value, Value};
use strum::IntoEnumIterator;
use tokio::task::JoinHandle;

use libsync::{crossbeam::mpmc::tokio::array_queue::io_channels::{io_channels, IOClient, IOServer}, BoundedSendError};

use libsync::crossbeam::mpmc::tokio::array_queue::Sender;

use crate::{widgets::{MapageType, OutputFormat}, AllOrNot, Command, CommandError, CommandResult, StreamedMessage, SupportedType, TypeInstance, Whatever};

use async_recursion::async_recursion;

use crate::TabIndenter;

use corlib::cell::RefCellStore;

use std::any::{type_name, type_name_of_val};

#[derive(Debug)]
pub enum MapageTypeActorInputMessage
{

    ProcessAll(OutputFormat, AllOrNot<SupportedType>, AllOrNot<Whatever>, AllOrNot<TypeInstance>),
    ProcessAllDefault(OutputFormat),
    ProcessSupportedType(OutputFormat, AllOrNot<SupportedType>),
    ProcessWhatever(OutputFormat, AllOrNot<Whatever>),
    ProcessTypeInstance(OutputFormat, AllOrNot<TypeInstance>),
    ProcessCommand(OutputFormat, Command),
    ProcessCommandResult(OutputFormat, CommandResult),
    ProcessCommandError(OutputFormat, CommandError),
    ProcessStreamedMessage(OutputFormat, StreamedMessage)
    
}

impl Display for MapageTypeActorInputMessage
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            MapageTypeActorInputMessage::ProcessAll(output_format, all_or_not_supported_type, whatever_sub_contents, all_or_not_type_instance) =>
            {

                write!(f, "ProcessAll({output_format:?}, {all_or_not_supported_type:?}, {whatever_sub_contents:?}, {all_or_not_type_instance:?})")

            }
            MapageTypeActorInputMessage::ProcessAllDefault(output_format) =>
            {

                write!(f, "ProcessAllDefault({output_format:?})")

            }
            MapageTypeActorInputMessage::ProcessSupportedType(output_format, all_or_not_supported_type) =>
            {
                
                write!(f, "ProcessSupportedType({output_format:?}, {all_or_not_supported_type:?})")

            }
            MapageTypeActorInputMessage::ProcessWhatever(output_format, all_or_not_whatever) =>
            {

                write!(f, "ProcessWhatever({output_format:?}, {all_or_not_whatever:?})")

            }
            MapageTypeActorInputMessage::ProcessTypeInstance(output_format, all_or_not_type_instance) =>
            {

                write!(f, "ProcessTypeInstance({output_format:?}, {all_or_not_type_instance:?})")

            },
            MapageTypeActorInputMessage::ProcessCommand(output_format, command) =>
            {

                write!(f, "ProcessCommand({output_format:?}, {command:?})")

            }
            MapageTypeActorInputMessage::ProcessCommandResult(output_format, command_result) =>
            {

                write!(f, "ProcessCommandResult({output_format:?}, {command_result:?})")

            }
            MapageTypeActorInputMessage::ProcessCommandError(output_format, command_error) =>
            {

                write!(f, "ProcessCommandResult({output_format:?}, {command_error:?})")

            }
            MapageTypeActorInputMessage::ProcessStreamedMessage(output_format, streamed_message) =>
            {

                write!(f, "ProcessStreamedMessage{output_format:?}, {streamed_message:?})")

            },

        }
        
    }

}

#[derive(Debug)]
pub enum MapageTypeActorOutputMessage
{

    WorkInProgressTextResult(WorkInProgressResult<SendableText>)

}

impl Display for MapageTypeActorOutputMessage
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            MapageTypeActorOutputMessage::WorkInProgressTextResult(work_in_progress_result) =>
            {
                
                write!(f, "WorkInProgressTextResult({work_in_progress_result})")

            }

        }
        
    }

}

pub struct MapageTypeActorState
{

    io_server: IOServer<MapageTypeActorInputMessage, MapageTypeActorOutputMessage> //,
    //test_rfc_store_state: RefCellStore<()>

}

impl MapageTypeActorState
{

    pub fn new() -> (IOClient<MapageTypeActorInputMessage, MapageTypeActorOutputMessage>, Self)
    {

        let (io_client, io_server) = io_channels(2, 100);

        (io_client, Self
        {

            io_server //,
            //test_rfc_store_state: RefCellStore::new(())

        })

    }

    pub fn spawn() -> IOClient<MapageTypeActorInputMessage, MapageTypeActorOutputMessage>
    {

        let (io_client, actor_state) = MapageTypeActorState::new();

        MapageTypeActor::spawn(actor_state);

        io_client

    }

    impl_on_started_and_ending_async!();

    async fn run_async(&mut self) -> bool
    {

        match self.io_server.input_receiver_ref().recv().await
        {

            Some(message) =>
            {

                let processing_res;

                match message
                {
    
                    MapageTypeActorInputMessage::ProcessAll(output_format, all_or_not_supported_type, all_or_not_whatever, all_or_not_type_instance) =>
                    {
                        
                        processing_res = self.process_all_message(output_format, all_or_not_supported_type, all_or_not_whatever, all_or_not_type_instance).await; //output_sender, 
        
                    }
                    MapageTypeActorInputMessage::ProcessAllDefault(output_format) =>
                    {
                        
                        processing_res = self.process_all_default_message(output_format).await; //, output_sender
                        
                    }
                    MapageTypeActorInputMessage::ProcessSupportedType(output_format, all_or_not_supported_type) =>
                    {
    
                        processing_res = self.process_all_or_not_enum_input_message(output_format, all_or_not_supported_type).await;

                        //processing_res = self.process_all_or_not_supported_type_message(output_format, all_or_not_supported_type).await; //, output_sender
    
                    }
                    MapageTypeActorInputMessage::ProcessWhatever(output_format, all_or_not_whatever) =>
                    {

                        processing_res = self.process_all_or_not_enum_input_message(output_format, all_or_not_whatever).await;
    
                        //processing_res = self.process_all_or_not_whatever_message(output_format, all_or_not_whatever).await; //, output_sender
    
                    }
                    MapageTypeActorInputMessage::ProcessTypeInstance(output_format, all_or_not_type_instance) =>
                    {

                        processing_res = self.process_all_or_not_enum_input_message(output_format, all_or_not_type_instance).await;

                    }
                    MapageTypeActorInputMessage::ProcessCommand(output_format, command) =>
                    {

                        processing_res = self.process_command_message(output_format, command).await;

                    }
                    MapageTypeActorInputMessage::ProcessCommandResult(output_format, command_result) =>
                    {

                        processing_res = self.process_command_result_message(output_format, command_result).await;

                    }
                    MapageTypeActorInputMessage::ProcessCommandError(output_format, command_error) =>
                    {

                        processing_res = self.process_command_error_message(output_format, command_error).await;

                    }
                    MapageTypeActorInputMessage::ProcessStreamedMessage(output_format, streamed_message) =>
                    {

                        processing_res = self.process_streamed_message_message(output_format, streamed_message).await;

                    }
    
                }

                if let Err(err) = processing_res
                {
        
                    print_display(err);
        
                    false
        
                }
                else
                {
        
                    true
                    
                }

            }
            None =>
            {
                
                //print!("MapageTypeActorState: Empty message received.");

                false

            }

        }

        //let output_sender = self.io_server.output_sender_ref().clone();

        //let output_sender = self.io_server.output_sender_ref();

        /*
        let processing_res;

        if let Some(message) = self.io_server.input_receiver_ref().recv().await
        {

            match message
            {

                MapageTypeActorInputMessage::ProcessAll(output_format, all_or_not_supported_type, all_or_not_whatever) =>
                {
                    
                    processing_res = self.process_all_message(output_format, all_or_not_supported_type, all_or_not_whatever).await; //output_sender, 
    
                }
                MapageTypeActorInputMessage::ProcessAllDefault(output_format) =>
                {
                    
                    processing_res = self.process_all_default_message(output_format).await; //, output_sender
                    
                }
                MapageTypeActorInputMessage::ProcessSupportedType(output_format, all_or_not_supported_type) =>
                {

                    processing_res = self.process_all_or_not_supported_type_message(output_format, all_or_not_supported_type).await; //, output_sender

                }
                MapageTypeActorInputMessage::ProcessWhatever(output_format, all_or_not_whatever) =>
                {

                    processing_res = self.process_all_or_not_whatever_message(output_format, all_or_not_whatever).await; //, output_sender

                }

            }

        }
        else
        {

            print!("MapageTypeActorState: Empty message received.");

            return false;
            
        }

        if let Err(err) = processing_res
        {

            print_display(err);

            false

        }
        else
        {

            true
            
        }
        */

    }

    async fn send_sendable_text(&self, sendable_text: SendableText) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.io_server.output_sender_ref().send(MapageTypeActorOutputMessage::WorkInProgressTextResult(WorkInProgressResult::not_done(sendable_text))).await

    }

    async fn send_str(&self, sendable_text: &'static str) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str(sendable_text)).await

    }

    async fn send_string_clone(&self, sendable_text: &String) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::String(sendable_text.clone())).await

    }

    async fn send_string(&self, sendable_text: String) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::String(sendable_text)).await

    }

    async fn send_2_newlines(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str("\n\n")).await

    }

    async fn send_4_newlines(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str("\n\n\n\n")).await

    }

    async fn send_error<E>(&self, error: E) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where E: Error + ToString
    {

        self.send_string(error.to_string()).await

    }

    /*
    async fn send_done_sendable_text(&mut self, sendable_text: SendableText) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.io_server.output_sender_ref().send(MapageTypeActorOutputMessage::WorkInProgressTextResult(WorkInProgressResult::done(sendable_text))).await

    }
    */

    async fn send_done(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.io_server.output_sender_ref().send(MapageTypeActorOutputMessage::WorkInProgressTextResult(WorkInProgressResult::done_none())).await

    }

    /*
    async fn send_not_done_enum_string_parts<T>(&mut self, item: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Serialize
    {

        let item_value_res = to_value(item);

        match item_value_res
        {

            Ok(res) =>
            {

                //let res_string = SendableText::String(res.to_string());

                //self.send_not_done_sendable_text(res_string).await?;

                //self.send_not_done_2_newlines().await?;

                Ok(())

            }
            Err(err) =>
            {

                let err_string = SendableText::String(err.to_string());

                self.send_not_done_sendable_text(err_string).await?;

                self.send_not_done_2_newlines().await?;

                Ok(())

            }

        }

    }
    */

    async fn send_serde_json_value_heading(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_str("serde_json::Value:: ...\n\n\n\n").await

    }

    async fn send_value_enum_heading(&self, output_format: OutputFormat) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        match output_format
        {

            OutputFormat::Json =>
            {

                self.send_serde_json_value_heading().await

            }

        }

    }

    //The GTK TextView Buffer doesn't like it when you try to append null characters to it.

    fn check_for_nulls(string: &String) -> Option<String>
    {

        if string.contains('\0')
        {

            Some(string.replace('\0', "\\0"))

        }
        else
        {

            None
            
        }

    }

    #[async_recursion]
    async fn send_serde_json_value_enum_string_parts<'a>(&'a self, value: &Value, tab_indenter: &TabIndenter<'a>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        match value
        {

            serde_json::Value::Null =>
            {

                //tab_indenter.send_indentation().await?;

                self.send_str("Null").await?;

                //self.send_str("serde_json::Value::Null").await?;

            },
            serde_json::Value::Bool(val) =>
            {

                //tab_indenter.send_indentation().await?;

                self.send_str("Bool(").await?;

                //self.send_str("serde_json::Value::Bool(").await?;

                self.send_string(val.to_string()).await?;

                //self.send_str(")\n\n").await?;

                self.send_str(")").await?;

            }
            serde_json::Value::Number(number) =>
            {

                //tab_indenter.send_indentation().await?;

                self.send_str("Number(").await?;

                //self.send_str("serde_json::Value::Number(").await?;

                self.send_string(number.to_string()).await?;

                //self.send_str(")\n\n").await?;

                self.send_str(")").await?;

            }
            serde_json::Value::String(string) =>
            {

                //tab_indenter.send_indentation().await?;

                self.send_str("String(\"").await?;

                //self.send_str("serde_json::Value::String(\"").await?;

                match Self::check_for_nulls(string)
                {
                    Some(new_string) => self.send_string(new_string).await?,
                    None => self.send_string_clone(string).await?
                }

                //self.send_str("\")\n\n").await?;

                self.send_str("\")").await?;

            }
            serde_json::Value::Array(vec) =>
            {

                //tab_indenter.send_indentation().await?;

                self.send_str("Vec([\n\n").await?;

                //self.send_str("serde_json::Value::Vec([\n\n").await?;

                let mut len = vec.len();

                let tab_indenter_elements = tab_indenter.next();

                for item in vec
                {

                    tab_indenter_elements.send_indentation().await?;

                    self.send_serde_json_value_enum_string_parts(item, &tab_indenter_elements).await?;

                    //self.send_2_newlines().await?;

                    len.mm();

                    if len > 0
                    {

                        self.send_str(",\n\n").await?;

                    }
                    else
                    {

                        self.send_2_newlines().await?;
                        
                    }

                }

                tab_indenter.send_indentation().await?;

                self.send_str("])").await?;   

                //self.send_str("])\n\n").await?;                        

            }
            serde_json::Value::Object(map) =>
            {

                //tab_indenter.send_indentation().await?;

                self.send_str("Object({\n\n").await?;

                //self.send_str("serde_json::Value::Object({\n\n").await?;

                let mut len = map.len();

                let tab_indenter_fields = tab_indenter.next();

                for item in map
                {

                    tab_indenter_fields.send_indentation().await?;

                    self.send_str("\"").await?;

                    self.send_string_clone(item.0).await?;

                    //self.send_str(item.0).await?;

                    //self.send_str("\":\n\n").await?;

                    self.send_str("\": ").await?;

                    //let tab_indenter = TabIndenter::new(self.io_server.output_sender_ref());

                    self.send_serde_json_value_enum_string_parts(item.1, &tab_indenter_fields).await?;

                    len.mm();

                    if len > 0
                    {

                        //tab_indenter.send_indentation().await?;

                        self.send_str(",\n\n").await?;

                    }
                    else
                    {

                        self.send_2_newlines().await?;
                        
                    }

                }

                tab_indenter.send_indentation().await?;

                self.send_str("})").await?;

            }

        }

        Ok(())

    }

    async fn process_all(&self, output_format: OutputFormat, all_or_not_supported_type: AllOrNot<SupportedType>, all_or_not_whatever: AllOrNot<Whatever>, all_or_not_type_instance: AllOrNot<TypeInstance>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //output_sender: &Sender<MapageTypeActorOutputMessage>, 
    {

        self.send_str("Process All:\n\n\n\n").await?;

        //self.send_str("SupportedType:\n\n\n\n").await?;

        self.process_all_or_not_enum_input(output_format, all_or_not_supported_type).await?;

        //self.process_all_or_not_supported_type(output_format, all_or_not_supported_type).await?; //, output_sender

        //self.send_str("Whatever:\n\n\n\n").await?;

        self.process_all_or_not_enum_input(output_format, all_or_not_whatever).await?;

        //self.process_all_or_not_whatever(output_format, all_or_not_whatever).await?; //, output_sender

        //self.send_str("TypeInstance:\n\n\n\n").await?;

        self.process_all_or_not_enum_input(output_format,  all_or_not_type_instance).await?;

        Ok(())

    }

    async fn process_all_message(&self, output_format: OutputFormat, all_or_not_supported_type: AllOrNot<SupportedType>, all_or_not_whatever: AllOrNot<Whatever>, all_or_not_type_instance: AllOrNot<TypeInstance>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //output_sender: &Sender<MapageTypeActorOutputMessage>, 
    {

        self.process_all(output_format, all_or_not_supported_type, all_or_not_whatever, all_or_not_type_instance).await?; //output_sender, 

        self.send_done().await

    }

    async fn process_all_default(&self, output_format: OutputFormat) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
    {

        self.send_str("Process All Default:\n\n\n\n").await?;

        //self.send_str("SupportedType:\n\n\n\n").await?;

        self.process_all_or_not_enum_input::<SupportedType>(output_format, AllOrNot::All).await?;

        //self.process_all_or_not_supported_type(output_format, AllOrNot::All).await?; //, output_sender

        //self.send_str("Whatever:\n\n\n\n").await?;

        self.process_all_or_not_enum_input::<Whatever>(output_format, AllOrNot::All).await?;

        //self.process_all_or_not_whatever(output_format, AllOrNot::All).await?; //, output_sender

        //self.send_str("TypeInstance:\n\n\n\n").await?;

        self.process_all_or_not_enum_input::<TypeInstance>(output_format, AllOrNot::All).await?;

        Ok(())

    }

    async fn process_all_default_message(&self, output_format: OutputFormat) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
    {

        self.process_all_default(output_format).await?; //, output_sender

        self.send_done().await

    }

    //Generic

    async fn process_input<T>(&self, output_format: OutputFormat, item_instance: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
        where T: Serialize + Clone
    {

        match output_format
        {

            OutputFormat::Json =>
            {

                let item_value_res = to_value(item_instance);

                match item_value_res
                {

                    Ok(res) =>
                    {

                        let tab_indenter = TabIndenter::new(self.io_server.output_sender_ref()); 

                        self.send_serde_json_value_enum_string_parts(&res, &tab_indenter).await?;

                        self.send_2_newlines().await?;

                        let item_string_res = to_string_pretty(&res);

                        match item_string_res
                        {
        
                            Ok(res) =>
                            {
        
                                self.send_string(res).await?;
        
                            }
                            Err(err) =>
                            {
        
                                self.send_error(err).await?;
        
                            }
        
                        }

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                        self.send_2_newlines().await?;

                        return Ok(());

                    }

                }

                self.send_4_newlines().await?;

            }

        }

        Ok(())

    }

    async fn process_enum_input<T>(&self, output_format: OutputFormat, item_instance: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
        where T: Into<&'static str> + Serialize + Clone
    {

        self.send_str(type_name_of_val(&item_instance)).await?;
        
        self.send_str(":\n\n\n\n").await?;

        //

        self.process_enum_input_variant(output_format, item_instance).await?;

        /*
        match output_format
        {

            OutputFormat::Json =>
            {

                let item_value_res = to_value(item_instance);

                match item_value_res
                {

                    Ok(res) =>
                    {

                        let tab_indenter = TabIndenter::new(self.io_server.output_sender_ref()); 

                        self.send_serde_json_value_enum_string_parts(&res, &tab_indenter).await?;

                        self.send_2_newlines().await?;

                        let item_string_res = to_string_pretty(&res);

                        match item_string_res
                        {
        
                            Ok(res) =>
                            {
        
                                self.send_string(res).await?;
        
                            }
                            Err(err) =>
                            {
        
                                self.send_error(err).await?;
        
                            }
        
                        }

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                        self.send_2_newlines().await?;

                        return Ok(());

                    }

                }

                /*
                let item_string_res = to_string_pretty(&item_instance);

                match item_string_res
                {

                    Ok(res) =>
                    {

                        self.send_string(res).await?;

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                    }

                }
                */

                self.send_4_newlines().await?;

            }

        }
        */

        Ok(())

    }

    async fn process_enum_input_variant<T>(&self, output_format: OutputFormat, item_instance: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
        where T: Into<&'static str> + Serialize + Clone
    {

        self.send_str(item_instance.clone().into()).await?;

        self.send_2_newlines().await?;

        self.process_input(output_format, item_instance).await?;

        Ok(())

    }

    async fn process_struct_input<T>(&self, output_format: OutputFormat,item_instance: T) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
        where T: Serialize + Clone
    {

        self.send_str(type_name_of_val(&item_instance)).await?;
        
        self.send_str(":\n\n\n\n").await?;

        self.process_input(output_format, item_instance).await?;

        Ok(())

    }

    //Process All Supported Types or just one.

    async fn process_all_or_not_enum_input<T>(&self, output_format: OutputFormat, all_or_not_input: AllOrNot<T>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Into<&'static str> + Serialize + Clone + IntoEnumIterator
    {

        self.send_str(type_name_of_val(&all_or_not_input)).await?;
        
        self.send_str(":\n\n\n\n").await?;

        match all_or_not_input
        {

            AllOrNot::All =>
            {

                self.send_str("AllOrNot::All\n\n\n\n").await?;

                for item in T::iter()
                {

                    self.process_enum_input_variant(output_format, item).await?;
            
                }

            }
            AllOrNot::NotAll(all_or_not_input_variant) =>
            {

                self.send_str("AllOrNot::NotAll\n\n\n\n").await?;

                self.process_enum_input_variant(output_format, all_or_not_input_variant).await?;

            }

        }

        Ok(())

    }

    async fn process_all_or_not_enum_input_message<T>(&self, output_format: OutputFormat, all_or_not_input: AllOrNot<T>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where T: Into<&'static str> + Serialize + Clone + IntoEnumIterator
    {

        self.send_value_enum_heading(output_format).await?;

        self.process_all_or_not_enum_input(output_format, all_or_not_input).await?;

        self.send_done().await

    }

    async fn process_command_message(&self, output_format: OutputFormat, command: Command) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_value_enum_heading(output_format).await?;

        self.process_struct_input(output_format,command).await?;
        
        self.send_done().await

    }

    async fn process_command_result_message(&self, output_format: OutputFormat, command_result: CommandResult) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_value_enum_heading(output_format).await?;

        self.process_struct_input(output_format,command_result).await?;
        
        self.send_done().await

    }

    async fn process_command_error_message(&self, output_format: OutputFormat, command_error: CommandError) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_value_enum_heading(output_format).await?;

        self.process_struct_input(output_format,command_error).await?;
        
        self.send_done().await

    }

    async fn process_streamed_message_message(&self, output_format: OutputFormat, streamed_message: StreamedMessage) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_value_enum_heading(output_format).await?;

        self.process_enum_input(output_format,streamed_message).await?;
        
        self.send_done().await

    }

    /*

    //SupportedType

    //Process and output a single SupportedType.

    async fn process_supported_type(&self, output_format: OutputFormat, supported_type: SupportedType) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
    {

        self.send_str(supported_type.into()).await?;

        //let str_value = SendableText::Str(supported_type.into());

        //self.send_sendable_text(str_value).await?;

        self.send_2_newlines().await?;

        match output_format
        {

            OutputFormat::Json =>
            {

                //self.send_str("serde_json::Value::...\n\n").await?;

                let item_value_res = to_value(supported_type);

                match item_value_res
                {

                    Ok(res) =>
                    {

                        /*
                        cannot borrow `*self` as mutable because it is also borrowed as immutable
                        mutable borrow occurs hererustcClick for full compiler diagnostic
                        */

                        //let output_sender_ref = self.io_server.output_sender_ref();

                        //self.send_str("serde_json::Value::...").await?;

                        //self.send_serde_json_value_heading().await?;

                        //let tab_indenter = TabIndenter::new(output_sender); //self.io_server.output_sender_ref()); //output_sender_ref);

                        let tab_indenter = TabIndenter::new(self.io_server.output_sender_ref()); 

                        self.send_serde_json_value_enum_string_parts(res, &tab_indenter).await?;

                        self.send_2_newlines().await?;

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                        //self.send_string(err.to_string()).await?;

                        //let err_string = SendableText::String(err.to_string());

                        //self.send_sendable_text(err_string).await?;

                        self.send_2_newlines().await?;

                        return Ok(());

                    }

                }

                let item_string_res = to_string_pretty(&supported_type);

                match item_string_res
                {

                    Ok(res) =>
                    {

                        self.send_string(res).await?;

                        //let res_string = SendableText::String(res);

                        //self.send_sendable_text(res_string).await?;

                        //self.send_4_newlines().await?;

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                        //self.send_string(err.to_string()).await?;

                        //let err_string = SendableText::String(err.to_string());

                        //self.send_sendable_text(err_string).await?;

                        //self.send_4_newlines().await?;

                    }

                }

                self.send_4_newlines().await?;

            }

        }

        Ok(())

    }

    //Process All Supported Types or just one.

    async fn process_all_or_not_supported_type(&self, output_format: OutputFormat, all_or_not_supported_type: AllOrNot<SupportedType>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage> //bool
    {

        self.send_value_enum_heading(output_format).await?;

        match all_or_not_supported_type
        {

            AllOrNot::All =>
            {

                for item in SupportedType::iter()
                {

                    self.process_supported_type(output_format, item).await?; //, output_sender
            
                }

                //Signal that the operation is complete

                //self.send_done().await?;

            }
            AllOrNot::NotAll(supported_type) =>
            {

                self.process_supported_type(output_format, supported_type).await?; //, output_sender

                //self.send_done().await?;

            }

        }

        Ok(())

    }

    async fn process_all_or_not_supported_type_message(&self, output_format: OutputFormat, all_or_not_supported_type: AllOrNot<SupportedType>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage> //bool
    {

        self.process_all_or_not_supported_type(output_format, all_or_not_supported_type).await?; //, output_sender

        self.send_done().await

    }

    //Whatever

    async fn process_all_or_not_whatever(&self, output_format: OutputFormat, all_or_not_whatever: AllOrNot<Whatever>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
    {

        self.send_value_enum_heading(output_format).await?;

        match all_or_not_whatever
        {

            AllOrNot::All =>
            {

                for item in Whatever::iter()
                {

                    self.process_whatever(output_format, item).await?; //, output_sender
            
                }

                //Signal that the operation is complete

                //self.send_done().await?;

            }
            AllOrNot::NotAll(whatever) =>
            {

                self.process_whatever(output_format, whatever).await?; //, output_sender

                //self.send_done().await?;

            }

        }

        Ok(())

    }

    async fn process_all_or_not_whatever_message(&self, output_format: OutputFormat, all_or_not_whatever: AllOrNot<Whatever>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
    {

        self.process_all_or_not_whatever(output_format, all_or_not_whatever).await?; //, output_sender

        self.send_done().await

    }

    async fn process_whatever(&self, output_format: OutputFormat, whatever: Whatever) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
    {

        self.send_str(whatever.clone().into()).await?;

        self.send_2_newlines().await?;

        match output_format
        {

            OutputFormat::Json =>
            {

                let item_value_res = to_value(whatever.clone());

                match item_value_res
                {

                    Ok(res) =>
                    {

                        //self.send_serde_json_value_heading().await?;

                        //self.send_str("serde_json::Value::...\n\n").await?;

                        //self.send_str("serde_json::Value::...").await?;

                        //let tab_indenter = TabIndenter::new(output_sender); //self.io_server.output_sender_ref());

                        let tab_indenter = TabIndenter::new(self.io_server.output_sender_ref()); 

                        self.send_serde_json_value_enum_string_parts(res, &tab_indenter).await?;

                        self.send_2_newlines().await?;

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                        self.send_2_newlines().await?;

                        return Ok(());

                    }

                }

                let item_string_res = to_string_pretty(&whatever);

                match item_string_res
                {

                    Ok(res) =>
                    {

                        self.send_string(res).await?;

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                    }

                }

                self.send_4_newlines().await?;

            }

        }

        Ok(())

    }

    //TypeInstance

    async fn process_all_or_not_type_instance(&self, output_format: OutputFormat, all_or_not_type_instance: AllOrNot<TypeInstance>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //, output_sender: &Sender<MapageTypeActorOutputMessage>
    {

        self.send_value_enum_heading(output_format).await?;

        match all_or_not_type_instance
        {

            AllOrNot::All =>
            {

                for item in TypeInstance::iter()
                {

                    self.process_type_instance(output_format, item).await?;
            
                }

            }
            AllOrNot::NotAll(type_instance) =>
            {

                self.process_type_instance(output_format, type_instance).await?;

            }

        }

        Ok(())

    }

    async fn process_all_or_not_type_instance_message(&self, output_format: OutputFormat, all_or_not_type_instance: AllOrNot<TypeInstance>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.process_all_or_not_type_instance(output_format, all_or_not_type_instance).await?;

        self.send_done().await

    }

    async fn process_type_instance(&self, output_format: OutputFormat, type_instance: TypeInstance) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_str(type_instance.clone().into()).await?;

        self.send_2_newlines().await?;

        match output_format
        {

            OutputFormat::Json =>
            {

                let item_value_res = to_value(type_instance.clone());

                match item_value_res
                {

                    Ok(res) =>
                    {

                        let tab_indenter = TabIndenter::new(self.io_server.output_sender_ref()); 

                        self.send_serde_json_value_enum_string_parts(res, &tab_indenter).await?;

                        self.send_2_newlines().await?;

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                        self.send_2_newlines().await?;

                        return Ok(());

                    }

                }

                let item_string_res = to_string_pretty(&type_instance);

                match item_string_res
                {

                    Ok(res) =>
                    {

                        self.send_string(res).await?;

                    }
                    Err(err) =>
                    {

                        self.send_error(err).await?;

                    }

                }

                self.send_4_newlines().await?;

            }

        }

        Ok(())

    }
    */

}

impl_mac_task_actor!(MapageTypeActor);

//

fn print_display<T>(debug_printable: T)
    where T: Display
{

    let err_string = format!("{debug_printable}:?");

    print!("{}", err_string);

}

//Minimal implementation:

/*

use act_rs::{impl_default_start_and_end_async, impl_mac_task_actor};

use paste::paste;

use act_rs::{impl_default_end_async, impl_default_start_async}; //Remove impl_default_start_and_end_async macro dependancies.

use tokio::task::JoinHandle;

pub struct MapageTypeActorState
{



}

impl MapageTypeActorState
{

    pub fn new()
    {

    }

    impl_default_start_and_end_async!();

    async fn run_async(&mut self) -> bool
    {

        false

    }

}

impl_mac_task_actor!(MapageTypeActor);

*/
