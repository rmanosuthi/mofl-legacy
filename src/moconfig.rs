use std::rc::Rc;
use crate::moenv::Environment;
use crate::moui::DEFAULT_PATH;
use crate::uihelper::UIHelper;
use crate::steam::Steam;
use gtk::prelude::*;
use gtk::ListStore;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    active_game: Option<String>,
    mofl_version: String,
    runtimes: Vec<(Runtimes, String, PathBuf)>,
    pub steam: Rc<Steam>
}

impl Config {
    pub fn new() -> Config {
        Config {
            active_game: None,
            mofl_version: env!("CARGO_PKG_VERSION").to_string(),
            runtimes: Vec::new(),
            steam: Rc::new(Steam::new())
        }
    }
    pub fn init_folders() -> Result<(), std::io::Error> {
        let mut tmp_path: PathBuf = Environment::get_home();
        tmp_path.push(DEFAULT_PATH);
        return fs::create_dir_all(&tmp_path);
    }
    pub fn init_game_folder(name: &String) {
        let mut game_cfg_path: PathBuf = Environment::get_home();
        game_cfg_path.push(DEFAULT_PATH);
        game_cfg_path.push("games");
        game_cfg_path.push(name);
        fs::create_dir_all(&game_cfg_path);
    }
    pub fn get_active_game(&self) -> &Option<String> {
        return &self.active_game;
    }
    pub fn to(&self, list: &ListStore) {
        for ref runtime in &self.runtimes {
            let runtime_str = match &runtime.0 {
                Runtimes::SystemWine => "System Wine",
                Runtimes::LutrisWine => "Lutris Wine",
                Runtimes::Proton => "Proton",
                _ => "Invalid",
            };
            list.insert_with_values(
                None,
                &[0, 1, 2],
                &[&runtime_str, &runtime.1, &runtime.2.to_str()],
            );
        }
    }
    pub fn load(tmp_path: &PathBuf) -> Option<Config> {
        match fs::read_to_string(tmp_path.as_path()) {
            Ok(v) => match serde_json::from_str(&v) {
                Ok(v) => return v,
                Err(e) => {
                    UIHelper::serde_err(&e);
                    return None;
                }
            },
            Err(e) => {
                info!("Creating new config at {}", tmp_path.display());
                let new_config = Config::new();
                match serde_json::to_string_pretty(&new_config) {
                    Ok(v) => match fs::write(tmp_path.as_path(), v) {
                        Ok(v) => (),
                        Err(e) => {
                            warn!("Failed to write new game config");
                            debug!("Error: {:?}", e);
                            return None;
                        }
                    },
                    Err(e) => {
                        UIHelper::serde_err(&e);
                        return None;
                    }
                }
                return Some(new_config);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Runtimes {
    SystemWine,
    LutrisWine,
    Proton,
}
