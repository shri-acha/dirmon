use crate::Directory;
use std::collections::{BTreeMap, HashMap, HashSet};

/// maybe returns the type for the extension
pub fn get_type_for_extension(
    file_dir_map: &BTreeMap<String, Vec<String>>,
    extension: &String,
) -> Option<String> {
    for (key, val) in file_dir_map {
        if val.contains(extension) {
            return Some(key.to_string());
        }
    }
    None
}

/// returns supported extensions and types from the source map  
pub fn get_supported_extension_and_type(
    directory: &Directory,
    file_dir_map_list: &HashMap<Directory, BTreeMap<String, Vec<String>>>,
) -> Option<(HashSet<String>, Vec<String>)> {
    let file_dir_map = file_dir_map_list.get(directory)?;

    let type_list: Vec<String> = file_dir_map
        .iter()
        .map(|(k, _)| k.to_string())
        .collect::<Vec<_>>();

    let extn_list: HashSet<String> = file_dir_map
        .iter()
        .flat_map(|(_, v)| v.clone())
        .map(|v| v.to_string())
        .collect();

    Some((extn_list, type_list))
}
