use std::error::Error;

use corlib::{text::SendableText, WorkInProgressResult};

use libsync::{crossbeam::mpmc::tokio::array_queue::Sender, BoundedSendError};

use crate::actors::MapageTypeActorOutputMessage;

pub struct ClientOutputter
{

    sender: Sender<MapageTypeActorOutputMessage>

}

impl ClientOutputter
{

    pub fn new(sender: Sender<MapageTypeActorOutputMessage>) -> Self
    {

        Self
        {
            
            sender
        
        }

    }

    pub async fn send_sendable_text(&self, sendable_text: SendableText) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.sender.send(MapageTypeActorOutputMessage::WorkInProgressTextResult(WorkInProgressResult::not_done(sendable_text))).await

    }

    pub async fn send_str(&self, sendable_text: &'static str) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str(sendable_text)).await

    }

    pub async fn send_string_clone(&self, sendable_text: &String) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::String(sendable_text.clone())).await

    }

    pub async fn send_string(&self, sendable_text: String) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::String(sendable_text)).await

    }

    pub async fn send_2_newlines(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str("\n\n")).await

    }

    pub async fn send_4_newlines(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.send_sendable_text(SendableText::Str("\n\n\n\n")).await

    }

    pub async fn send_error<E>(&self, error: E) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
        where E: Error + ToString
    {

        self.send_string(error.to_string()).await

    }

    pub async fn send_done(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.sender.send(MapageTypeActorOutputMessage::WorkInProgressTextResult(WorkInProgressResult::done_none())).await

    }
    
}


