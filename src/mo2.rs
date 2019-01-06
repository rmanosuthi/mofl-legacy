use crate::mogame::Game;
use crate::momod::Mod;
use std::path::PathBuf;
use std::fs;

/// stub - Given an MO2 game folder, create a populated MOFL game folder and return a Game struct
pub fn import(path: PathBuf) -> Game {
    let mut game = Game::new(String::from(path.file_name().unwrap().to_str().unwrap()));
    let mut path = PathBuf::from(&path);
    path.push("mods");
    match fs::read_dir(&path) {
        Ok(v) => {
            for ref entry in v {
                match entry {
                    Ok(v) => {
                        println!("Reading {:?}", v.path());
                        match Mod::from_mo2(&game.path, v.path()) {
                            Some(v) => game.mods.push(v),
                            None => ()
                        }
                    },
                    Err(e) => ()
                }
            }
        },
        Err(e) => ()
    }
    return Game::new("".to_string());
}
