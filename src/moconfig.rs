use gtk::ListStore;
use gtk::prelude::*;
use std::path::PathBuf;
use std::env;
use std::fs;
use crate::moui::DEFAULT_PATH;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    active_game: Option<String>,
    mofl_version: String,
    runtimes: Vec<(Runtimes, String, PathBuf)>
}

impl Config {
    pub fn new() -> Config {
        Config {
            active_game: None,
            mofl_version: env!("CARGO_PKG_VERSION").to_string(),
            runtimes: Vec::new()
        }
    }
    pub fn init_folders() {
        let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        tmp_path.push(DEFAULT_PATH);
        fs::create_dir_all(&tmp_path);
    }
    pub fn init_game_folder(name: &String) {
        let mut game_cfg_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap()); 
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
                _ => "Invalid"
            };
            list.insert_with_values(None, &[0, 1, 2], &[&runtime_str, &runtime.1, &runtime.2.to_str()]);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Runtimes {
    SystemWine, LutrisWine, Proton
}