use super::channel::DirmonChannel;
use crate::components::Directory;
use crate::notify::{self,RecursiveMode};
use log::info;
use anyhow::Result;
use std::time::Duration;
use notify::Watcher;

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
    fn watch(self,directory: Directory, watch_mode: DirmonWatchMode) -> Result<()>;
}

struct DirmonWatcher{
    channel: DirmonChannel,
    config: DirmonWatcherConfig,
}

#[derive(Default)]
pub struct DirmonWatcherConfig {
    config: notify::Config,
}

impl DirmonWatcher{
    pub fn from(channel: DirmonChannel, config: DirmonWatcherConfig) -> Self {
        Self { channel, config }
    }
}

impl Watchable for DirmonWatcher {
    fn watch(mut self,directory: Directory, watch_mode: DirmonWatchMode) -> Result<()> {

        let notify_watch_mode = match watch_mode {
            DirmonWatchMode::Recursive => RecursiveMode::Recursive,
            DirmonWatchMode::NonRecursive => RecursiveMode::NonRecursive,
        };

        let poll_delay: Duration = Duration::from_secs(POLL_DELAY_SECS);

        let mut notify_watcher = notify::PollWatcher::new(
            self.channel.Tx,
            notify::Config::default().with_poll_interval(poll_delay), //WIP 
        )?;

        info!("listening on {:?}", directory);
        Ok(notify_watcher.watch(&directory.d_path, notify_watch_mode)?)
    }
}
