use notify::{recommended_watcher, Event, Result};
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;

#[derive(Debug)]
pub struct DirMonDirectory {
    pub w_dir: String,
    pub w_extns: Vec<String>,
}

#[derive(Debug)]
pub struct DirMonConfig {
    pub w_dirmon: Vec<DirMonDirectory>,
}

impl DirMonConfig {
    pub fn w_event_handler(&self) {

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
                        println!("[EVENT] Event at:\n{:?}\t{:?}",path,e.kind);
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
