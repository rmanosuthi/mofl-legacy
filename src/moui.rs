use crate::mo2;
use crate::moconfig::Config;
use crate::moenv::Environment;
use crate::mogame::Game;
use crate::momod::Mod;
use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::MenuItemExt;
use gtk::{
    ApplicationWindow, Builder, Button, Dialog, ListStore, Menu, MenuItem, MenuToolButton,
    ToolButton, TreeStore, Window, WindowType,
};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
//use moenv::Environment;
pub const DEFAULT_PATH: &'static str = ".config/mofl";

pub struct UI {
    game: Rc<RefCell<Game>>,
    config: Config,
    builder: Rc<Builder>,
    main_window: Rc<ApplicationWindow>,
}
impl UI {
    pub fn new(builder: gtk::Builder) -> UI {
        match Config::init_folders() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        };
        let mut tmp_path = Environment::get_home();
        tmp_path.push(DEFAULT_PATH);
        tmp_path.push("config.json");
        let config: Config = match UI::read_mofl_config(&tmp_path) {
            Some(v) => v,
            None => panic!("Failed to create new config"),
        };
        println!("{:?}", &config);
        let mut tmp_game = match Game::from(&config) {
            Some(v) => v,
            None => panic!("No active game defined"),
        };
        tmp_game.add_mods_from_folder();
        let game = Rc::new(RefCell::new(tmp_game));
        UI {
            game: game,
            config: config,
            builder: Rc::new(builder.clone()),
            main_window: Rc::new(builder.get_object("mowindow").unwrap()),
        }
    }
    pub fn register_events(&self) {
        let pref_window = Rc::new(
            self.builder
                .get_object::<Dialog>("window-preferences")
                .unwrap(),
        );
        let bt_run_exe: ToolButton = self.builder.get_object::<ToolButton>("bt-run-exe").unwrap();

        let window_preferences_bt_close = self
            .builder
            .get_object::<Button>("window-preferences-bt-close")
            .unwrap();
        {
            let pw = pref_window.clone();
            window_preferences_bt_close.connect_clicked(move |_| {
                println!("Closing preferences");
                pw.emit_close();
            });
        }
        let exe_edit: MenuItem = self
            .builder
            .get_object::<MenuItem>("menu-sel-exe-edit")
            .unwrap();
        let edit_pref: MenuItem = self.builder.get_object("gtk-preferences").unwrap();

        exe_edit.connect_activate(move |_| {});
        pref_window.connect_delete_event(move |win, _| {
            win.hide();
            Inhibit(true)
        });
        edit_pref.connect_activate(move |_| {
            println!("Preferences clicked");
            &pref_window.show();
            //Window::new(WindowType::Toplevel).show();
        });
        let handle = self.game.clone();
        bt_run_exe.connect_clicked(move |_| {
            handle.borrow_mut().start();
        });
        //self.game.start();
    }
    pub fn build_ui(&self, application: &gtk::Application) {
        self.register_events();
        self.main_window.set_application(application);
        self.main_window.set_title("Mod Organizer for Linux");
        self.main_window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });
        self.main_window.show_all();
        let category_list: ListStore = self.builder.get_object("liststore-mod-categories").unwrap();
        //let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        //let mut exe_list: ListStore = self.builder.get_object("liststore-runtimes").unwrap();
        self.game.as_ref().borrow().save();
        let menu_exe_list = self.builder.get_object::<Menu>("menu-exe-list").unwrap();
        self.game
            .as_ref()
            .borrow_mut()
            .add_exes_to_menu(&menu_exe_list);
        println!("{:?}", &menu_exe_list);
        self.game.as_ref().borrow_mut().set_menu_button(
            &self
                .builder
                .get_object::<MenuToolButton>("menu-sel-exe")
                .unwrap(),
        );
        self.game.as_ref().borrow().update_active_exe_ui();
        self.game.as_ref().borrow().add_categories_to_view(&category_list);
        let mod_list: ListStore = self.builder.get_object("liststore-mod-list").unwrap();
        for ref _mod in &self.game.as_ref().borrow_mut().mods {
            _mod.to(&mod_list);
        }
    }
    fn read_mofl_config(tmp_path: &PathBuf) -> Option<Config> {
        match fs::read_to_string(tmp_path.as_path()) {
            Ok(v) => match serde_json::from_str(&v) {
                Ok(v) => return v,
                Err(e) => {
                    println!("Failed to deserialize game config: {:?}", e);
                    return None;
                }
            },
            Err(e) => {
                println!("Creating new config at {}", tmp_path.display());
                let new_config = Config::new();
                match serde_json::to_string_pretty(&new_config) {
                    Ok(v) => match fs::write(tmp_path.as_path(), v) {
                        Ok(v) => (),
                        Err(e) => {
                            println!("Failed to write new game config: {:?}", e);
                            return None;
                        }
                    },
                    Err(e) => {
                        println!("Failed to serialize game config: {:?}", e);
                        return None;
                    }
                }
                return Some(new_config);
            }
        }
    }
    // Deprecated - see `game.from(config: &Config) -> Game`
    /*fn read_game_config(config: &Config) -> Game {
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
    }*/
    // Deprecated - see `game.save()`
    /*fn save_game_config(config: &Config, game: &Game) {
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
    }*/
}
