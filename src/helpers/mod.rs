use std::{path::{self,PathBuf},fmt::{self,write},io};

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

pub fn get_files(dir: &Directory)->io::Result<Vec<File>>{
    let mut files: Vec<File> = vec![];
    if dir.d_path.is_dir(){
        if let Ok(d_path) = dir.d_path
                                .read_dir(){
            for entry in d_path {
                if let Ok(entry) = entry{
                    let mut file_buf = File::new(entry.path().to_str().unwrap().to_string(),);
                    files.push(file_buf);
                }
            }
        }
    }
    Ok(files)
}

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
