use super::channel::DirmonChannel;
use crate::components::Directory;
use crate::notify::{self, RecursiveMode};
use anyhow::Result;
use log::info;
use notify::Watcher;
use std::time::Duration;
use std::sync::mpsc::{Sender};

// notify::PollWatcher::new(dirmon_channel.Tx,
//          notify::Config::default()
//          .with_poll_interval(poll_delay)
//          )?;

const POLL_DELAY_SECS: u64 = 1;

pub enum DirmonWatchMode {
    Recursive,
    NonRecursive,
}

pub trait Watchable {
    fn watch(&self, directory: &Directory, watch_mode: DirmonWatchMode) -> Result<()>;
}

pub struct DirmonWatcher {
    Tx: Sender<notify::Result<notify::Event>>,
    config: DirmonWatcherConfig,
}

#[derive(Default)]
pub struct DirmonWatcherConfig {
    config: notify::Config,
}

impl DirmonWatcher {
    pub fn from(Tx: Sender<notify::Result<notify::Event>>, config: DirmonWatcherConfig) -> Self {
        Self { Tx, config }
    }
}

impl Watchable for DirmonWatcher {
    fn watch(&self, directory: &Directory, watch_mode: DirmonWatchMode) -> Result<()> {
        let notify_watch_mode = match watch_mode {
            DirmonWatchMode::Recursive => RecursiveMode::Recursive,
            DirmonWatchMode::NonRecursive => RecursiveMode::NonRecursive,
        };

        let poll_delay: Duration = Duration::from_secs(POLL_DELAY_SECS);

        let mut notify_watcher = notify::PollWatcher::new(
            self.Tx.clone(),
            notify::Config::default().with_poll_interval(poll_delay), //WIP
        )?;

        info!("listening on {:?}", directory);
        Ok(notify_watcher.watch(&directory.d_path, notify_watch_mode)?)
    }
}
