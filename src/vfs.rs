use crate::mogame::Game;
use crate::moui::DEFAULT_PATH;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn generate(game: &Game) {
    let mut game_dir = PathBuf::from(env::var_os("HOME").unwrap());
    game_dir.push(DEFAULT_PATH);
    game_dir.push("games");
    game_dir.push(&game.label);
    game_dir.push("mods");
    let mut symlink_target = PathBuf::from("/tmp/mofl/game");
    symlink_target.push(&game.label);
    symlink_target.push("Data/");
    fs::create_dir_all(&symlink_target);
    match fs::read_dir(&game_dir) {
        Ok(v) => {
            for ref entry in v {
                match entry {
                    Ok(v) => {
                        // v is a mod's folder here (DirEntry)
                        let mut mod_folder = PathBuf::from(v.path());
                        mod_folder.push("Data/");
                        match fs::read_dir(&mod_folder) {
                            Ok(v) => {
                                for mod_entry in v {
                                    let from = PathBuf::from(mod_entry.unwrap().path());
                                    let mut to = PathBuf::from(&symlink_target);
                                    to.push(&from.file_name().unwrap());
                                    println!("Linking {:?} to {:?}", &from, &to);
                                    std::os::unix::fs::symlink(from, &to);
                                }
                            }
                            Err(e) => (),
                        }
                    }
                    Err(e) => (),
                }
            }
        }
        Err(e) => println!("Failed to read game dir"),
    }
}
