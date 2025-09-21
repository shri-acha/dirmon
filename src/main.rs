mod helpers;
use helpers::*;
use std::collections::{HashMap,HashSet};
use std::{path::{self,PathBuf},fmt::{self,write},io::{self},fs};
use fs_extra::file;

fn main() {

    let supported_extensions: Vec<_> = vec!["mp3","mpv","wav","mov","pdf","txt"];

    let supported_types: Vec<_> = vec!["Audio","Video","Documents","Executables"];


    let monitoring_dir: Directory = Directory::new(String::from("/home/shri/Documents/TICKETS"),vec![]);

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

    // println!("Extensions: {:?}",u_extensions);

    for extension in u_extensions.iter(){
        let dir_name = get_type_for_extension(extension);
        if let Some(dir_name) = dir_name {
            let u_path = monitoring_dir.d_path.join(dir_name);
            if !u_path.exists(){
                if let Ok(_) = fs::create_dir(&u_path){
                    println!("{:?} created!",u_path);
                }else{
                    println!("{:?} creation failed!",u_path);
                } }
            else{
                 println!("{:?} already exists!",u_path);
            }
        }else{
            println!("{:?} extension type not supported!",dir_name);
        }
    }
    // file move pass
    //
    for file in &files_list{
        if let Some(dir_name) = get_type_for_extension(&file.f_extension){
        let u_path = monitoring_dir.d_path.join(dir_name);
        let d_path = u_path.join(file.f_name.clone());
        let s_path = &file.f_path;

            if u_path.exists(){
                if let Ok(size) =  fs_extra::file::move_file(s_path,&d_path,
                    &fs_extra::file::CopyOptions::new()){
                    println!("successfully moved file! [{}] s:{:?}\td:{:?}\n",size,s_path,&d_path);
                }else {
                    println!("failed to move file! s:{:?}\td:{:?}\n",s_path,&d_path);
                }
            }
            else{
                    println!("{:?} missing directory!",u_path);
                }
            }
        }
    }

