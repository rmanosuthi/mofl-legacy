use momod::momod;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct mogame {
    pub label: String,
    pub executables: Vec<PathBuf>,
    pub mods: Vec<momod>,
    pub folder_layout: Vec<PathBuf>,
    pub last_load_order: i64
}
impl mogame {
    pub fn new() -> mogame {
        mogame {
            label: "".to_string(),
            executables: Vec::new(),
            mods: Vec::new(),
            folder_layout: Vec::new(),
            last_load_order: -1
        }
    }
    /// stub - Start a process
    pub fn start(&self, exe: PathBuf) -> bool {
        // check if file exists
        // spawn child process
        return true;
    }
    /// stub - Stop a process
    pub fn stop(&self, exe: PathBuf) -> bool {
        // check if file exists
        // stop child process
        return true;
    }
}