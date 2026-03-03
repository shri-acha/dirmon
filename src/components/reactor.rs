use crate::helpers::{match_response};
use std::sync::mpsc::Receiver;
use crate::{error,Directory};
use std::collections::{HashMap,BTreeMap,HashSet};

pub struct DirmonReactor{
    rx: Receiver<notify::Result<notify::Event>>,
}

impl DirmonReactor{
    pub fn from(rx: Receiver<notify::Result<notify::Event>>,) ->Self{
        Self{
            rx,
        }
    }
}

impl DirmonReactor {
    pub fn blocking_react(self,file_dir_map_list:HashMap<Directory, BTreeMap<String, Vec<String>>>,supported_extensions: HashSet<String>){

        for res in self.rx {
            match &res {
                Ok(event) => {

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

                    for event_monitoring_directory in event_monitoring_directory_list {
                        if let Some(file_dir_map) = file_dir_map_list.get(&event_monitoring_directory) {
                            // ????
                            let _ = match_response(file_dir_map, &supported_extensions, &res); // have guards before hand, so shouldn't crash
                        }
                    }
                }
                Err(e) => {
                    error!("{}",e);
                    todo!();
                }
            }
        }
    }
}
