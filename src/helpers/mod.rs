use std::{path::{self,PathBuf},fmt::{self,write},io,fs::{self}};
use std::collections::{HashMap,HashSet};
use std::hash::Hash;



#[derive(Debug,Clone)]
pub struct File {
    pub f_path: path::PathBuf,
    pub f_name: String,
    pub f_extension: String,
}

impl File{
    pub fn new(f_path: String)->Self{
        let f_path = PathBuf::from(f_path);
        let f_name = f_path.file_name().unwrap().to_str().unwrap_or("").to_string();
        let f_extension = get_file_extension(&f_name).unwrap_or("".to_string());
        Self{
            f_path,
            f_name,
            f_extension,
        }
    }
}

impl fmt::Display for File{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        let casted_path = self.f_path.clone()
                            .into_os_string()
                            .into_string()
                            .unwrap_or_default();
        write!(f,
            "{}",
            casted_path)
    }
}

#[derive(Debug,Clone)]
pub struct Directory{
    pub d_path: PathBuf,
    pub d_files :Vec<File>, 
}

impl Directory {

    pub fn new(d_path: String,d_files: Vec<File>)->Self{
        Self{
            d_path:PathBuf::from(d_path),
            d_files:d_files,
        }
    }

    pub fn add_file(&mut self,file_buf: File)->io::Result<()>{
        self.d_files.push(file_buf);
        Ok(())
    }
}


fn get_file_extension(file_name: &String)->Option<String>{

    let mut vec_buf: Vec<String> = vec![];
    for split_str in file_name.split('.'){
        let split_str = String::from(split_str);
        vec_buf.push(split_str);
    }

    vec_buf.get(1).cloned()
}

pub fn get_files(dir: &Directory)->io::Result<Vec<Box<File>>>{

    let mut files: Vec<Box<File>> = vec![];
    if dir.d_path.is_dir(){
        if let Ok(d_path) = dir.d_path
                                .read_dir(){
            for entry in d_path {
                if let Ok(entry) = entry{
                    let mut file_buf = Box::new(File::new(entry.path().to_str().unwrap().to_string(),));
                    files.push(file_buf);
                }
            }
        }
    }
    Ok(files)
}

pub fn get_type_for_extension(extension: &str)->Option<String>{

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

pub fn move_files(monitoring_dir: &Directory,files_list: &Vec<Box<File>>)->Result<String,String> {
    for file in files_list{
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
        return Err("internal error, no type exists for supported extension!".to_string());
}

pub fn check_and_write_dir(monitoring_dir:&Directory ,u_extensions: &HashSet<String>)->Result<String,String>{
    for extension in u_extensions.iter(){
        let dir_name = get_type_for_extension(extension);
        if let Some(dir_name) = dir_name {
            let u_path = monitoring_dir.d_path.join(dir_name);
            if !u_path.exists(){
                if let Ok(_) = fs::create_dir(&u_path){
                    println!("{:?} created!",u_path);
                    return Ok(format!("{:?} created!",u_path));
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
    return Err(format!("no files in directory!"));
}
