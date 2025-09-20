mod helpers;
use helpers::*;
use std::collections::HashMap;
use std::{path::{self,PathBuf},fmt::{self,write},io::{self}};



fn main() {
    let monitoring_dir: Directory = Directory::new(String::from("/home/shri/Downloads/"),vec![]);
    let files_list = get_files(&monitoring_dir);
    println!("{:?}", files_list);
}




