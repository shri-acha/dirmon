// core
mod components;
mod helpers;
use crate::components::{
    Directory, File,
    channel::DirmonChannel,
    reactor::DirmonReactor,
    watcher::{DirmonWatchMode, DirmonWatcher, DirmonWatcherConfig, Watchable},
};
use helpers::*;
use log::{debug, error};
use notify::{self};
use std::fmt;
use std::time::Duration;

const POLL_DELAY_SECS: u64 = 1;
const CONFIG_FILE: &'static str = ".dirmon.conf";

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let dirmon_channel = DirmonChannel::channel();
    let DirmonChannel { tx, rx } = dirmon_channel;

    let Some((mut monitoring_dir_list, file_dir_map_list, file_dir_map)) = load_config(CONFIG_FILE)
    else {
        panic!("Failure to load config file!");
    };

    let (supported_extensions, supported_types) = get_spprtd_extns_and_type(&file_dir_map);

    // info!(
    //     "supported_types: {:?}\nsupported_extensions: {:?}",
    //     supported_types, supported_extensions
    // );
    // debug!("file_dir_map_list: {:?}", file_dir_map_list);

    let poll_delay: Duration = Duration::from_secs(POLL_DELAY_SECS);

    let watcher = DirmonWatcher::from(
        tx,
        DirmonWatcherConfig::from(notify::Config::default().with_poll_interval(poll_delay)),
    );

    let mut watcher_handles = vec![];

    // Watcher instance creator
    // spins a new watcher thread for each monitoring directory
    for monitoring_dir in monitoring_dir_list.iter_mut() {
        // runs for the start
        //
        // initialization
        let files_list: Vec<Box<File>> = get_files(monitoring_dir).unwrap_or(vec![]);
        monitoring_dir.d_files = files_list.clone();
        debug!("Monitoring Directory: {:?}", monitoring_dir);
        debug!("Files list: {:?}", files_list);
        let _ = check_and_write_dir(
            &file_dir_map,
            monitoring_dir,
            &files_list,
            &supported_extensions,
        );
        let _ = move_files(&file_dir_map, monitoring_dir, &files_list);

        if let Ok(handle) = watcher.watch(monitoring_dir, DirmonWatchMode::NonRecursive) {
            watcher_handles.push(handle);
        }
    }

    let reactor = DirmonReactor::from(rx);
    reactor.blocking_react(file_dir_map_list, supported_extensions);
    Ok(())
}
