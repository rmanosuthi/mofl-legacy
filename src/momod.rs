extern crate chrono;
use momod::chrono::prelude::*;

pub struct momod {
    label: String,
    load_order: i64,
    nexus_id: i64,
    dir: String,
    fav: bool,
    last_updated: DateTime<Local>
}
impl momod {
    // accessors
    pub fn get_label(&self) -> &String {
        return &self.label;
    }
    pub fn set_label(&mut self, input: String) -> () {
        self.label = input;
    }
    pub fn get_load_order(&self) -> i64 {
        return self.load_order;
    }
    pub fn set_load_order(&mut self, input: i64) -> () {
        self.load_order = input;
    }
    pub fn get_nexus_id(&self) -> i64 {
        return self.nexus_id;
    }
    pub fn set_nexus_id(&mut self, input: i64) -> () {
        self.nexus_id = input;
    }
    pub fn get_dir(&self) -> &String {
        return &self.dir;
    }
    pub fn set_dir(&mut self, input: String) -> () {
        self.dir = input;
    }
    pub fn get_fav(&self) -> bool {
        return self.fav;
    }
    pub fn set_fav(&mut self, input: bool) {
        self.fav = input;
    }
    pub fn get_last_updated(&self) -> &DateTime<Local> {
        return &self.last_updated;
    }
    pub fn update(&mut self) -> () {
        self.last_updated = Local::now();
    }
    pub fn new() -> momod {
    momod {
        label: String::from(""),
        load_order: -1,
        nexus_id: -1,
        dir: String::from(""),
        fav: false,
        last_updated: Local::now()
    }
}
}
impl std::fmt::Display for momod {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        println!("{}", format!("{}{}", "#", &self.load_order));
        println!("{}", format!("{}{}", "Label: ", &self.label));
        println!("{}", format!("{}{}", "Nexus: ", &self.nexus_id));
        println!("{}", format!("{}{}", "Dir: ", &self.dir));
        println!("{}", format!("{}{}", "Last Updated: ", &self.last_updated));
        Ok(())
    }
}