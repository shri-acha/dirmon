use crate::helpers::file_extension;
use std::collections::HashMap;
use notify::{recommended_watcher, Event, EventKind, Result};
use notify::{RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc;

#[derive(Debug)]
pub struct DirMonDirectory {
    pub w_dir: String,
    pub w_extns: Vec<String>,
}

#[derive(Debug)]
pub struct DirMonConfig {
    pub w_dirmon: HashMap<String,DirMonDirectory>,
}

impl DirMonConfig {
    pub fn w_event_handler(&self, e: &notify::Event, path: &mut PathBuf) {
        match &e.kind {
            EventKind::Create(_) => {
                println!("{:?}", file_extension(path));
                if let Some(ext) = file_extension(path) {
                    if let Some(dirmon_dir) = self.w_dirmon.get(path){
                        println!("{:?}",dirmon_dir);
                    }
                }
            }
            _ => {}
        }
        println!("[EVENT] Event at: {:?}", path);
    }
}

#[derive(Debug)]
pub struct DirMonInstance {
    pub w_dirmon_conf: DirMonConfig,
}

impl DirMonInstance {
    pub fn start_listening(&self) -> Result<()> {
        let (tx, rx) = mpsc::channel::<Result<Event>>(); // Non - Debug Channels

        let mut n_watcher = notify::recommended_watcher(tx)?; // Watcher instance

        for watch_dir in &self.w_dirmon_conf.w_dirmon {
            n_watcher.watch(Path::new(&watch_dir.w_dir), RecursiveMode::Recursive)?;
        }

        for res in rx {
            match res {
                Ok(e) => {
                    for path in &e.paths {
                        let mut path = path.to_path_buf().clone();
                        self.w_dirmon_conf.w_event_handler(&e, &mut path);
                    }
                }
                Err(_) => {
                    println!("[ERROR] Error handling event in instance");
                }
            }
        }
        return Ok(());
    }
}
