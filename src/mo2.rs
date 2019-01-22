use std::rc::Rc;
use crate::mogame::Game;
use crate::momod::Mod;
use crate::steam::Steam;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// stub - Given an MO2 game folder, create a populated MOFL game folder and return a Game struct
pub fn import(path: PathBuf, steam: Rc<Steam>) -> Option<Game> {
    let game_name = match path.file_name() {
        Some(v) => match v.to_str() {
            Some(v) => v,
            None => {
                println!(
                    "Cannot read MO2 game name (&OsStr -> &str conversion failed), aborting..."
                );
                println!("Does it contain non UTF-8 characters?");
                return None;
            }
        },
        None => {
            println!("Given MO2 game folder is invalid, aborting...");
            return None;
        }
    };
    let mut game = Game::new(String::from(game_name), steam);
    let mut path = PathBuf::from(&path);
    path.push("mods");
    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        println!("Reading {:?}", entry.path());
        match Mod::from_mo2(&game.mofl_game_path, PathBuf::from(entry.path())) {
            Some(v) => {
                println!("Adding mod...");
                game.mods.push(v);
            }
            None => (),
        }
    }
    return Some(game);
}
