use std::path::PathBuf;
use std::env;
use std::fs;
use moui::DEFAULT_PATH;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    active_game: PathBuf,
    mofl_version: String
}

impl Config {
    pub fn new() -> Config {
        Config {
            active_game: PathBuf::new(),
            mofl_version: env!("CARGO_PKG_VERSION").to_string()
        }
    }
    pub fn init_folders() {
        let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        tmp_path.push(DEFAULT_PATH);
        fs::create_dir_all(&tmp_path);
    }
}