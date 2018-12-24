extern crate chrono;
use momod::chrono::prelude::*;
use std::path::PathBuf;

pub struct Mod {
    label: String,
    load_order: i64,
    nexus_id: i64,
    dir: PathBuf,
    fav: bool,
    last_updated: DateTime<Local>
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
}
}
impl std::fmt::Display for Mod {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        println!("{}", format!("{}{}", "#", &self.load_order));
        println!("{}", format!("{}{}", "Label: ", &self.label));
        println!("{}", format!("{}{}", "Nexus: ", &self.nexus_id));
        match &self.dir.to_str(){
            Some(v) => println!("{}", format!("{}{}", "Dir: ", v)),
            None => {}
        }
        println!("{}", format!("{}{}", "Last Updated: ", &self.last_updated));
        Ok(())
    }
}