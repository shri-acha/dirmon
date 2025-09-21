mod helpers;
use helpers::*;
use std::collections::{HashMap,HashSet};
use std::{path::{self,PathBuf},fmt::{self,write},io::{self},fs};
use std::thread;
use std::time::Duration;
use fs_extra::file;

fn main() {

    let supported_extensions: Vec<_> = vec!["mp3","mp4","wav","mov","pdf","txt","bash","sh"];

    let supported_types: Vec<_> = vec!["Audio","Video","Documents","Executables"];

    let monitoring_dir: Directory = Directory::new(String::from("/home/shri/Documents/TICKETS"),vec![]);

    let poll_delay: Duration = Duration::from_secs(1);






    // println!("Extensions: {:?}",u_extensions);
    //
        loop {

            thread::sleep(poll_delay);
            // directory creation pass
            //
            let files_list: Vec<Box<File>> = get_files(&monitoring_dir).unwrap_or(vec![]);


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
    }
