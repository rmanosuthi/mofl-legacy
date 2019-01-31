extern crate chrono;
use crate::mogame::Game;
use crate::momod::chrono::prelude::*;
use crate::steam::Steam;
use crate::uihelper::UIHelper;
use gtk::prelude::*;
use gtk::ListStore;
use ini::Ini;
use std::borrow::Borrow;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    enabled: bool,
    load_order: i64,
    label: String,
    version: String,
    category: i64,
    updated: u64,
    nexus_id: i64,
    #[serde(skip)]
    pub game_path: Rc<PathBuf>,
}

impl Mod {
    /// Gets the label of the mod
    pub fn get_label(&self) -> &String {
        return &self.label;
    }
    /// Sets the label of the mod
    pub fn set_label(&mut self, input: String) -> () {
        self.label = input;
    }
    /// Gets the load order of the mod
    pub fn get_load_order(&self) -> i64 {
        return self.load_order;
    }
    /// Sets the load order of the mod
    pub fn set_load_order(&mut self, input: i64) -> () {
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
    /// Creates a new Mod
    pub fn new(game_path: &Rc<PathBuf>) -> Mod {
        Mod {
            enabled: false,
            load_order: -1,
            label: String::new(),
            version: String::from("9999"),
            category: 0,
            updated: 0,
            nexus_id: -1,
            game_path: game_path.clone(),
        }
    }
    pub fn from(list: &ListStore, game: &Game) -> Option<Vec<Mod>> {
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
    }
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
            Err(e) => UIHelper::serde_err(&e)
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
                debug!("Entry");
                if e_path.is_dir() && e_path != path {
                    debug!("Adding {:?}", e_path.clone());
                    if absolute == true {
                        list.push(e_path.to_path_buf());
                    } else {
                        let new_path_tmp: &str = e_path.to_str().unwrap();
                        list.push(PathBuf::from(
                            new_path_tmp
                                .split_at(self.get_mod_dir().to_str().unwrap().len() + 1)
                                .1,
                        ));
                    }
                }
            }
        } else {
            error!("Path {:?} is not a dir!", &path);
        }
    }
    pub fn from_mo2(game_path: &Rc<PathBuf>, path_from: PathBuf) -> Option<Mod> {
        let mut result = Mod::new(&game_path);
        let mut mo2_ini_path = PathBuf::from(&path_from);
        mo2_ini_path.push("meta.ini");
        match Ini::load_from_file_noescape(&mo2_ini_path) {
            Ok(ini) => {
                match ini.section(Some("General")) {
                    Some(v) => {
                        result.enabled = false;
                        result.load_order = -1;
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
                            let mut dest = PathBuf::from(game_path.as_ref());
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
impl std::fmt::Display for Mod {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        println!("{}", format!("{}{}", "~: ", self.enabled));
        println!("{}", format!("{}{}", "#: ", self.load_order));
        println!("{}", format!("{}{}", "Label: ", self.label));
        println!("{}", format!("{}{}", "Version: ", self.version));
        println!("{}", format!("{}{}", "Category: ", self.category));
        println!("{}", format!("{}{}", "Updated: ", self.updated));
        println!("{}", format!("{}{}", "Nexus: ", self.nexus_id));
        Ok(())
    }
}
