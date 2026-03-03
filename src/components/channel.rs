use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver,Sender};

pub struct DirmonChannel {
    pub rx: Receiver<notify::Result<notify::Event>>,
    pub tx: Sender<notify::Result<notify::Event>>,
}

impl DirmonChannel {
    pub fn channel() -> Self {
        let (tx, rx) = channel::<notify::Result<notify::Event>>();
        Self { rx, tx }
    }
}
