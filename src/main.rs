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



fn main(){

        let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

        let supported_extensions: Vec<_> = vec!["mp3","mp4","wav","mov","pdf","txt","bash","sh"];

        let supported_types: Vec<_> = vec!["Audio","Video","Documents","Executables"];

        let monitoring_dir: Directory = Directory::new(String::from("/home/shri/.gitbuilds/dirmon/test/test_directory/"),vec![]);
        let poll_delay: Duration = Duration::from_secs(1);

        // 
        //
        // let mut watcher = notify::PollWatcher::new(tx,
        //     notify::Config::default()
        //     .with_poll_interval(poll_delay)
        //     )?;
        //
        //

        let mut terminal = ratatui::init(); // initializing terminal
        let mut app = App::default();

        while !app.exit {
            let _ = app.run(&mut terminal);
        }

    ratatui::restore(); 

    
       
        // watcher.watch(&monitoring_dir.d_path, notify::RecursiveMode::NonRecursive)?;
        //let mut count: u32= 0;

       //let files_list: Vec<Box<File>> = get_files(&monitoring_dir).unwrap_or(vec![]);
        
        //// initialization
        //let _ = check_and_write_dir(&monitoring_dir,&files_list,&supported_extensions);
        //let _ = move_files(&monitoring_dir,&files_list);

        //for res in rx {
        //    match res {
        //        Ok(event) => {
        //            //directory creation pass 
                    
        //           if files_list.len() <= 0 {
        //               continue;
        //           }else {

        //                let _ = check_and_write_dir(&monitoring_dir,&files_list,&supported_extensions);
        //                let _ = move_files(&monitoring_dir,&files_list);
        //                count+=1;
        //                println!("[COUNT]: {}\nEvent:{:?}",count,event);
        //   }
        //        },
        //        Err(e) => return Err(e),
        //    }
        //}
        //return Ok(())
}

