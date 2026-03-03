// core
mod components;
mod helpers;
use crate::components::{
    Directory, File,
    channel::DirmonChannel,
    watcher::{DirmonWatchMode, DirmonWatcher, DirmonWatcherConfig, Watchable},
    reactor::{DirmonReactor},
};
use helpers::*;
use log::{error, info};
use notify::{self};
use std::collections::{BTreeMap, HashMap};
use std::time::Duration;
use std::{fmt};

const CONFIG_FILE: &'static str = ".dirmon.conf";

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let dirmon_channel = DirmonChannel::channel();
    let DirmonChannel{ tx,rx } = dirmon_channel;
    let mut monitoring_dir: Directory = Directory::default();
    let Some((monitoring_dir_list,file_dir_map_list,file_dir_map)) = load_config(CONFIG_FILE) else{todo!();};

    let (supported_extensions, supported_types) = get_spprtd_extns_and_type(&file_dir_map);

    info!(
        "supported_types: {:?}\nsupported_extensions: {:?}",
        supported_types, supported_extensions
    );
    info!("file_dir_map_list: {:?}", file_dir_map_list);

    const POLL_DELAY_SECS: u64 = 1;
    let poll_delay: Duration = Duration::from_secs(POLL_DELAY_SECS);

    let watcher = DirmonWatcher::from(tx, DirmonWatcherConfig::from(notify::Config::default().with_poll_interval(poll_delay)));

    // Watcher instance creator
    // spins a new watcher thread for each monitoring directory
    for monitoring_dir in monitoring_dir_list {
        info!("listening on {:?}", monitoring_dir);

        // runs for the start
        //
        // initialization
        let files_list: Vec<Box<File>> = get_files(&monitoring_dir).unwrap_or(vec![]);

        let _ = check_and_write_dir(
            &file_dir_map,
            &monitoring_dir,
            &files_list,
            &supported_extensions,
        );

        let _ = move_files(&file_dir_map, &monitoring_dir, &files_list);
        let _ = watcher.watch(&monitoring_dir, DirmonWatchMode::NonRecursive);
    }

    let reactor = DirmonReactor::from(rx);
    reactor.blocking_react(file_dir_map_list,supported_extensions);
    Ok(())
}
