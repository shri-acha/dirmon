// core
mod helpers;
mod components;
use helpers::*;
use components::*;
use std::collections::{BTreeMap,HashSet,HashMap};
use std::{path::{self},fmt::{self,write},io::{self},fs};
use std::time::Duration;
use fs_extra::file;
use notify::{self,Watcher};
use std::sync::mpsc;
use std::thread::{self};
use std::sync::Arc;
use configparser::ini::Ini;
use log::{debug, error, info};
use std::io::Error;


const CONFIG_FILE : &'static str= ".dirmon.conf";
const POLL_DELAY_SECS : u64 = 1;

fn main()->notify::Result<()>{

        env_logger::init();

        let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
        let mut config_raw  = Ini::new_cs(); 
        
        let mut monitoring_dir_list : Vec<Directory> = vec![];
        let mut file_dir_map_list : HashMap<Directory,BTreeMap<String,Vec<String>>> = HashMap::new();
        
        let mut monitoring_dir: Directory = Directory::default();

        let mut file_dir_map : BTreeMap<String,Vec<String>> = BTreeMap::new();

        // loading config with error guards 
        if let Ok(config_loaded) = config_raw.load(CONFIG_FILE){
        // loaded config works by parsing keys and option<value>
            for (monitoring_dir_buf,file_dir_map_buf) in config_loaded {
                monitoring_dir = Directory::from(monitoring_dir_buf.clone(),vec![]);
                for (type_value,extns) in file_dir_map_buf {

                    if let Some(extns) = extns{

                        // println!("{:?}",extns);
                        file_dir_map.insert(type_value,extns.split(',').map(|e| e.to_string()).collect());
                    }else {

                        info!("missing values for {:?}",type_value);
                    }

                }
                monitoring_dir_list.push(monitoring_dir.clone());
                file_dir_map_list.insert(monitoring_dir.clone(),file_dir_map.clone());
            }
        }else{
                        error!("error in reading config, missing config!");
                        return Ok(());
        }


        let (supported_extensions,supported_types) = get_spprtd_extns_and_type(&file_dir_map);

        let poll_delay: Duration = Duration::from_secs(POLL_DELAY_SECS);

        info!("supported_types: {:?}\nsupported_extensions: {:?}",supported_types,supported_extensions); 
        info!("file_dir_map_list: {:?}",file_dir_map_list);
    let mut watcher = notify::PollWatcher::new(tx,
         notify::Config::default()
         .with_poll_interval(poll_delay)
         )?;
        

        // spins a new watcher thread for each monitoring directory
        for monitoring_dir in monitoring_dir_list {
        info!("listening on {:?}",monitoring_dir);
            watcher.watch(&monitoring_dir.d_path, notify::RecursiveMode::NonRecursive)?;
        }


        // initialization
       let mut files_list: Vec<Box<File>> = get_files(&monitoring_dir).unwrap_or(vec![]);
        
        let _ = check_and_write_dir(
            &file_dir_map,
            &monitoring_dir,
            &files_list,
            &supported_extensions);

        let _ = move_files(&file_dir_map,&monitoring_dir,&files_list);

        for res in rx {
            let file_dir_map : BTreeMap<String,Vec<String>>= BTreeMap::new();
            match res {
                Ok( ref event)=>{
                    let event_monitoring_directory_list: Vec<Directory> = event.paths
                        .iter()
                        .filter_map(|e|{
                            match e.parent() {
                                Some(parent_path) => {
                                    Some(parent_path.display().to_string())
                                }
                                None =>{
                                    error!("Path {:?} has no parent directory, skipping.", e);
                                    None
                                }
                            }
                        })
                    .map(|e|Directory::from(e,vec![]))
                        .collect();

                    for event_monitoring_directory in event_monitoring_directory_list {
                        match_response(file_dir_map_list.get(&event_monitoring_directory).unwrap(),&supported_extensions,&res); // have guards before hand, so shouldn't crash
                    }
                }
                Err(_)=>{
                    todo!();
                    }
                }
            } 
        Ok(())
}
