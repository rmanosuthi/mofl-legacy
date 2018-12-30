use std::path::PathBuf;
use std::fs;
use std::env;
use gio;
use gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, MenuItem, ListStore, TreeStore, Window, WindowType};
use gtk::MenuItemExt;
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
        let mod_list: ListStore = builder.get_object("treestore-mod-list").unwrap();
        let category_list: ListStore = builder.get_object("treestore-mod-categories").unwrap();
        let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        let mut exe_list: ListStore = builder.get_object("liststore-runtimes").unwrap();
        let edit_pref: MenuItem = builder.get_object("gtk-preferences").unwrap();
        edit_pref.connect_activate_item(move |_| {
            /*let pref_window = builder.get_object::<Window>("window-preferences").unwrap();
            pref_window.show_all();
            pref_window.run();*/
            Window::new(WindowType::Toplevel).show();
        });
        tmp_path.push(DEFAULT_PATH);
        tmp_path.push("config.json");
        let config: Config = UI::read_mofl_config(&tmp_path);
        println!("{:?}", &config);
        let mut game = UI::read_game_config(&config);
        println!("{:?}", game);
        game.add_mods_from_folder();
        for ref _mod in &game.mods {
            _mod.to(&mod_list);
        }
        game.add_categories_to_view(&category_list);
        println!("{:?}", game);
        config.to(&mut exe_list);
        UI::save_game_config(&config, &game);
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
    fn read_game_config(config: &Config) -> Game {
        let mut game_cfg_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        game_cfg_path.push(DEFAULT_PATH);
        game_cfg_path.push("games");
        game_cfg_path.push(config.get_active_game());
        game_cfg_path.push("game.json");
        match fs::read_to_string(&game_cfg_path.as_path()) {
            Ok(v) => serde_json::from_str(&v).unwrap(),
            Err(e) => {
                println!("Creating new game config at {}", &game_cfg_path.display());
                Config::init_game_folder(config.get_active_game());
                let new_game_config = Game::new(config.get_active_game().to_owned());
                fs::write(&game_cfg_path.as_path(), serde_json::to_string(&new_game_config).unwrap()).unwrap();
                new_game_config
            }
        }
    }
    fn save_game_config(config: &Config, game: &Game) {
        let mut game_cfg_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        game_cfg_path.push(DEFAULT_PATH);
        game_cfg_path.push("games");
        game_cfg_path.push(config.get_active_game());
        game_cfg_path.push("game.json");
        fs::write(&game_cfg_path.as_path(), serde_json::to_string(game).unwrap()).unwrap();
    }
}
