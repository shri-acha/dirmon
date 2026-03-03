use crate::notify::{Error, Event, EventHandler, Result};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver,Sender};

pub struct DirmonChannel {
    pub Rx: Receiver<notify::Result<notify::Event>>,
    pub Tx: Sender<notify::Result<notify::Event>>,
}

impl DirmonChannel {
    pub fn channel() -> Self {
        let (tx, rx) = channel::<notify::Result<notify::Event>>();
        Self { Rx: rx, Tx: tx }
    }
}
