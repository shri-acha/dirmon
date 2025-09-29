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



fn main()->notify::Result<()>{

        let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

        let file_dir_map_buf : BTreeMap<String,Vec<String>> =BTreeMap::from([
            ("Documents".to_string(),vec!["txt".to_string(),"pdf".to_string()]),
            ("Video".to_string(),vec!["mov".to_string(),"mp4".to_string()]),
            ("Audio".to_string(),vec!["wav".to_string(),"mp3".to_string()]),
            ]);

        let (supported_extensions,supported_types) = get_spprtd_extns_and_type(&file_dir_map_buf);

        let monitoring_dir: Directory = Directory::new(
            String::from("/home/shri/.gitbuilds/dirmon/test/test_directory"),vec![]
            );

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
        let _ = check_and_write_dir(&monitoring_dir,&files_list,&supported_extensions);
        let _ = move_files(&monitoring_dir,&files_list);

        for res in rx {
            match res {
                Ok(event) => {
                    //directory creation pass 
                    
                   if files_list.len() <= 0 {
                       continue;
                   }else {

                        if let Ok(_)  = check_and_write_dir(
                            &monitoring_dir,
                            &files_list,
                            &supported_extensions)
                        {
                            println!("directory modified!");
                        }else {
                            println!("error modifying directory!");
                        }
                        if let Ok(_) = move_files(&monitoring_dir,&files_list){
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
        return Ok(())
}
