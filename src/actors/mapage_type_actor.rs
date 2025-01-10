use std::{error::Error, fmt::{Debug, Display}};

use act_rs::{impl_default_on_start_and_end_async, impl_mac_task_actor};

use corlib::{inc_dec::IncDecSelf, text::SendableText, WorkInProgressResult};
use paste::paste;

//use act_rs::{impl_default_end_async, impl_default_start_async}; //Remove impl_default_start_and_end_async macro dependancies.

use serde::Serialize;
use serde_json::{to_string, to_string_pretty, to_value, Value};
use strum::IntoEnumIterator;
use tokio::task::JoinHandle;

use libsync::{crossbeam::mpmc::tokio::array_queue::io_channels::{io_channels, IOClient, IOServer}, BoundedSendError};

use crate::{widgets::{MapageType, OutputFormat}, AllOrNot, SupportedType, Whatever};

use async_recursion::async_recursion;

#[derive(Debug)]
pub enum MapageTypeActorInputMessage
{

    ProcessSupportedType(OutputFormat, AllOrNot<SupportedType>),
    ProcessWhatever(OutputFormat, AllOrNot<Whatever>)

}

impl Display for MapageTypeActorInputMessage
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            MapageTypeActorInputMessage::ProcessSupportedType(output_format, all_or_not_supported_type) =>
            {
                
                write!(f, "ProcessSupportedType({output_format:?}, {all_or_not_supported_type:?})")

            }
            MapageTypeActorInputMessage::ProcessWhatever(output_format, all_or_not_supported_type) =>
            {

                write!(f, "ProcessProcessWhatever({output_format:?}, {all_or_not_supported_type:?})")

            }

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

    io_server: IOServer<MapageTypeActorInputMessage, MapageTypeActorOutputMessage>

}

impl MapageTypeActorState
{

    pub fn new() -> (IOClient<MapageTypeActorInputMessage, MapageTypeActorOutputMessage>, Self)
    {

        let (io_client, io_server) = io_channels(2, 100);

        (io_client, Self
        {

            io_server

        })

    }

    pub fn spawn() -> IOClient<MapageTypeActorInputMessage, MapageTypeActorOutputMessage>
    {

        let (io_client, io_server) = MapageTypeActorState::new();

        MapageTypeActor::spawn(io_server);

        io_client

    }

    impl_default_on_start_and_end_async!();

    async fn run_async(&mut self) -> bool
    {

        let processing_res;

        if let Some(message) = self.io_server.input_receiver_ref().recv().await
        {

            match message
            {

                MapageTypeActorInputMessage::ProcessSupportedType(output_format, all_or_not_supported_type) =>
                {

                    processing_res = self.process_all_or_not_supported_type(output_format, all_or_not_supported_type).await;

                }
                MapageTypeActorInputMessage::ProcessWhatever(output_format, all_or_not_whatever) =>
                {

                    processing_res = self.process_all_or_not_whatever(output_format, all_or_not_whatever).await;

                }

            }

        }
        else
        {

            return true;
            
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

    async fn send_sendable_text(&mut self, sendable_text: SendableText) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.io_server.output_sender_ref().send(MapageTypeActorOutputMessage::WorkInProgressTextResult(WorkInProgressResult::not_done(sendable_text))).await

    }

    async fn send_str(&mut self, sendable_text: &'static str) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str(sendable_text)).await

    }

    async fn send_string(&mut self, sendable_text: String) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::String(sendable_text)).await

    }

    async fn send_2_newlines(&mut self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str("\n\n")).await

    }

    async fn send_4_newlines(&mut self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str("\n\n\n\n")).await

    }

    async fn send_error<E>(&mut self, error: E) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
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

    async fn send_done(&mut self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
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

    //The GTK TextView Buffer doesn't like it when you try to append null characters to it.

    fn check_for_nulls(string: &mut String)
    {

        if string.contains('\0')
        {

            *string = string.replace('\0', "\\0");

        }

    }

    #[async_recursion]
    async fn send_serde_json_value_enum_string_parts(&mut self, value: Value) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        match value
        {

            serde_json::Value::Null =>
            {

                self.send_str("serde_json::Value::Null").await?;

            },
            serde_json::Value::Bool(val) =>
            {

                self.send_str("serde_json::Value::Bool(").await?;

                self.send_string(val.to_string()).await?;

                self.send_str(")\n\n").await?;

            }
            serde_json::Value::Number(number) =>
            {

                self.send_str("serde_json::Value::Number(").await?;

                self.send_string(number.to_string()).await?;

                self.send_str(")\n\n").await?;

            }
            serde_json::Value::String(mut string) =>
            {

                self.send_str("serde_json::Value::String(\"").await?;

                Self::check_for_nulls(&mut string);

                self.send_string(string).await?;

                self.send_str("\")\n\n").await?;

            }
            serde_json::Value::Array(vec) =>
            {

                self.send_str("serde_json::Value::Vec([\n\n").await?;

                let mut len = vec.len();

                for item in vec
                {

                    self.send_serde_json_value_enum_string_parts(item).await?;

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

                self.send_str("])\n\n").await?;                        

            }
            serde_json::Value::Object(map) =>
            {

                self.send_str("serde_json::Value::Object({\n\n").await?;

                let mut len = map.len();

                for item in map
                {

                    self.send_string(item.0).await?;

                    self.send_str(":\n\n").await?;

                    self.send_serde_json_value_enum_string_parts(item.1).await?;

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

                self.send_str("})\n\n").await?;

            }

        }

        Ok(())

    }

    //SupportedType

    //Process and output a single SupportedType.

    async fn process_supported_type(&mut self, output_format: OutputFormat, supported_type: SupportedType) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_str(supported_type.into()).await?;

        //let str_value = SendableText::Str(supported_type.into());

        //self.send_sendable_text(str_value).await?;

        self.send_2_newlines().await?;

        match output_format
        {

            OutputFormat::Json =>
            {

                let item_value_res = to_value(supported_type);

                match item_value_res
                {

                    Ok(res) =>
                    {

                        self.send_serde_json_value_enum_string_parts(res).await?;

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

    async fn process_all_or_not_supported_type(&mut self, output_format: OutputFormat, all_or_not_supported_type: AllOrNot<SupportedType>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>> //bool
    {

        match all_or_not_supported_type
        {

            AllOrNot::All =>
            {

                for item in SupportedType::iter()
                {

                    self.process_supported_type(output_format, item).await?;
            
                }

                //Signal that the operation is complete

                self.send_done().await?;

            }
            AllOrNot::NotAll(supported_type) =>
            {

                self.process_supported_type(output_format, supported_type).await?;

                self.send_done().await?;

            }

        }

        Ok(())

    }

    //Whatever

    async fn process_all_or_not_whatever(&mut self, output_format: OutputFormat, all_or_not_whatever: AllOrNot<Whatever>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        match all_or_not_whatever
        {

            AllOrNot::All =>
            {

                for item in Whatever::iter()
                {

                    self.process_whatever(output_format, item).await?;
            
                }

                //Signal that the operation is complete

                self.send_done().await?;

            }
            AllOrNot::NotAll(whatever) =>
            {

                self.process_whatever(output_format, whatever).await?;

                self.send_done().await?;

            }

        }

        Ok(())

    }

    async fn process_whatever(&mut self, output_format: OutputFormat, whatever: Whatever) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
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

                        self.send_serde_json_value_enum_string_parts(res).await?;

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
