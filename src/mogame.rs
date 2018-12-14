use momod::momod;
use std::fs::File;
use std::io::prelude::*;

pub struct mogame {
    pub label: String,
    pub executables: Vec<String>,
    pub mods: Vec<momod>
}
impl mogame {
    pub fn new() -> mogame {
        mogame {
            label: "".to_string(),
            executables: Vec::new(),
            mods: Vec::new()
        }
    }
    /// stub - Start a process
    pub fn start(&self, exe: String) -> bool {
        // check if file exists
        // spawn child process
        return true;
    }
    /// stub - Stop a process
    pub fn stop(&self, exe: String) -> bool {
        // check if file exists
        // stop child process
        return true;
    }
}