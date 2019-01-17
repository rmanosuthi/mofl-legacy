use crate::moenv::Environment;
use std::fs;
use std::path::PathBuf;

pub struct Steam {
    location: PathBuf,
}

impl Steam {
    pub fn new() -> Steam {
        let mut try_steam_path = Environment::get_home();
        try_steam_path.push(".steam");
        try_steam_path.push("steam");
        match fs::read_dir(&try_steam_path) {
            Ok(v) => {
                return Steam {
                    location: try_steam_path,
                }
            }
            Err(e) => panic!("Cannot find steam"),
        }
    }
    pub fn get_common_entries(&self) -> Vec<PathBuf> {
        let mut result: Vec<PathBuf> = Vec::new();
        let mut try_common = PathBuf::from(&self.location);
        try_common.push("steamapps");
        try_common.push("common");
        match fs::read_dir(&try_common) {
            Ok(v) => {
                for ref entry in v {
                    match entry {
                        Ok(v) => {
                            result.push(v.path());
                        }
                        Err(e) => (),
                    }
                }
            }
            Err(e) => (),
        }
        return result;
    }
    // TODO: Only return version number
    pub fn get_proton_versions(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let common_entries = self.get_common_entries();
        for entry in common_entries {
            let name = entry.file_name().unwrap().to_str().unwrap();
            if name.contains("Proton") {
                result.push(String::from(name));
            }
        }
        return result;
    }
}
