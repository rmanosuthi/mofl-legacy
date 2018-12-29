use std::path::PathBuf;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    active_game: PathBuf,
    mofl_version: String
}