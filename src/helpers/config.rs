use configparser::ini::Ini;
use std::collections::{BTreeMap, HashMap};
use crate::{Directory};
use crate::{error,info};

pub fn load_config(
    config_file_name: &'static str,
) -> Option<(
    Vec<Directory>,
    HashMap<Directory, BTreeMap<String, Vec<String>>>,
    BTreeMap<String, Vec<String>>,
)> {
    let mut monitoring_dir_list: Vec<Directory> = vec![];
    let mut file_dir_map_list: HashMap<Directory, BTreeMap<String, Vec<String>>> = HashMap::new();
    let mut file_dir_map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let mut config_raw = Ini::new_cs();
    // loading config with error guards
    if let Ok(config_loaded) = config_raw.load(config_file_name) {
        // loaded config works by parsing keys and option<value>
        //
        // file_dir_map: <HEADER_NAME,Vec<Extensions>>
        for (monitoring_dir_buf, file_dir_map_buf) in config_loaded {
            let monitoring_dir = Directory::from(monitoring_dir_buf.clone(), vec![]);
            // type_value (????) , extensions
            for (type_value, extns) in file_dir_map_buf {
                if let Some(extns) = extns {
                    // println!("{:?}",extns);
                    file_dir_map.insert(
                        type_value,
                        extns.split(',').map(|e| e.to_string()).collect(),
                    );
                } else {
                    info!("missing values for {:?}", type_value);
                }
            }
            monitoring_dir_list.push(monitoring_dir.clone());
            file_dir_map_list.insert(monitoring_dir.clone(), file_dir_map.clone());
        }
        Some((monitoring_dir_list, file_dir_map_list, file_dir_map))
    } else {
        error!("error in reading config, missing config!");
        None
    }
}
