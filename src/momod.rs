extern crate chrono;
use gtk::prelude::*;
use gtk::ListStore;
use momod::chrono::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    enabled: bool,
    load_order: i64,
    label: String,
    version: String,
    category: u64,
    updated: u64,
    nexus_id: i64,
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
    }
    /// Creates a new Mod
    pub fn new() -> Mod {
        Mod {
            label: String::from(""),
            load_order: -1,
            nexus_id: -1,
            dir: PathBuf::from(""),
            fav: false,
            last_updated: Local::now()
        }
    }*/
    pub fn from(list: &ListStore) -> Option<Vec<Mod>> {
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
                    });
                }
                return Some(result);
            }
            None => None,
        }
        //list.get_value(list.get_iter_first().unwrap(), 0).get::<String>().unwrap();
    }
    pub fn to(&self, list: &ListStore) {
        list.insert_with_values(None, &[0, 1, 2, 3, 4, 5, 6], &[
            &self.enabled,
            &self.load_order,
            &self.label,
            &self.version,
            &self.category,
            &self.updated,
            &self.nexus_id
        ]);
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
