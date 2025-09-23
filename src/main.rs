// core

mod helpers;
use helpers::*;
use std::collections::{HashMap,HashSet};
use std::{path::{self},fmt::{self,write},io::{self},fs};
use std::thread;
use std::time::Duration;
use fs_extra::file;
use notify::{self,Event,Watcher};
use std::sync::mpsc;


fn main() ->notify::Result<()>{

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();


    let supported_extensions: Vec<_> = vec!["mp3","mp4","wav","mov","pdf","txt","bash","sh"];
    let supported_types: Vec<_> = vec!["Audio","Video","Documents","Executables"];
    let monitoring_dir: Directory = Directory::new(String::from("/home/shri/.gitbuilds/dirmon/test/test_directory/"),vec![]);
    let poll_delay: Duration = Duration::from_secs(1);
    let mut watcher = notify::PollWatcher::new(tx,
        notify::Config::default()
        .with_poll_interval(poll_delay)
        )?;

    // let mut terminal = ratatui::init();
    // let mut app  = App::new(monitoring_dir);

    // let app_result = app.run(&mut terminal);

    // ratatui::restore();
    // app_result

     // println!("Extensions: {:?}",u_extensions);
    
        //loop {

        //    thread::sleep(poll_delay);
            //directory creation pass
            
           // let files_list: Vec<Box<File>> = get_files(&monitoring_dir).unwrap_or(vec![]);
            
           // if files_list.len() <= 0 {
           //     continue;
           // }else {
           //     let mut extensions: Vec<String> = vec![];
           //      for file in &files_list {
           //          // println!("{:?}",file.f_extension);
           //          if supported_extensions.contains(&file.f_extension.as_str()){
           //             extensions.push(file.f_extension.clone());
           //         }
           //      }

           //      let mut u_extensions:HashSet<String> = extensions.into_iter().map(|e| e).collect();

           //      let _ = check_and_write_dir(&monitoring_dir,&u_extensions);
           //      let _ = move_files(&monitoring_dir,&files_list);
           // }
        //}
       
        watcher.watch(&monitoring_dir.d_path, notify::RecursiveMode::NonRecursive)?;

        for res in rx {
            match res {
                Ok(event) => {
                    //directory creation pass 
                   let files_list: Vec<Box<File>> = get_files(&monitoring_dir).unwrap_or(vec![]);
                    
                   if files_list.len() <= 0 {
                       continue;
                   }else {
                       let mut extensions: Vec<String> = vec![];
                        for file in &files_list {
                            // println!("{:?}",file.f_extension);
                            if supported_extensions.contains(&file.f_extension.as_str()){
                               extensions.push(file.f_extension.clone());
                           }
                        }

                        let mut u_extensions:HashSet<String> = extensions.into_iter().map(|e| e).collect();

                        let _ = check_and_write_dir(&monitoring_dir,&u_extensions);
                        let _ = move_files(&monitoring_dir,&files_list);
           }
                },
                Err(e) => return Ok(()),
            }
        }
        return Ok(())
}

