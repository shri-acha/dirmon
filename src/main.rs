// core
mod helpers;
mod tui;
use helpers::*;
use std::collections::{HashMap,HashSet};
use std::{path::{self},fmt::{self,write},io::{self},fs};
use std::thread;
use std::time::Duration;
use fs_extra::file;
use notify::{self,Watcher};
use std::sync::mpsc;
use tui::*;
use std::thread::{self};
use std::sync::Arc;



fn main(){

        let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

        let supported_extensions: Vec<_> = vec!["mp3","mp4","wav","mov","pdf","txt","bash","sh"];
        let supported_types: Vec<_> = vec!["Audio","Video","Documents","Executables"];

        let file_dir_map_buf BTreeMap<String,Vec<String>> =BTreeMap::from([("Audio".to_string(),vec!["mp3".to_string()])]) 


        let monitoring_dir: Directory = Directory::new(String::from("/home/shri/.gitbuilds/dirmon/test/test_directory/"),vec![]);
        let poll_delay: Duration = Duration::from_secs(1);


        // monitoring_dir,
        // dir_buffer,
        // ext_buffer,
        // file_dir_map,
        // focused_field,
        // exit,

        let mut app = Arc::new(App::new(monitoring_dir.d_path,
                "".to_string(),
                "".to_string(),
                file_dir_map_buf,
                FocusedField::Directory);


    thread::spawn(move||{

        let mut terminal = ratatui::init(); // initializing terminal

        while !app.exit {
            let _ = app.run(&mut terminal);
        }
    ratatui::restore(); 
    })

         
        
        let mut watcher = notify::PollWatcher::new(tx,
             notify::Config::default()
             .with_poll_interval(poll_delay)
             )?;
        
       
     
        watcher.watch(&app.monitoring_dir.d_path, notify::RecursiveMode::NonRecursive)?;
        let mut count: u32= 0;

       let files_list: Vec<Box<File>> = get_files(&app.monitoring_dir).unwrap_or(vec![]);
        
        // initialization
        let _ = check_and_write_dir(&app.monitoring_dir,&files_list,&supported_extensions);
        let _ = move_files(&app.monitoring_dir,&files_list);

        for res in rx {
            match res {
                Ok(event) => {
                    //directory creation pass 
                    
                   if files_list.len() <= 0 {
                       continue;
                   }else {

                        let _ = check_and_write_dir(&app.monitoring_dir,&files_list,&supported_extensions);
                        let _ = move_files(&app.monitoring_dir,&files_list);
                        count+=1;
                        println!("[COUNT]: {}\nEvent:{:?}",count,event);
           }
                },
                Err(e) => return Err(e),
            }
        }
        return Ok(())
}

