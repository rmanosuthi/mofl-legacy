use gtk::Builder;
use gtk::ListStore;
use crate::game::partial::GameEdit::*;
use crate::game::partial::GamePartial;
use crate::game::GameModel;
use crate::game::momod::Mod;
use crate::game::momod::ModModel;
use crate::steam::Steam;
use crate::uihelper::UIHelper;
use crate::worker::WorkerReply;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use walkdir::WalkDir;

// FIXME
/// stub - Given an MO2 game folder, create a populated MOFL game folder and return a Game struct
/*pub fn import(path: PathBuf, steam: Rc<Steam>) -> Option<GameModel> {
    if path.is_dir() {
        let game_name = match path.file_name() {
            Some(v) => match v.to_str() {
                Some(v) => v,
                None => {
                    warn!("Failed importing an MO2 installation: Invalid folder name (most likely cause is non UTF-8 characters)");
                    debug!("Path is {:?}", &path);
                    return None;
                }
            },
            None => {
                warn!("Failed importing an MO2 installation: Invalid path");
                debug!("Path is {:?}", &path);
                return None;
            }
        };
        // assume Creation Engine game since MO2 only supports those
        //let mut game = Game::new(String::from(game_name), String::new(), steam, None, list_store.clone());
        let mut game = UIHelper::prompt_new_game().unwrap();
        let mut path = PathBuf::from(&path);
        path.push("mods");
        for entry in WalkDir::new(&path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            debug!("Received path {:?}", entry.path());
            /*match Mod::from_mo2(game.mofl_game_path.clone(), PathBuf::from(entry.path()), list_store.clone()) {
                Some(v) => {
                    debug!("Adding mod {:?}", &v);
                    game.mods.push(v);
                }
                None => (),
            }*/
        }
        return Some(game);
    } else {
        warn!("Failed importing an MO2 installation...");
        debug!("Path is {:?}", &path);
        warn!("Possible causes:");
        warn!("- Path is not a directory");
        warn!("- Broken symlink");
        warn!("- Access denied");
        return None;
    }
}*/

pub fn worker_import(game_name: &str, path: &Path, send_to_relm: relm::Sender<WorkerReply>) {
    send_to_relm.send(WorkerReply::ImportMo2Start);
    for entry in WalkDir::new(&path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        if let Some(m) = ModModel::from_mo2(game_name, entry.path()) {
            send_to_relm.send(WorkerReply::ImportMo2(m));
        }
    }
    send_to_relm.send(WorkerReply::ImportMo2Done);
}