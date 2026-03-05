// core
mod components;
mod helpers;
use crate::components::{
    Directory, File,
    channel::DirmonChannel,
    reactor::DirmonReactor,
    watcher::{DirmonWatchMode, DirmonWatcher, DirmonWatcherConfig, Watchable},
};
use log::{debug, error, info};
use notify::{self};
use std::fmt;
use std::time::Duration;

const POLL_DELAY_SECS: u64 = 1;
const CONFIG_FILE: &'static str = ".dirmon.conf";

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let dirmon_channel = DirmonChannel::channel();
    let DirmonChannel { tx, rx } = dirmon_channel;

    let Some((mut monitoring_dir_list, file_dir_map_list, file_dir_map)) =
        helpers::config::load_config(CONFIG_FILE)
    else {
        panic!("Failure to load config file!");
    };
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
        let Some((supported_extensions, supported_types)) =
            helpers::extensions::get_supported_extension_and_type(
                &monitoring_dir,
                &file_dir_map_list,
            )
        else {
            panic!("missing supported mapping for monitoring directory!");
        };

        info!("monitoring directory:{}", monitoring_dir.d_path.display());
        info!("supported_types: {:?}", supported_types);
        info!("supported_extensions: {:?}", supported_extensions);

        // runs for the start
        // initialization
        let files_list: Vec<Box<File>> =
            helpers::files::get_files(monitoring_dir).unwrap_or(vec![]);
        monitoring_dir.d_files = files_list.clone();
        // debug!("Files list: {:?}", files_list);
        let _ = helpers::files::check_and_write_dir(&file_dir_map, monitoring_dir, &files_list);
        let _ = helpers::files::move_files(&file_dir_map, monitoring_dir, &files_list);

        if let Ok(handle) = watcher.watch(monitoring_dir, DirmonWatchMode::NonRecursive) {
            watcher_handles.push(handle);
        }
    }

    let reactor = DirmonReactor::from(rx);
    reactor.blocking_react(file_dir_map_list);
    Ok(())
}
