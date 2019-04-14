extern crate chrono;
use crate::mogame::Game;
use crate::momod::chrono::prelude::*;
use crate::steam::Steam;
use crate::uihelper::UIHelper;
use gtk::prelude::*;
use gtk::ListStore;
use gtk::TreeIter;
use ini::Ini;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    enabled: bool,
    pub load_order: Option<u64>,
    label: String,
    version: String,
    category: i64,
    updated: u64,
    nexus_id: i64,
    #[serde(skip)]
    pub game_path: Rc<PathBuf>,
    #[serde(skip)]
    pub list_store: Option<Rc<ListStore>>,
    #[serde(skip)]
    pub tree_iter: Option<TreeIter>
}

impl Mod {
    fn on_update(&self) {
        match &self.list_store {
            Some(v) => match &self.tree_iter {
                Some(t) => v.set(
                    t,
                    &[0, 1, 2, 3, 4, 5, 6],
                    &[
                        &self.enabled,
                        &self.load_order.as_ref().unwrap_or(&0), // FIX
                        &self.label,
                        &self.version,
                        &self.category,
                        &self.updated,
                        &self.nexus_id,
                    ],
                ),
                None => return,
            },
            None => return,
        }
    }
    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
        self.on_update();
        self.save();
    }
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.on_update();
    }
    /// Gets the label of the mod
    pub fn get_label(&self) -> &String {
        return &self.label;
    }
    /// Sets the label of the mod
    pub fn set_label(&mut self, input: String) -> () {
        self.label = input;
    }
    /// Gets the load order of the mod
    pub fn get_load_order(&self) -> Option<u64> {
        return self.load_order;
    }
    /// Sets the load order of the mod
    pub fn set_load_order(&mut self, input: Option<u64>) -> () {
        self.load_order = input;
    }
    /// Gets the Nexus ID of the mod
    pub fn get_nexus_id(&self) -> i64 {
        return self.nexus_id;
    }
    /// Sets the Nexus ID of the mod
    pub fn set_nexus_id(&mut self, input: i64) -> () {
        self.nexus_id = input;
    }
    /// Gets the last updated time of the mod
    pub fn get_updated(&self) -> u64 {
        return self.updated;
    }
    /// Gets the last updated time of the mod
    pub fn set_updated(&mut self, input: u64) {
        self.updated = input;
    }
    pub fn get_path(&self) -> PathBuf {
        let mut path = self.game_path.as_ref().clone();
        path.push("mods/");
        path.push(self.nexus_id.to_string());
        return path;
    }
    /// Creates a new Mod
    pub fn new(game_path: Rc<PathBuf>, list_store: Rc<ListStore>) -> Mod {
        let mut new_mod = Mod {
            enabled: false,
            load_order: None,
            label: String::new(),
            version: String::from("9999"),
            category: 0,
            updated: 0,
            nexus_id: -1,
            game_path: game_path,
            list_store: None,
            tree_iter: None,
        };
        let t = list_store.insert_with_values(
            None,
            &[0, 1, 2, 3, 4, 5, 6],
            &[
                &new_mod.enabled,
                &new_mod.load_order.as_ref().unwrap_or(&0), // FIX,
                &new_mod.label,
                &new_mod.version,
                &new_mod.category,
                &new_mod.updated,
                &new_mod.nexus_id,
            ],
        );
        new_mod.list_store = Some(list_store);
        new_mod.tree_iter = Some(t);
        return new_mod;
    }
    pub fn from_path(
        mod_cfg_path: &Path,
        game_path: Rc<PathBuf>,
        list_store: Rc<ListStore>,
    ) -> Option<Mod> {
        match fs::read_to_string(mod_cfg_path) {
            Ok(v) => match serde_json::from_str(&v) {
                Ok(v) => {
                    let mut v: Mod = v;
                    let t = list_store.insert_with_values(
                        None,
                        &[0, 1, 2, 3, 4, 5, 6],
                        &[
                            &v.enabled,
                            &v.load_order.as_ref().unwrap_or(&0), // FIX,
                            &v.label,
                            &v.version,
                            &v.category,
                            &v.updated,
                            &v.nexus_id,
                        ],
                    );
                    v.list_store = Some(list_store);
                    v.tree_iter = Some(t);
                    v.game_path = game_path;
                    return Some(v);
                }
                Err(e) => {
                    UIHelper::serde_err(mod_cfg_path, &e);
                    return None;
                }
            },
            Err(e) => {
                error!("Failed to read mod.json: {:?}", e);
                return None;
            }
        }
    }
    /*pub fn from(list: &ListStore, game: &Game) -> Option<Vec<Mod>> {
        match list.get_iter_first() {
            Some(v) => {
                let mut result: Vec<Mod> = Vec::new();
                result.push(Mod {
                    enabled: list
                        .get_value(&v, 0)
                        .get::<bool>()
                        .expect("Cannot get value enabled"),
                    load_order: list
                        .get_value(&v, 1)
                        .get::<i64>()
                        .expect("Cannot get value load_order"),
                    label: list
                        .get_value(&v, 2)
                        .get::<String>()
                        .expect("Cannot get value label"),
                    version: list
                        .get_value(&v, 3)
                        .get::<String>()
                        .expect("Cannot get value version"),
                    category: list
                        .get_value(&v, 4)
                        .get::<i64>()
                        .expect("Cannot get value category"),
                    updated: list
                        .get_value(&v, 5)
                        .get::<u64>()
                        .expect("Cannot get value updated"),
                    nexus_id: list
                        .get_value(&v, 6)
                        .get::<i64>()
                        .expect("Cannot get value nexus_id"),
                    game_path: game.mofl_game_path.clone(),
                });
                while list.iter_next(&v) == true {
                    result.push(Mod {
                        enabled: list
                            .get_value(&v, 0)
                            .get::<bool>()
                            .expect("Cannot get value enabled"),
                        load_order: list
                            .get_value(&v, 1)
                            .get::<i64>()
                            .expect("Cannot get value load_order"),
                        label: list
                            .get_value(&v, 2)
                            .get::<String>()
                            .expect("Cannot get value label"),
                        version: list
                            .get_value(&v, 3)
                            .get::<String>()
                            .expect("Cannot get value version"),
                        category: list
                            .get_value(&v, 4)
                            .get::<i64>()
                            .expect("Cannot get value category"),
                        updated: list
                            .get_value(&v, 5)
                            .get::<u64>()
                            .expect("Cannot get value updated"),
                        nexus_id: list
                            .get_value(&v, 6)
                            .get::<i64>()
                            .expect("Cannot get value nexus_id"),
                        game_path: game.mofl_game_path.clone(),
                    });
                }
                return Some(result);
            }
            None => None,
        }
        //list.get_value(list.get_iter_first().unwrap(), 0).get::<String>().unwrap();
    }
    pub fn to(&self, list: &ListStore) {
        list.insert_with_values(
            None,
            &[0, 1, 2, 3, 4, 5, 6],
            &[
                &self.enabled,
                &self.load_order,
                &self.label,
                &self.version,
                &self.category,
                &self.updated,
                &self.nexus_id,
            ],
        );
    }*/
    pub fn save(&self) {
        let mut dest = self.get_mod_dir();
        dest.push("mod.json");
        match serde_json::to_string_pretty(&self) {
            Ok(v) => match fs::write(dest.as_path(), v) {
                Ok(v) => (),
                Err(e) => {
                    error!("Failed to write new game config: {:?}", e);
                }
            },
            Err(e) => UIHelper::serde_err(dest.as_path(), &e),
        }
    }
    pub fn get_mod_dir(&self) -> PathBuf {
        let mut dest = PathBuf::from(self.game_path.as_ref());
        dest.push("mods");
        if self.nexus_id == 0 {
            dest.push("unknown-id");
            dest.push(&self.label);
        } else {
            dest.push(self.nexus_id.to_string());
        }
        debug!("Mod dir queried, returning {:?}", &dest);
        return dest;
    }
    pub fn get_folders(&self) -> Vec<PathBuf> {
        let mut result: Vec<PathBuf> = Vec::new();
        let mut src = self.get_mod_dir();
        src.push("Data/");
        self.recursive_get_folders(src, &mut result, false);
        return result;
    }
    fn recursive_get_folders(&self, path: PathBuf, list: &mut Vec<PathBuf>, absolute: bool) {
        debug!("Received {:?}", &path);
        if path.is_dir() {
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                let e_path = entry.path();
                if e_path.is_dir() && e_path != path {
                    debug!("Adding {:?}", e_path.clone());
                    if absolute == true {
                        list.push(e_path.to_path_buf());
                    } else {
                        let new_path_tmp: &str = e_path.to_str().unwrap();
                        list.push(PathBuf::from(
                            new_path_tmp
                                .split_at(self.get_path().to_str().unwrap().len() + 1)
                                .1,
                        ));
                    }
                }
            }
        } else {
            error!("Path {:?} is not a dir!", &path);
        }
    }
    pub fn from_mo2(
        game_path: Rc<PathBuf>,
        path_from: PathBuf,
        list_store: Rc<ListStore>,
    ) -> Option<Mod> {
        let mut result = Mod::new(game_path.clone(), list_store);
        let mut mo2_ini_path = PathBuf::from(&path_from);
        mo2_ini_path.push("meta.ini");
        match Ini::load_from_file_noescape(&mo2_ini_path) {
            Ok(ini) => {
                match ini.section(Some("General")) {
                    Some(v) => {
                        result.enabled = false;
                        result.load_order = None;
                        match path_from.file_name() {
                            Some(v) => match v.to_str() {
                                Some(v) => {
                                    info!("Importing mod {}", &v);
                                    result.label = String::from(v);
                                }
                                None => {
                                    error!("Failed to convert path {:?} to string.", &v);
                                    error!("Does it contain non UTF-8 characters?");
                                    return None; // Label is necessary, so return none if there's none
                                }
                            },
                            None => (),
                        };
                        match v.get("version") {
                            Some(v) => result.version = v.to_owned(),
                            None => (),
                        };
                        match v.get("category") {
                            Some(v) => match v.replace(",", "").parse::<i64>() {
                                Ok(v) => result.category = v,
                                Err(e) => {
                                    error!("Failed to parse category: {:?}", e);
                                }
                            },
                            None => (),
                        };
                        // don't set result.updated
                        match v.get("modid") {
                            Some(v) => match v.parse::<i64>() {
                                Ok(v) => result.nexus_id = v,
                                Err(e) => {
                                    error!("Failed to parse Nexus ID: {:?}", e);
                                }
                            },
                            None => (),
                        };
                        for entry in WalkDir::new(&path_from).into_iter().filter_map(|e| e.ok()) {
                            let mut dest = PathBuf::from(game_path.as_path());
                            dest.push("mods");
                            if result.nexus_id == 0 {
                                dest.push("unknown-id");
                                dest.push(&result.label);
                            } else {
                                dest.push(result.nexus_id.to_string());
                            }
                            dest.push("Data");
                            fs::create_dir_all(&dest);
                            dest.push(entry.file_name());
                            debug!("Copying {:?} to {:?}", entry.path().to_path_buf(), &dest);
                            //fs::copy(&v.path(), &dest);
                        }
                    }
                    None => (),
                }
            }
            Err(e) => {
                error!("Failed to read MO2 ini {:?}", &e);
                return None;
            }
        }
        debug!(">>> returning something");
        Some(result)
    }
}

impl Drop for Mod {
    fn drop(&mut self) {
        match self.list_store {
            Some(ref l) => match self.tree_iter {
                Some(ref t) => {
                    l.remove(t);
                }
                None => (),
            },
            None => (),
        }
    }
}
impl std::fmt::Display for Mod {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        println!("{}", format!("{}{}", "~: ", self.enabled));
        println!("{}", format!("{}{}", "#: ", self.load_order.as_ref().unwrap_or(&0))); // FIX
        println!("{}", format!("{}{}", "Label: ", self.label));
        println!("{}", format!("{}{}", "Version: ", self.version));
        println!("{}", format!("{}{}", "Category: ", self.category));
        println!("{}", format!("{}{}", "Updated: ", self.updated));
        println!("{}", format!("{}{}", "Nexus: ", self.nexus_id));
        Ok(())
    }
}
