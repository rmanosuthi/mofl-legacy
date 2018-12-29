use std::path::PathBuf;
use std::env;
use std::fs;
use moui::DEFAULT_PATH;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    active_game: String,
    mofl_version: String
}

impl Config {
    pub fn new() -> Config {
        Config {
            active_game: "".to_string(),
            mofl_version: env!("CARGO_PKG_VERSION").to_string()
        }
    }
    pub fn init_folders() {
        let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        tmp_path.push(DEFAULT_PATH);
        fs::create_dir_all(&tmp_path);
    }
    pub fn get_active_game(&self) -> &String {
        return &self.active_game;
    }
}