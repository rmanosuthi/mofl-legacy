use crate::mogame::Game;
use std::path::PathBuf;

/// stub - Given an MO2 game folder, create a populated MOFL game folder and return a Game struct
pub fn import(path: PathBuf) -> Game {
    return Game::new("".to_string());
}
