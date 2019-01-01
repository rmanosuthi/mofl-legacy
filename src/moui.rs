use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::MenuItemExt;
use gtk::{
    ApplicationWindow, Builder, Button, Dialog, ListStore, Menu, MenuItem, ToolButton, TreeStore, Window, WindowType,
};
use crate::moconfig::Config;
use crate::mogame::Game;
use crate::momod::Mod;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
//use moenv::Environment;
pub const DEFAULT_PATH: &'static str = ".config/mofl";

pub struct UI {
    game: Rc<RefCell<Game>>,
    config: Config,
    builder: Rc<Builder>,
    main_window: Rc<ApplicationWindow>
}
impl UI {
    pub fn new(builder: gtk::Builder) -> UI {
        Config::init_folders();
        let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        tmp_path.push(DEFAULT_PATH);
        tmp_path.push("config.json");
        let config: Config = UI::read_mofl_config(&tmp_path);
        println!("{:?}", &config);
        let game = Rc::new(RefCell::new(UI::read_game_config(&config)));
        println!("{:?}", game);
        game.borrow_mut().add_mods_from_folder();
        println!("1");
        let bt_run_exe: ToolButton = builder.get_object::<ToolButton>("bt-run-exe").unwrap();
        println!("2");
        let handle = game.clone();
        println!("3");
        bt_run_exe.connect_clicked(move |_| {
            handle.borrow_mut().start();
        });
        println!("4");
        UI {
            game: game,
            config: config,
            builder: Rc::new(builder.clone()),
            main_window: Rc::new(builder.get_object("mowindow").unwrap())
        }
    }
    pub fn register_events(&self) {
        let pref_window = self.builder.get_object::<Dialog>("window-preferences").unwrap();
        let window_preferences_bt_close = self.builder.get_object::<Button>("window-preferences-bt-close").unwrap();
        window_preferences_bt_close.connect_clicked(move |_| {
            println!("Closing preferences");
            pref_window.emit_close();
        });
        let exe_edit: MenuItem = self.builder.get_object::<MenuItem>("menu-sel-exe-edit").unwrap();
        exe_edit.connect_activate(move |_| {
            
        });
        //self.game.start();
    }
    pub fn build_ui(&self, application: &gtk::Application) {
        self.register_events();
        &self.main_window.set_application(application);
        &self.main_window.set_title("mofl");
        &self.main_window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });
        &self.main_window.show_all();
        let mod_list: ListStore = self.builder.get_object("liststore-mod-list").unwrap();
        let category_list: ListStore = self.builder.get_object("liststore-mod-categories").unwrap();
        let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        let mut exe_list: ListStore = self.builder.get_object("liststore-runtimes").unwrap();
        let edit_pref: MenuItem = self.builder.get_object("gtk-preferences").unwrap();
        let pref_window = self.builder.get_object::<Dialog>("window-preferences").unwrap();
        pref_window.connect_delete_event(move |win, _| {
            win.hide();
            Inhibit(true)
        });
        edit_pref.connect_activate(move |_| {
            println!("Preferences clicked");
            &pref_window.show();
            //Window::new(WindowType::Toplevel).show();
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
        let menu_exe_list = self.builder.get_object::<Menu>("menu-exe-list").unwrap();
        game.add_exes_to_menu(&menu_exe_list);
        println!("{:?}", &menu_exe_list);
        let a = &game.get_active_executable().unwrap();
        game.set_menu_button(&self.builder.get_object("menu-sel-exe").unwrap());
        game.set_active_executable(&a);
    }
    fn read_mofl_config(tmp_path: &PathBuf) -> Config {
        match fs::read_to_string(tmp_path.as_path()) {
            Ok(v) => serde_json::from_str(&v).unwrap(),
            Err(e) => {
                println!("Creating new config at {}", tmp_path.display());
                let new_config = Config::new();
                fs::write(
                    tmp_path.as_path(),
                    serde_json::to_string(&new_config).unwrap(),
                )
                .unwrap();
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
                fs::write(
                    &game_cfg_path.as_path(),
                    serde_json::to_string(&new_game_config).unwrap(),
                )
                .unwrap();
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
        fs::write(
            &game_cfg_path.as_path(),
            serde_json::to_string(game).unwrap(),
        )
        .unwrap();
    }
}
