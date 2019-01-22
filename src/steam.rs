use crate::moenv::Environment;
use crate::moui::UI;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
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
            Err(e) => return Steam {
                location: UI::dialog_path_crit("Please locate where Steam is installed")
            },
        }
    }
    pub fn get_common_entries(&self) -> Vec<PathBuf> {
        let mut result: Vec<PathBuf> = Vec::new();
        let mut try_common = PathBuf::from(&self.location);
        try_common.push("steamapps");
        try_common.push("common");
        for entry in WalkDir::new(&try_common).into_iter().filter_map(|e| e.ok()) {
            result.push(entry.path().to_path_buf());
        }
        return result;
    }
    pub fn get_game_path(&self, name: String) -> PathBuf {
        for game in self.get_common_entries() {
            match game.file_name() {
                Some(v) => if v.to_str().unwrap() == name {
                    return game;
                },
                None => ()
            }
        }
        return UI::dialog_path_crit("Please specify where the game is...");
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
impl Default for Steam {
    fn default() -> Self {
        panic!("Default requested for Steam, this should never happen, aborting");
    }
}