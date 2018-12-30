use std::path::PathBuf;
use std::env;
use std::fs;
use moui::DEFAULT_PATH;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    active_game: String,
    mofl_version: String,
    runtimes: Vec<(Runtimes, String, PathBuf)>
}

impl Config {
    pub fn new() -> Config {
        Config {
            active_game: "".to_string(),
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
    pub fn get_active_game(&self) -> &String {
        return &self.active_game;
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Runtimes {
    SystemWine, LutrisWine, Proton
}