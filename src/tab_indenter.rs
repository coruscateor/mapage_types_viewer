use corlib::{inc_dec::IncDecSelf, WorkInProgressResult};
use libsync::{crossbeam::mpmc::tokio::array_queue::Sender, BoundedSendError};

use crate::actors::MapageTypeActorOutputMessage;

/*
#[derive(Clone)]
pub struct TabIndenter
{

    sender: Sender<MapageTypeActorOutputMessage>,
    level: u32

}

impl TabIndenter
{

    pub fn new(sender: &Sender<MapageTypeActorOutputMessage>) -> Self
    {

        Self
        {

            sender: sender.clone(),
            level: 0

        }

    }

    pub fn next(&self) -> TabIndenter
    {

        Self
        {

            sender: self.sender.clone(),
            level: self.level + 1

        }

    }

    pub fn level(&self) -> u32
    {

        self.level

    }

    pub async fn send_indentation(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        let mut level = self.level;

        while level > 0
        {

            self.sender.send(MapageTypeActorOutputMessage::WorkInProgressTextResult(WorkInProgressResult::not_done("\t".into()))).await?;

            level.mm();

        }

        Ok(())

    }

}
*/

/*
cannot borrow `*self` as mutable because it is also borrowed as immutable
mutable borrow occurs hererustcClick for full compiler diagnostic
*/

#[derive(Clone)]
pub struct TabIndenter<'a>
{

    sender: &'a Sender<MapageTypeActorOutputMessage>,
    level: u32

}

impl<'a> TabIndenter<'a>
{

    pub fn new(sender: &'a Sender<MapageTypeActorOutputMessage>) -> Self
    {

        Self
        {

            sender, //: sender.clone(),
            level: 0

        }

    }

    pub fn next(&self) -> TabIndenter
    {

        Self
        {

            sender: self.sender,
            level: self.level + 1

        }

    }

    pub fn level(&self) -> u32
    {

        self.level

    }

    pub async fn send_indentation(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        let mut level = self.level;

        while level > 0
        {

            self.sender.send(MapageTypeActorOutputMessage::WorkInProgressTextResult(WorkInProgressResult::not_done("\t".into()))).await?;

            level.mm();

        }

        Ok(())

    }

}

