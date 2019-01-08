use std::ffi::OsStr;
use crate::mogame::Game;
use crate::momod::Mod;
use std::path::PathBuf;
use std::fs;

/// stub - Given an MO2 game folder, create a populated MOFL game folder and return a Game struct
pub fn import(path: PathBuf) -> Option<Game> {
    let game_name = match path.file_name() {
        Some(v) => match v.to_str() {
            Some(v) => {
                v
            },
            None => {
                println!("Cannot read MO2 game name (&OsStr -> &str conversion failed), aborting...");
                println!("Does it contain non UTF-8 characters?");
                return None;
            }
        },
        None => {
            println!("Given MO2 game folder is invalid, aborting...");
            return None;
        }
    };
    let mut game = Game::new(String::from(game_name));
    let mut path = PathBuf::from(&path);
    path.push("mods");
    match fs::read_dir(&path) {
        Ok(v) => {
            for ref entry in v {
                match entry {
                    Ok(v) => {
                        println!("Reading {:?}", v.path());
                        match Mod::from_mo2(&game.path, v.path()) {
                            Some(v) => {
                                println!("Adding mod...");
                                game.mods.push(v);
                                },
                            None => ()
                        }
                    },
                    Err(e) => ()
                }
            }
        },
        Err(e) => ()
    }
    return Some(game);
}
