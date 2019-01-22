use crate::moenv::Environment;
use crate::mogame::Game;
use crate::moui::DEFAULT_PATH;
use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

// TODO
// - Traverse to last file since folders don't work well with symlinks, recursion?
// - Check load order before linking, necessary?
pub fn generate(game: &Game) {
    let mut game_dir = Environment::get_home();
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
                                    match mod_entry {
                                        Ok(v) => {
                                            let from = PathBuf::from(v.path());
                                            let mut to = PathBuf::from(&symlink_target);
                                            match from.file_name() {
                                                Some(v) => {
                                                    to.push(v);
                                                    println!("Linking {:?} to {:?}", &from, &to);
                                                    std::os::unix::fs::symlink(from, &to);
                                                }
                                                None => println!("Failed to read file name"),
                                            }
                                        }
                                        Err(e) => println!("Cannot read folder content: {:?}", e),
                                    }
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
