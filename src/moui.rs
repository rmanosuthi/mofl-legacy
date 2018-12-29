use std::path::PathBuf;
use gio;
use gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, ListStore, TreeStore, Window, WindowType};
use momod::Mod;
use mogame::Game;
use moconfig::Config;
//use moenv::Environment;
static DEFAULT_PATH: &str = "~/.config/mofl";
pub struct UI {
    game: Game
}
impl UI {
    pub fn build_ui(application: &gtk::Application) {
        let glade_src = include_str!("window.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: ApplicationWindow = builder
            .get_object("mowindow")
            .expect("Couldn't get mowindow");
        window.set_application(application);
        window.set_title("mofl - Skyrim SE");
        window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });
        window.show_all();
        let mut tmp_path: PathBuf = PathBuf::from(DEFAULT_PATH);
        tmp_path.push("config.json");
        let config: Config = serde_json::from_str(&std::fs::read_to_string(&tmp_path.as_path()).unwrap()).unwrap();
        /*let mod_vec = super::momod::Mod::from(&builder.get_object::<ListStore>("treestore-mod-list").expect("Cannot load object")).expect("from return failed");
        //println!("{}", mod_vec.get(0).unwrap());

        let serialized = serde_json::to_string(&mod_vec).unwrap();
        println!("serialized = {}", serialized);
        let deserialized: Vec<Mod> = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);

        let list = &builder.get_object::<ListStore>("treestore-mod-list").expect("Cannot load object");
        list.clear();
        //println!("{}", serde_json::to_string(&super::momod::Mod::from(list).unwrap()).unwrap());
        for ref m in deserialized {
            m.to(list);
        }
        println!("{}", serde_json::to_string(&super::momod::Mod::from(list).unwrap()).unwrap());*/
    }
}
