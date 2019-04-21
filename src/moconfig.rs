use crate::moenv::Environment;
use crate::moui::DEFAULT_PATH;
use crate::save::Save;
use crate::steam::Steam;
use crate::uihelper::UIHelper;
use crate::wine::Wine;
use gtk::prelude::*;
use gtk::ListStore;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub active_game: Option<String>,
    pub mofl_version: String,
    pub steam: Rc<Steam>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            active_game: None,
            mofl_version: env!("CARGO_PKG_VERSION").to_string(),
            steam: Rc::new(Steam::new()),
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
    /*pub fn to(&self, list: &ListStore) {
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
    }*/
    pub fn load(tmp_path: &PathBuf) -> Result<Config, std::io::Error> {
        if tmp_path.exists() == false {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        } else {
            match fs::read_to_string(tmp_path.as_path()) {
                Ok(v) => match serde_json::from_str(&v) {
                    Ok(v) => return Ok(v),
                    Err(e) => {
                        UIHelper::serde_err(tmp_path.as_path(), &e);
                        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
                    }
                },
                Err(e) => return Err(e),
            }
        }
    }
}

impl Save for Config {
    fn save(&self) -> Result<PathBuf, std::io::Error> {
        // TODO - Also save mods
        let mut cfg_path: PathBuf = Environment::get_home();
        cfg_path.push(DEFAULT_PATH);
        cfg_path.push("config.json");
        match serde_json::to_string_pretty(&self) {
            Ok(v) => match fs::write(&cfg_path.as_path(), v) {
                Ok(v) => return Ok(cfg_path),
                Err(e) => {
                    error!("Failed to write game config: {:?}", &e);
                    return Err(e);
                }
            },
            Err(e) => {
                UIHelper::serde_err(cfg_path.as_path(), &e);
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
    }
}