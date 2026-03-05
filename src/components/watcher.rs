use crate::components::Directory;
use crate::notify::{self, RecursiveMode};
use anyhow::Result;
use log::info;
use notify::Watcher;
use std::sync::mpsc::Sender;

// notify::PollWatcher::new(dirmon_channel.Tx,
//          notify::Config::default()
//          .with_poll_interval(poll_delay)
//          )?;

#[allow(dead_code)]
pub enum DirmonWatchMode {
    Recursive,
    NonRecursive,
}

pub trait Watchable {
    fn watch(&self, directory: &Directory, watch_mode: DirmonWatchMode) -> Result<impl Watcher>;
}

pub struct DirmonWatcher {
    tx: Sender<notify::Result<notify::Event>>,
    config: DirmonWatcherConfig,
}

#[derive(Default)]
pub struct DirmonWatcherConfig {
    inner: notify::Config,
}

impl DirmonWatcherConfig {
    pub fn from(config: notify::Config) -> Self {
        Self { inner: config }
    }
}

impl DirmonWatcher {
    pub fn from(tx: Sender<notify::Result<notify::Event>>, config: DirmonWatcherConfig) -> Self {
        Self { tx, config }
    }
}

impl Watchable for DirmonWatcher {
    fn watch(&self, directory: &Directory, watch_mode: DirmonWatchMode) -> anyhow::Result<impl Watcher> {
        let notify_watch_mode = match watch_mode {
            DirmonWatchMode::Recursive => RecursiveMode::Recursive,
            DirmonWatchMode::NonRecursive => RecursiveMode::NonRecursive,
        };

        let mut notify_watcher = notify::PollWatcher::new(
            self.tx.clone(),
            self.config.inner,
        )?;

        info!("listening on {:?}", directory);
        notify_watcher.watch(&directory.d_path, notify_watch_mode)?;
        Ok(notify_watcher)
    }
}
