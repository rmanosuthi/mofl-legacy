extern crate chrono;
use crate::mogame::Game;
use crate::momod::chrono::prelude::*;
use gtk::prelude::*;
use gtk::ListStore;
use std::borrow::Borrow;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use ini::Ini;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    enabled: bool,
    load_order: i64,
    label: String,
    version: String,
    category: u64,
    updated: u64,
    nexus_id: i64,
    #[serde(skip)]
    game_path: Rc<PathBuf>,
}

impl Mod {
    /*/// Gets the label of the mod
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
    /// Gets the directory of the mod
    pub fn get_dir(&self) -> &PathBuf {
        return &self.dir;
    }
    /// Sets the directory of the mod
    pub fn set_dir(&mut self, input: PathBuf) -> () {
        self.dir = input;
    }
    /// Gets the favourite status of the mod
    pub fn get_fav(&self) -> bool {
        return self.fav;
    }
    /// Sets the favourite status of the mod
    pub fn set_fav(&mut self, input: bool) {
        self.fav = input;
    }
    /// Gets the last updated time of the mod
    pub fn get_last_updated(&self) -> &DateTime<Local> {
        return &self.last_updated;
    }
    /// Sets the last updated time of the mod to when it's called
    pub fn update(&mut self) -> () {
        self.last_updated = Local::now();
    }*/
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
                        .get::<u64>()
                        .expect("Cannot get value category"),
                    updated: list
                        .get_value(&v, 5)
                        .get::<u64>()
                        .expect("Cannot get value updated"),
                    nexus_id: list
                        .get_value(&v, 6)
                        .get::<i64>()
                        .expect("Cannot get value nexus_id"),
                    game_path: game.path.clone(),
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
                            .get::<u64>()
                            .expect("Cannot get value category"),
                        updated: list
                            .get_value(&v, 5)
                            .get::<u64>()
                            .expect("Cannot get value updated"),
                        nexus_id: list
                            .get_value(&v, 6)
                            .get::<i64>()
                            .expect("Cannot get value nexus_id"),
                        game_path: game.path.clone(),
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
    pub fn from_mo2(game_path: &Rc<PathBuf>, path_from: PathBuf) -> Option<Mod> {
        let mut result = Mod::new(&game_path);
        let mut mo2_ini_path = PathBuf::from(&path_from);
                mo2_ini_path.push("meta.ini");
        match Ini::load_from_file(&mo2_ini_path) {
            Ok(ini) => {
                match ini.section(Some("General")) {
                    Some(v) => {
                        result.enabled = false;
                        result.load_order = -1;
                        result.label = match path_from.file_name() {
                            Some(v) => String::from(v.to_str().unwrap()),
                            None => "UNKNOWN".to_string()
                        };
                        result.version = match v.get("version") {
                            Some(v) => v.to_owned(),
                            None => "9999".to_string()
                        };
                        result.category = match v.get("category") {
                            Some(v) => v.parse::<u64>().unwrap(),
                            None => 0
                        };
                        // don't set result.updated
                        result.nexus_id = match v.get("modid") {
                            Some(v) => v.parse::<i64>().unwrap(),
                            None => -1
                        };
                    },
                    None => ()
                }

            },
            Err(e) => {
                println!("Failed to read MO2 ini");
                return None;
            }
        }
        let mut dest = PathBuf::from(game_path.as_ref());
        dest.push("mods");
        dest.push("");
        dest.push("Data");

        match fs::read_to_string(&path_from) {
            Ok(v) => {}
            Err(e) => {
                println!("Failed to read MO2 ini");
                return None;
            }
        }
        match fs::read_dir(&path_from) {
            Ok(v) => {
                for ref entry in v {
                    match entry {
                        Ok(v) => {
                            dest.push(v.file_name());
                            println!("Copying {:?} to {:?}", v.path(), &dest);
                            //fs::copy(&v.path(), &dest);
                        }
                        Err(e) => (),
                    }
                }
            }
            Err(e) => println!("Failed to import MO2 mod {:?}", &path_from),
        }
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
