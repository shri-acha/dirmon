
use std::io::{self};
use std::path::{PathBuf,self};
use crate::fmt;

#[derive(Debug,Clone,Eq,Hash,PartialEq)]
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

#[derive(Debug,Clone,Eq,Hash,PartialEq)]
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

