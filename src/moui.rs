use std::path::PathBuf;
use std::fs;
use std::env;
use gio;
use gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, ListStore, TreeStore, Window, WindowType};
use momod::Mod;
use mogame::Game;
use moconfig::Config;
//use moenv::Environment;
pub const DEFAULT_PATH: &'static str = ".config/mofl";
pub struct UI {
    game: Game,
    config: Config
}
impl UI {
    pub fn build_ui(application: &gtk::Application) {
        Config::init_folders();
        let glade_src = include_str!("window.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: ApplicationWindow = builder
            .get_object("mowindow")
            .expect("Couldn't get mowindow");
        window.set_application(application);
        window.set_title("mofl");
        window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });
        window.show_all();
        let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        tmp_path.push(DEFAULT_PATH);
        tmp_path.push("config.json");
        let config: Config = UI::read_mofl_config(&tmp_path);
        println!("{:?}", &config);
        /*
        match  {
            Ok(v) => {
                serde_json::from_str(&std::fs::read_to_string(&tmp_path.as_path()).unwrap())
            },
            Err(e) => {
                return Config::new();
            }
        };
        */
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
    fn read_mofl_config(tmp_path: &PathBuf) -> Config {
        match fs::read_to_string(tmp_path.as_path()) {
            Ok(v) => serde_json::from_str(&v).unwrap(),
            Err(e) => {
                println!("Creating new config at {}", tmp_path.display());
                let new_config = Config::new();
                fs::write(tmp_path.as_path(), serde_json::to_string(&new_config).unwrap()).unwrap();
                new_config
            }
        }
    }
    fn active_game_from_config(config: Config) -> Result<Game, std::io::Error> {
        let mut game_cfg_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap()); 
        game_cfg_path.push("games");
        game_cfg_path.push(config.get_active_game());
        game_cfg_path.push("game.json");
        match fs::read_to_string(&game_cfg_path.as_path()) {
            Ok(v) => Ok(serde_json::from_str(&v).unwrap()),
            Err(e) => Err(e)
        }
    }
}
