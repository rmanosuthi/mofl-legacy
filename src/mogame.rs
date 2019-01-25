use crate::moconfig::Config;
use crate::moenv::Environment;
use crate::momod::Mod;
use crate::moui::DEFAULT_PATH;
use crate::moui::UI;
use crate::uihelper::UIHelper;
use crate::steam::Steam;
use crate::vfs;
use gtk::prelude::*;
use gtk::MenuToolButton;
use gtk::{ListStore, MenuItem};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub label: String,
    pub executables: Vec<Executable>,
    active_executable: Option<Executable>, // TODO - use Option<Executable> and handle properly

    #[serde(skip)]
    pub mods: Vec<Mod>,

    pub folder_layout: Vec<PathBuf>,
    pub last_load_order: i64,
    pub categories: Vec<(u64, String)>,
    pub steam_name: String,
    pub steam_id: i64,
    pub path: PathBuf,

    #[serde(skip)]
    menu_button: Option<MenuToolButton>,

    #[serde(skip)]
    pub mofl_game_path: Rc<PathBuf>,

    #[serde(skip)]
    #[serde(default = "Steam::serde_steam_panic")]
    steam: Rc<Steam>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Executable {
    pub label: String,
    pub path: PathBuf,
    pub arguments: String,

    #[serde(skip)]
    menu_item: Option<MenuItem>,
}
impl Executable {
    pub fn set_menu_item(&mut self, item: MenuItem) {
        self.menu_item = Some(item);
    }
}
impl Game {
    /// Creates an empty Game
    pub fn new(label: String, steam: Rc<Steam>) -> Game {
        println!("New game title: {}", &label);
        let mut path = Environment::get_home();
        path.push(DEFAULT_PATH);
        path.push("games");
        path.push(&label);
        fs::create_dir_all(&path);
        Game {
            label: label.clone(),
            executables: Vec::new(),
            active_executable: None,
            mods: Vec::new(),
            folder_layout: Vec::new(),
            last_load_order: -1,
            categories: Vec::new(),
            menu_button: None,
            mofl_game_path: Rc::new(path),
            steam_name: label.clone(),
            steam_id: -1,
            path: steam.as_ref().get_game_path(label),
            steam: steam
        }
    }
    /// Loads a game from a given configuration.
    /// If given a non-empty value but game folder is empty, create a new one and populate it.
    /// TODO: Game path
    pub fn from(config: &Config, steam: Rc<Steam>) -> Option<Game> {
        match config.get_active_game() {
            Some(v) => {
                let mut game_cfg_path: PathBuf = Environment::get_home();
                game_cfg_path.push(DEFAULT_PATH);
                game_cfg_path.push("games");
                game_cfg_path.push(&v);
                game_cfg_path.push("game.json");
                match fs::read_to_string(&game_cfg_path.as_path()) {
                    Ok(v) => match serde_json::from_str(&v) {
                        Ok(v) => {
                            let mut v: Game = v;
                            let mut path = Environment::get_home();
                            path.push(DEFAULT_PATH);
                            path.push("games");
                            path.push(&v.label);
                            v.mofl_game_path = Rc::new(path);
                            return Some(v);
                        }
                        Err(e) => {
                            UIHelper::serde_err(&e);
                            return None;
                        }
                    },
                    Err(e) => {
                        println!("Creating new game config at {}", &game_cfg_path.display());
                        Config::init_game_folder(&v);
                        let new_game_config = Game::new(v.to_string(), steam.clone());
                        match serde_json::to_string_pretty(&new_game_config) {
                            Ok(v) => match fs::write(&game_cfg_path.as_path(), v) {
                                Ok(v) => (),
                                Err(e) => {
                                    println!("Failed to write new game config: {:?}", e);
                                }
                            },
                            Err(e) => {
                                println!("Failed to serialize game to config: {:?}", e);
                            }
                        }
                        Some(new_game_config)
                    }
                }
            }
            None => {
                println!("No active game in config");
                None
            }
        }
    }
    pub fn save(&self) -> () {
        // TODO - Also save mods
        let mut game_cfg_path: PathBuf = Environment::get_home();
        game_cfg_path.push(DEFAULT_PATH);
        game_cfg_path.push("games");
        game_cfg_path.push(&self.label);
        game_cfg_path.push("game.json");
        match serde_json::to_string_pretty(&self) {
            Ok(v) => match fs::write(&game_cfg_path.as_path(), v) {
                Ok(v) => (),
                Err(e) => {
                    println!("Failed to write new game config: {:?}", e);
                }
            },
            Err(e) => {
                println!("Failed to serialize game to config: {:?}", e);
            }
        }
    }
    pub fn save_all(&self) {
        self.save();
        for ref e_mod in &self.mods {
            e_mod.save();
        }
    }
    pub fn get_active_executable(&self) -> &Option<Executable> {
        return &self.active_executable;
    }
    pub fn set_active_executable(&mut self, exe: Executable) {
        self.active_executable = Some(exe);
        self.update_active_exe_ui();
    }
    pub fn set_menu_button(&mut self, button: &MenuToolButton) {
        self.menu_button = Some(button.clone());
    }
    pub fn update_active_exe_ui(&self) {
        match &self.menu_button {
            Some(ref bt) => match &self.active_executable {
                Some(ref v) => {
                    &bt.set_label(v.label.as_str());
                }
                None => println!("Cannot get active executable"),
            },
            None => println!("Cannot get menu button"),
        }
    }
    pub fn add_categories_to_view(&self, list: &ListStore) {
        for ref category in &self.categories {
            list.insert_with_values(None, &[0], &[&category.1]);
        }
    }
    pub fn add_exes_to_menu(&mut self, menu: &gtk::Menu) {
        for i in self.executables.iter_mut() {
            match i.menu_item {
                Some(ref v) => {}
                None => match i.path.to_str() {
                    Some(v) => {
                        let new_item = gtk::MenuItem::new_with_label(v);
                        println!("{:?}", &new_item);
                        &menu.prepend(&new_item);
                        i.set_menu_item(new_item);
                    }
                    None => {
                        println!("Failed to convert path to string.");
                        println!("Does it contain non UTF-8 characters?");
                    }
                },
            }
        }
        &menu.show_all(); // IMPORTANT!
    }
    pub fn print_mod_folders(&self) {
        for ref m in &self.mods {
            println!("{:?}", m.get_folders());
        }
    }
    /// Adds mods from the game folder
    pub fn add_mods_from_folder(&mut self) {
        println!("Game path: {:?}", self.mofl_game_path.as_ref());
        let mut game_cfg_path: PathBuf = Environment::get_home();
        game_cfg_path.push(DEFAULT_PATH);
        game_cfg_path.push("games");
        game_cfg_path.push(&self.label);
        game_cfg_path.push("mods");
        fs::create_dir_all(&game_cfg_path);
        for entry in WalkDir::new(&game_cfg_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let mut mod_json: PathBuf = entry.path().to_path_buf();
            mod_json.push("mod.json");
            match fs::read_to_string(&mod_json.as_path()) {
                Ok(v) => match serde_json::from_str(&v) {
                    Ok(v) => {
                        let mut v: Mod = v;
                        v.game_path = self.mofl_game_path.clone();
                        self.mods.push(v);
                    }
                    Err(e) => println!("Failed to deserialize game config: {:?}", e),
                },
                Err(e) => println!("Failed to read mod.json: {:?}", e),
            }
        }
    }
    /// Updates the base path for the game
    /// This is usually called by Environment
    /*pub fn update_base_path(&mut self, input: PathBuf) -> () {
        self.base_path = input;
    }*/
    /// Imports a mod, taking its path as an argument
    pub fn import(&mut self, file: PathBuf) -> bool {
        let new_mod = self.mod_from_archive(file);
        match new_mod {
            Some(v) => {
                self.mods.push(v);
                return true;
            }
            None => return false,
        }
    }
    fn mod_from_archive(&self, file: PathBuf) -> Option<Mod> {
        // TODO: better validation, update to conform with new structure
        if file.is_file() == false {
            return None;
        }
        // file must exist
        let mut result: Mod = match file.file_name() {
            Some(v) => {
                let mut new_mod = Mod::new(&self.mofl_game_path);
                new_mod.set_label(v.to_str().unwrap().to_string());
                new_mod
            }
            None => return None,
        };
        // extract archive
        let label = result.get_label().to_owned();
        let mut path = PathBuf::from(self.mofl_game_path.as_ref());
        path.push("mods");
        path.push(&self.gen_uuid().to_string());
        let cmd = Command::new("7z")
            .current_dir(path)
            .arg("x")
            .arg(
                file.canonicalize()
                    .expect("Cannot convert file path into absolute path"),
            )
            .arg("-o".to_owned() + "Data/")
            .output()
            .expect("Extract failed");
        println!("{:?}", cmd.stdout);
        return Some(result);
    }
    fn gen_uuid(&self) -> u64 {
        return 0;
    }
    /*fn sanitize(&self, input: Mod) -> bool {
        // holy error handling Batman!
        for entry in fs::read_dir(input.get_dir()).expect("Cannot read mod dir") {
            let entry: fs::DirEntry = entry.expect("Also cannot read dir");
            for str_comp in &self.folder_layout {
                if entry.metadata().expect("Cannot read metadata").is_dir() == true {
                    let entry_file_name = entry.file_name();
                    let entry_name = entry_file_name
                        .to_str()
                        .expect("Cannot convert file name into string");
                    //.to_str().expect("Cannot convert file name into string");
                    if entry_name == str_comp.to_str().expect("") {
                        fs::rename(entry_name, str_comp).expect("Cannot rename folder");
                    }
                }
            }
        }
        return true;
    }*/
    /// stub - Validates mod
    fn check_sanity(input: Mod) -> bool {
        return true;
    }
    /// stub - Start a process
    pub fn start(&self) -> bool {
        println!("Mounting...");
        // check if file exists
        // spawn child process
        vfs::generate(&self);
        return true;
    }
    /// stub - Stop a process
    pub fn stop(&self, exe: PathBuf) -> bool {
        // check if file exists
        // stop child process
        return true;
    }
}
