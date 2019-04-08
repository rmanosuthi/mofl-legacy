use gtk::ApplicationWindow;
use std::rc::Rc;
use crate::moenv::Environment;
use crate::moui::UI;
use crate::uihelper::UIHelper;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Steam {
    location: PathBuf
}

impl Steam {
    pub fn new() -> Steam {
        let mut try_steam_path = Environment::get_home();
        if cfg!(target_os = "linux") {
        try_steam_path.push(".steam");
        try_steam_path.push("steam");
        } else if cfg!(target_os = "macos") {
            warn!("MacOS detected, mofl support is on a best-effort basis!");
            try_steam_path.push("Library");
            try_steam_path.push("Application Support");
            try_steam_path.push("Steam");
        } else {
            try_steam_path = UIHelper::dialog_path_crit("Unsupported platform, please locate where Steam is installed", Some("Steam directory not given and you're on an unsupported platform. Aborting."));
        }

        match fs::read_dir(&try_steam_path) {
            Ok(v) => {
                info!("Steam path is {:?}", &try_steam_path);
                return Steam {
                    location: try_steam_path
                }
            }
            Err(e) => {
                let prompt_steam = UIHelper::dialog_path_crit("Please locate where Steam is installed", Some("The Steam installation folder was not specified and mofl couldn't determine it automatically. Aborting."));
                info!("Steam path is {:?}", &prompt_steam);
                return Steam {
                    location: prompt_steam
                }
            },
        }
    }
    pub fn get_common_entries(&self) -> Vec<PathBuf> {
        let mut result: Vec<PathBuf> = Vec::new();
        let mut try_common = PathBuf::from(&self.location);
        try_common.push("steamapps");
        try_common.push("common");
        debug!("Steam: getting common entries {:?}", &try_common);
        for entry in WalkDir::new(&try_common).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            debug!("Steam: common entries {:?}", &entry.path());
            result.push(entry.path().to_path_buf());
        }
        return result;
    }
    pub fn get_game_path(&self, name: &String) -> PathBuf {
        for game in self.get_common_entries() {
            match game.file_name() {
                Some(v) => if v.to_str().unwrap() == name {
                    debug!("Steam game path: {:?}", &game);
                    return game;
                },
                None => ()
            }
        }
        return UIHelper::dialog_path_crit("Please specify where the game is...", None);
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