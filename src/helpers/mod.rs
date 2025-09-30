use std::{path::{self,PathBuf},fmt::{self,write},io,fs::{self}};
use std::collections::{HashSet,BTreeMap};
use std::hash::Hash;
use log::{error,info,debug};

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
        let f_extension = f_path.extension() 
                        .and_then(|os_string| os_string.to_str())
                          .map(|s| s.to_string())
                          .unwrap_or_default();

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
    pub d_path: Box<PathBuf>,
    pub d_files :Vec<File>, 
}

impl Default for Directory {
    fn default()->Self{
        Self{
            d_path: Box::new(PathBuf::from("")),
            d_files: vec![],
        }
    }
}

impl Directory {

    pub fn from(d_path: String,d_files: Vec<File>)->Self{
        Self{
            d_path:Box::new(PathBuf::from(d_path)),
            d_files:d_files,
        }
    }

    pub fn add_file(&mut self,file_buf: File)->io::Result<()>{
        self.d_files.push(file_buf);
        Ok(())
    }
}

/// returns the list of files within the desired directory
pub fn get_files(dir: &Directory)->io::Result<Vec<Box<File>>>{

    let mut files: Vec<Box<File>> = vec![];

    if dir.d_path.is_dir(){
        if let Ok(d_path) = dir.d_path
                                .read_dir(){
            for entry in d_path {
                if let Ok(entry) = entry{
                    if !entry.file_type()?.is_dir(){
                        let mut file_buf = Box::new(File::new(entry.path().to_str().unwrap_or_default().to_string(),)); // breaks for unicode
                        files.push(file_buf);
                    }
                }
            }
        }
    }else {
        error!("path configured to monitor is not a directory!");
        return Err(io::Error::other("path configured to monitor is not a directory!"));
    }
    Ok(files)
}
/// maybe returns the type for the extension
pub fn get_type_for_extension(file_dir_map: &BTreeMap<String,Vec<String>>,extension: &String )->Option<String>{
 
    for (key,val) in file_dir_map {
            if val.contains(extension){
                return Some(key.to_string());
            }
    }
    return None;

}

/// moves the desired file into their corresponding directory(Type)
pub fn move_files(file_dir_map: &BTreeMap<String,Vec<String>>,
    monitoring_dir: &Directory,files_list: &Vec<Box<File>>)->Option<String> {

    for file in files_list{
        if let Some(dir_name) = get_type_for_extension(file_dir_map,&file.f_extension){
        let u_path = monitoring_dir.d_path.join(dir_name);
        let d_path = u_path.join(file.f_name.clone());
        let s_path = &file.f_path;

            if s_path.exists() {
                if !d_path.exists() {
                    if let Ok(size) =  fs_extra::file::move_file(s_path,&d_path,
                        &fs_extra::file::CopyOptions::new()){
                        debug!("successfully moved file! [{}] s:{:?}\td:{:?}",size,s_path,&d_path);
                    }else {
                        debug!("failed to move file! s:{:?}\td:{:?}",s_path,&d_path);
                    }
                }else {
                        info!("file already exists in the destination!");
                }
            }
            else{
                    error!("{:?} source directory doesn't exist!",s_path);
                }
            }
        }
        return Some("all files scanned successfully!".to_string());
}

/// checks for the desired extension of files and creates the corresponding parent sub-directory
pub fn check_and_write_dir(
    file_dir_map: &BTreeMap<String,Vec<String>>,
    monitoring_dir:&Directory,files_list: &Vec<Box<File>>,
    supported_extensions: &HashSet<String>
    )->io::Result<String>{ 

   let mut u_extensions: HashSet<String> = HashSet::new();

   // keeps a list of unique extensions that are also supported by the instance
    for file in files_list {
        // println!("{:?}{:?}",file.f_extension, supported_extensions); 
        if supported_extensions.contains(&file.f_extension){
           u_extensions.insert(file.f_extension.clone());
       }
    }
    
    if u_extensions.len() <= 0 {
     debug!("directory empty"); 
    }
    else {
    for extension in u_extensions.iter(){

        let dir_name = get_type_for_extension(file_dir_map,extension);
        // println!("{:?}",dir_name);

        if let Some(dir_name) = dir_name {

            let u_path = monitoring_dir.d_path.join(dir_name);
            if !u_path.exists(){
                debug!("{:?} source path exists!",u_path);

                if let Ok(_) = fs::create_dir(&u_path){
                    debug!("{:?} created!",u_path);
                    return Ok(format!("{:?} created!",u_path)); // necessary log (necessary)
                }else{
                   error!("{:?} creation failed!",u_path); // safe log (necessary)
                } 
            }
            else{
                 info!("{:?} already exists!",u_path); // floods log (unecessary)
            }
        }else{
            error!("{:?} extension type not supported!",dir_name); // floods log (only when a
                                                                     // file type )
        }
    }
    }
    return Ok(String::from("[DEBUG] return"))
}
/// returns supported extensions and types from the source map  
pub fn get_spprtd_extns_and_type(file_dir_map : &BTreeMap<String,Vec<String>>)->(HashSet<String>,Vec<String>)
{


    let type_list: Vec<String> = file_dir_map
        .iter()
        .map(|(k,_)| k.to_string())
        .collect::<Vec<_>>();

    let extn_list: HashSet<String> = file_dir_map

        .iter()
        .flat_map(|(_,v)| {
            v.clone()
        })
        .map(|v|v.to_string())
        .collect();

    return (extn_list,type_list,);
}
