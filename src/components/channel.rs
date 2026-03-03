use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use crate::notify::{Event,Error,EventHandler,Result};

pub struct DirmonChannel{
    pub Rx: Receiver<notify::Result<notify::Event>>,
    pub Tx: Sender<notify::Result<notify::Event>>,
}

impl DirmonChannel {
    pub fn channel() -> Self {
        let (tx, rx) = channel::<notify::Result<notify::Event>>();
        Self { Rx: rx, Tx: tx }
    }
}
