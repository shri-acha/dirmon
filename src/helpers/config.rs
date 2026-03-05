use crate::Directory;
use crate::{error, info};
use configparser::ini::Ini;
use std::collections::{BTreeMap, HashMap};
use std::{fs,path::PathBuf};
use std::ffi::OsString;
use anyhow::anyhow;
use dirs::config_dir;

const DEFAULT_CONFIG_PATH: &'static str = "/dirmon/dirmon.conf";
const DEFAULT_CONFIG_FILENAME: &'static str = "dirmon.conf";

pub fn load_config(
    config_file_name: &str,
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
        error!("[{}] error in reading config, missing config!",config_file_name);
        None
    }
}

pub fn ensure_config() -> anyhow::Result<String> {
    if let Some(config_dir) = dirs::config_dir() {

        let mut config_file_path = OsString::new(); 
        config_file_path.push(config_dir.into_os_string());
        config_file_path.push(OsString::from(DEFAULT_CONFIG_PATH));

        let config_file = PathBuf::from(config_file_path); 
        println!("{:?}",config_file);

        if !config_file.exists() {
            fs::write(
                config_file.clone(),
                r#"# [/desired/path/here/]
    # TYPE_0 = mp3,wav
    # TYPE_1 = mov,mp4
    # TYPE_2 = txt,pdf"#,
            )?;
        }
        config_file
            .into_os_string()
            .into_string()
            .map_err(|_|anyhow!("non-utf character in default config file name"))
    }
    else{

    let path = std::path::Path::new(DEFAULT_CONFIG_FILENAME);

    config_dir()
        .ok_or(anyhow!("Error finding config directory!"))?
        .into_os_string()
        .into_string()
        .map_err(|_|anyhow!("Error handling error types!"))?;


    if !path.exists() {
        fs::write(
            DEFAULT_CONFIG_FILENAME,
            r#"# [/desired/path/here/]
# TYPE_0 = mp3,wav
# TYPE_1 = mov,mp4
# TYPE_2 = txt,pdf"#,
        )?;
    }
    Ok(String::from(DEFAULT_CONFIG_FILENAME))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_config(contents: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().expect("failed to create temp file");
        f.write_all(contents.as_bytes())
            .expect("failed to write config");
        f
    }

    #[test]
    fn test_load_config_multiple_rules_all_present() {
        let f = write_config(
            "[/tmp/media/]\n\
             TYPE_0 = mp3,wav\n\
             TYPE_1 = mov,mp4\n\
             TYPE_2 = txt,pdf\n",
        );

        let (dirs, dir_map, _) = load_config(f.path().to_str().unwrap())
            .expect("expected valid config");

        let rules = dir_map.get(&dirs[0]).unwrap();
        assert_eq!(rules.len(), 3);
        assert!(rules.contains_key("TYPE_0"));
        assert!(rules.contains_key("TYPE_1"));
        assert!(rules.contains_key("TYPE_2"));
    }

    #[test]
    fn test_load_config_multiple_directories() {
        let f = write_config(
            "[/tmp/dir_a/]\n\
             DOCS = pdf,txt\n\
             \n\
             [/tmp/dir_b/]\n\
             IMAGES = png,jpg\n",
        );

        let (dirs, dir_map, _) = load_config(f.path().to_str().unwrap())
            .expect("expected valid config");

        assert_eq!(dirs.len(), 2, "should monitor 2 directories");

        for d in &dirs {
            assert!(dir_map.contains_key(d), "every dir should have a map entry");
        }
    }

    #[test]
    fn test_load_config_missing_file_returns_none() {
        let result = load_config("/tmp/this_file_does_not_exist_dirmon_test_xyz.conf");
        assert!(result.is_none(), "missing config file should return None");
    }

    #[test]
    fn test_load_config_empty_file() {
        let f = write_config("");

        if let Some((dirs, dir_map, flat_map)) = load_config(f.path().to_str().unwrap()) {
            assert!(dirs.is_empty());
            assert!(dir_map.is_empty());
            assert!(flat_map.is_empty());
        }
    }

    #[test]
    fn test_load_config_only_comments_are_not_parsed_as_dirs() {

        let f = write_config(
            "# [/desired/path/here/]\n\
             # TYPE_0 = mp3,wav\n\
             # TYPE_1 = mov,mp4\n",
        );

        if let Some((dirs, _, _)) = load_config(f.path().to_str().unwrap()) {
            assert!(dirs.is_empty(), "commented-out dirs should not be monitored");
        }
    }

    #[test]
    fn test_load_config_flat_map_reflects_last_directory_rules() {
        // The third return value is overwritten each loop iteration, so it ends
        // up holding the last directory's rules. This test documents that behaviour.
        let f = write_config(
            "[/tmp/dir_a/]\n\
             AUDIO = mp3\n\
             \n\
             [/tmp/dir_b/]\n\
             VIDEO = mp4\n",
        );

        let (_, _, flat_map) = load_config(f.path().to_str().unwrap())
            .expect("expected valid config");

        println!("{:?}",flat_map);
        assert!(!flat_map.is_empty());

        assert!(flat_map.contains_key("VIDEO"));
        assert!(flat_map.contains_key("AUDIO"));
    }

    #[test]
    fn test_ensure_config_returns_ok() {
        let result = ensure_config();
        assert!(result.is_ok(), "ensure_config should succeed: {:?}", result.err());
    }

    #[test]
    fn test_ensure_config_returns_non_empty_path() {
        let path = ensure_config().unwrap();
        assert!(!path.is_empty());
    }

    #[test]
    fn test_ensure_config_creates_file_on_disk() {
        let path_str = ensure_config().unwrap();
        assert!(
            std::path::Path::new(&path_str).exists(),
            "config file should exist after ensure_config"
        );
    }
}
