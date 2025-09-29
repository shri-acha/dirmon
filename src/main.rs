// core
mod helpers;
use helpers::*;
use std::collections::{BTreeMap,HashSet};
use std::{path::{self},fmt::{self,write},io::{self},fs};
use std::time::Duration;
use fs_extra::file;
use notify::{self,Watcher};
use std::sync::mpsc;
use std::thread::{self};
use std::sync::Arc;
use configparser::ini::Ini;



fn main()->notify::Result<()>{

        let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
        let mut config_raw  = Ini::new_cs(); 

        let mut monitoring_dir: Directory = Directory::default();

        let mut file_dir_map : BTreeMap<String,Vec<String>> =BTreeMap::new();

        if let Ok(config_loaded) = config_raw.load("test/test-conf-00.conf"){
            for (monitoring_dir_buf,file_dir_map_buf) in config_loaded {
                monitoring_dir = Directory::from(monitoring_dir_buf,vec![]);
                for (type_value,extns) in file_dir_map_buf {
                    if let Some(extns) = extns{
                        // println!("{:?}",extns);
                        file_dir_map.insert(type_value,extns.split(',').map(|e| e.to_string()).collect());
                    }else {

                        println!("[WARNING] missing values for {:?}",type_value);
                    }
                }
            }
        }else{
                        println!("[ERROR] error in reading config");
        }

        let (supported_extensions,supported_types) = get_spprtd_extns_and_type(&file_dir_map);

        let poll_delay: Duration = Duration::from_secs(1);

        println!("listening on {:?}",monitoring_dir);
        println!("supported_types: {:?}\nsupported_extensions: {:?}",supported_extensions, supported_types); 
        
    let mut watcher = notify::PollWatcher::new(tx,
         notify::Config::default()
         .with_poll_interval(poll_delay)
         )?;
         
        watcher.watch(&monitoring_dir.d_path, notify::RecursiveMode::NonRecursive)?;
        let mut count: u32=0;

       let files_list: Vec<Box<File>> = get_files(&monitoring_dir).unwrap_or(vec![]);
        
        // initialization
        let _ = check_and_write_dir(
            &file_dir_map,
            &monitoring_dir,
            &files_list,
            &supported_extensions);

        let _ = move_files(&file_dir_map,&monitoring_dir,&files_list);

        for res in rx {
            match res {
                Ok(event) => {
                    //directory creation pass 
                    
                   if files_list.len() <= 0 {
                       continue;
                   }else {

                       match check_and_write_dir(
                            &file_dir_map,
                            &monitoring_dir,
                            &files_list,
                            &supported_extensions) {

                           Ok(_) => {
                            println!("directory modified!");
                           }
                            Err(e)=>{
                            println!("error modifying directory!: {}",e);
                            println!("[STATE]:\t{:?}{:?}{:?}{:?}",
                            &file_dir_map,
                            &monitoring_dir,
                            &files_list,
                            &supported_extensions);
                        }
                       }

                        if let Ok(_) = move_files(&file_dir_map,&monitoring_dir,&files_list){
                            println!("files moved!");
                        }else {
                            println!("error moving files!");
                        }
                        count+=1;
                        println!("[COUNT]: {}\nEvent:{:?}",count,event);
           }
                },
                Err(e) => return Err(e),
            }
        }
        Ok(())
}
