pub mod config;
pub mod extensions;
pub mod files;
use crate::{Directory, File};
use log::{debug, error};
use std::collections::BTreeMap;

pub fn match_response(
    file_dir_map: &BTreeMap<String, Vec<String>>,
    res: &notify::Result<notify::Event>,
) -> notify::Result<()> {
    if let Ok(event) = res {
        if let notify::event::EventKind::Create(_) = &event.kind {
            // Create event occurs
            // for every move and
            let event_monitoring_directory_list: Vec<Directory> = event
                .paths
                .iter()
                .filter_map(|e| match e.parent() {
                    Some(parent_path) => Some(parent_path.display().to_string()),
                    None => {
                        error!("Path {:?} has no parent directory, skipping.", e);
                        None
                    }
                })
                .map(|e| Directory::from(e, vec![]))
                .collect();

            for mut event_monitoring_directory in event_monitoring_directory_list {
                let files_list: Vec<Box<File>> =
                    files::get_files(&event_monitoring_directory).unwrap_or(vec![]);
                if files_list.is_empty() {
                    continue;
                } else {
                    match files::check_and_write_dir(
                        file_dir_map,
                        &event_monitoring_directory,
                        &files_list,
                    ) {
                        Ok(_) => {
                            debug!("directory modified!");
                        }
                        Err(e) => {
                            error!("error modifying directory!: {}", e);
                            error!(
                                "[STATE]:\t{:?}{:?}{:?}",
                                file_dir_map, &event_monitoring_directory, &files_list,
                            );
                        }
                    }

                    if let Some(m) =
                        files::move_files(file_dir_map, &event_monitoring_directory, &files_list)
                    {
                        debug!("{}", m);
                    } else {
                        error!("error moving files!");
                    }
                    debug!("Event:{:?}", event.paths);
                }

                event_monitoring_directory.d_files = files_list;
            }
        }
    }
    Ok(())
}
