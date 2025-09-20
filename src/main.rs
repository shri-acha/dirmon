mod helpers;
use helpers::*;
use std::collections::{HashMap,HashSet};
use std::{path::{self,PathBuf},fmt::{self,write},io::{self},fs};

fn get_type_for_extension(extension: &str)->Option<String>{

    let file_choice: HashMap<&str,Vec<&str>> = HashMap::from([
        ("Audio",vec!["mp3","wav"]),
        ("Videos",vec!["mp4","mov"]),
        ("Documents",vec!["pdf","txt"]),
        ("Executables",vec!["sh","bash"]),
    ]);
    
    for (key,val) in file_choice {
            if val.contains(&extension){
                return Some(key.to_string());
            }
    }
    return None;
}


fn main() {

    let supported_extensions: Vec<_> = vec!["mp3","mpv","wav","mov","pdf","txt"];

    let supported_types: Vec<_> = vec!["Audio","Video","Documents","Executables"];


    let monitoring_dir: Directory = Directory::new(String::from("/home/shri/Downloads/"),vec![]);
    let files_list: Vec<File> = get_files(&monitoring_dir).unwrap_or(vec![]);


    let mut extensions: Vec<String> = vec![];
    for file in files_list {
        // println!("{:?}",file.f_extension);
        if supported_extensions.contains(&file.f_extension.as_str()){
            extensions.push(file.f_extension);
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
                }
            }
            else{
                 println!("{:?} already exists!",u_path);
            }
        }else{
            println!("{:?} extension type not supported!",dir_name);
        }
    }

}
