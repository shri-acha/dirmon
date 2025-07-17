use std::path::{self, PathBuf};

pub fn file_extension(file_path: &mut std::path::PathBuf) -> Option<String> {
    if let Some(file_name) = file_path.file_name() {
        let file_name = file_name.to_string_lossy().to_string(); // Doesn't support non-UTF-8 characters
        let file_name_arr = file_name.split(".").collect::<Vec<_>>();
        if let Some(ext) = file_name_arr.get(1) {
            Some(ext.to_string())
        } else {
            None
        }
    } else {
        None
    }
}
