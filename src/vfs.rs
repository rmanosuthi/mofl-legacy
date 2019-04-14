use std::path::Path;
use std::collections::BTreeMap;
use std::collections::HashMap;
use crate::moenv::Environment;
use crate::mogame::Game;
use crate::moui::DEFAULT_PATH;
use crate::special_game::SpecialGame;
use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn generate_plugins_txt(game: &Game) -> Vec<String> {
    debug!("Arr len {}", game.mods.len());
    let mut result = Vec::new();
    let mut list: BTreeMap<u64, Vec<String>> = BTreeMap::new();
    for m in &game.mods {
        match m.load_order {
            Some(lo) => {
                let mut index: u64 = lo;
                debug!("{}", &lo);
                list.insert(index, Vec::new());
                let mut mod_data_path = m.get_path();
                mod_data_path.push("Data/");
                debug!("Mod data path: {:?}", &mod_data_path);
                for entry in WalkDir::new(mod_data_path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                    match entry.path().extension().unwrap_or(std::ffi::OsStr::new("")).to_str() {
                        Some("esm") => {
                            list.get_mut(&index).unwrap().push(format!("*{:?}", entry.path().file_name().unwrap()));
                        },
                        Some("esp") => {
                            list.get_mut(&index).unwrap().push(format!("*{:?}", entry.path().file_name().unwrap()));
                        },
                        _ => {}
                    }
                }
            },
            None => ()
        }
    }
    for (k, v) in list {
        for m in v {
            result.push(m);
        }
    }
    debug!("{:?}", &result);
    return result;
}

// TODO
// - Traverse to last file since folders don't work well with symlinks, recursion?
// - Check load order before linking, necessary?
/*pub fn generate(game: &Game) -> Option<PathBuf> {
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
                                                    debug!("Linking {:?} to {:?}", &from, &to);
                                                    std::os::unix::fs::symlink(from, &to);
                                                }
                                                None => error!("Failed to read file name"),
                                            }
                                        }
                                        Err(e) => error!("Cannot read folder content: {:?}", e),
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
        Err(e) => println!("Failed to read game dir")
    }
    return None;
}*/

pub fn generate_vfs(game: &Game) -> Result<PathBuf, std::io::Error> {
    match game.special {
        Some(SpecialGame::ESO) => (),
        None => for m in &game.mods {
            for entry in m.get_folders() {
                fs::create_dir_all(entry)?;
            }
            for file in WalkDir::new(m.get_path()).into_iter().filter_map(|e| e.ok()) {
                if file.path().is_file() {
                    std::os::unix::fs::symlink(file.path(), format!("{:?}{:?}", game.path, file.path()))?;
                }
            }
        }
    }
    return Ok(game.path.clone());
}
