use crate::gamepartial::GameEdit::*;
use crate::gamepartial::GamePartial;
use crate::moconfig::Config;
use crate::moenv::Environment;
use crate::momod::Mod;
use crate::moui::DEFAULT_PATH;
use crate::moui::UI;
use crate::mount::Mount;
use crate::special_game::SpecialGame;
use crate::steam::Steam;
use crate::uihelper::UIHelper;
use crate::vfs;
use crate::wine::Wine;
use crate::wine::WineType;

// traits

use crate::save::Save;

use gtk::prelude::*;
use gtk::Builder;
use gtk::MenuToolButton;
use gtk::TreeIter;
use gtk::TreePath;
use gtk::{ListStore, MenuItem, TreeModelExt};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::rc::Rc;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub label: String,
    #[serde(default = "UIHelper::serde_dialog_text_input")]
    pub steam_label: String,
    pub executables: Vec<Executable>,
    pub active_executable: Option<Executable>, // TODO - use Option<Executable> and handle properly

    #[serde(skip)]
    pub mods: Vec<Mod>,

    pub last_load_order: i64,
    pub categories: Vec<(u64, String)>,
    pub steam_id: i64,
    pub path: PathBuf,
    pub special: Option<SpecialGame>,
    pub wine: Wine,
    pub mount: Mount,

    #[serde(skip)]
    pub menu_button: Option<MenuToolButton>,

    #[serde(skip)]
    pub mofl_game_path: Rc<PathBuf>,

    #[serde(skip)]
    //#[serde(default = "Steam::serde_steam_panic")]
    pub steam: Option<Rc<Steam>>,

    #[serde(skip)]
    pub list_store: Option<Rc<ListStore>>,
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
    pub fn new(
        label: String,
        steam_label: String,
        steam: Rc<Steam>,
        special: Option<SpecialGame>,
        list_store: Rc<ListStore>,
        wine: Wine,
        mount: Mount,
    ) -> Game {
        debug!("New game title: {}", &label);
        let mut path = Environment::get_home();
        path.push(DEFAULT_PATH);
        path.push("games");
        path.push(&label);
        fs::create_dir_all(&path);
        let mut wine_prefix = steam.as_ref().get_game_path(&label);
        wine_prefix.push("pfx");
        Game {
            label: label.clone(),
            steam_label: steam_label,
            executables: Vec::new(),
            active_executable: None,
            mods: Vec::new(),
            wine: wine,
            last_load_order: -1,
            categories: Vec::new(),
            menu_button: None,
            mount: mount,
            mofl_game_path: Rc::new(path),
            steam_id: -1,
            path: steam.as_ref().get_game_path(&label),
            steam: Some(steam),
            special: special,
            list_store: Some(list_store.clone()),
        }
    }
    /// Loads a game from a given configuration.
    /// If given a non-empty value but game folder is empty, create a new one and populate it.
    /// TODO: Game path
    pub fn from(config: &mut Config, list_store: Rc<ListStore>) -> Option<Game> {
        let steam = config.steam.clone();
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
                            v.steam = Some(steam.clone());
                            let mut path = Environment::get_home();
                            path.push(DEFAULT_PATH);
                            path.push("games");
                            path.push(&v.label);
                            debug!("{:?}", &v);
                            let mut wine_prefix = steam.as_ref().get_game_path(&v.steam_label);
                            wine_prefix.push("pfx");
                            v.list_store = Some(list_store);
                            v.mofl_game_path = Rc::new(path);
                            // v.wine_prefix = wine_prefix;
                            if v.path.is_dir() == false {
                                error!("Game path {:?} is either not a directory, is a broken symlink, or you're not allowed to access it", &v.path);
                            }
                            v.save();
                            return Some(v);
                        }
                        Err(e) => {
                            UIHelper::serde_err(game_cfg_path.as_path(), &e);
                            return None;
                        }
                    },
                    Err(e) => {
                        debug!("Creating new game config at {}", &game_cfg_path.display());
                        Config::init_game_folder(&v);
                        /*let new_game_config =
                        Game::new(v.to_string(),
                                  UIHelper::dialog_text_input(
                                      "Please provide the game's Steam name",
                                      &format!("Active game '{}' declared but cannot find configuration.\nThe game's Steam name is needed to proceed.", v.to_string())
                                  ),
                                  config.steam.clone(),
                                  None,
                                  list_store,
                                  Wine; // FIX*/
                        //let new_game_config = UIHelper::prompt_new_game(config.steam.clone(), list_store);
                        let mut new_game_config = UIHelper::prompt_new_game(Some(GamePartial {
                            label: None,
                            special: None,
                            steam_label: None,
                            wine: None,
                            mount: None,
                            steam_id: None,
                        }))
                        .unwrap();
                        new_game_config.steam = Some(config.steam.clone());
                        new_game_config.list_store = Some(list_store);
                        match serde_json::to_string_pretty(&new_game_config) {
                            Ok(v) => match fs::write(&game_cfg_path.as_path(), v) {
                                Ok(v) => (),
                                Err(e) => {
                                    error!("Failed to write new game config: {:?}", e);
                                }
                            },
                            Err(e) => UIHelper::serde_err(game_cfg_path.as_path(), &e),
                        }
                        Some(new_game_config)
                    }
                }
            }
            None => {
                // TODO: Ask for active game
                /*//let game = UIHelper::prompt_new_game(config.steam.clone(), list_store);
                let mut game = UIHelper::prompt_new_game(
                    Some(GamePartial {
                        label: None,
                        special: None,
                        steam_label: None,
                        wine: None,
                        mount: None,
                        steam_id: None
                    }),
                )
                .unwrap();
                game.steam = Some(config.steam.clone());
                game.list_store = Some(list_store);
                config.active_game = Some(game.label.clone());
                return Some(game);*/
                return None;
            }
        }
    }
    pub fn from_game_partial(partial: GamePartial) -> Game {
        /*let label = match partial.label {
            Some(v) => v,
            None => unreachable!()
        };
        let steam_label = match partial.steam_label {
            Some(v) => v,
            None => unreachable!()
        };*/
        return Game {
            label: partial.label.unwrap_or_else(|| UIHelper::dialog_err_gp("label")),
            steam_label: partial.steam_label.unwrap_or_else(|| UIHelper::dialog_err_gp("steam_label")),
            executables: Vec::new(),                 // fix
            active_executable: None,                 // fix
            mods: Vec::new(),
            last_load_order: -1,                     // fix
            categories: Vec::new(),                  // fix
            steam_id: partial.steam_id.unwrap_or_else(|| UIHelper::dialog_err_gp("steam_id")),
            path: PathBuf::new(),                    // fix
            special: partial.special,
            wine: partial.wine.unwrap_or_else(|| UIHelper::dialog_err_gp("wine")),
            mount: Mount::FUSE_OVERLAYFS,            // fix
            menu_button: None,                       // fix?
            mofl_game_path: Rc::new(PathBuf::new()), // fix
            steam: None,                             // fix?
            list_store: None,                        // fix
        };
    }
    fn to_game_partial(&self) -> GamePartial {
        return GamePartial {
            label: Some(self.label.clone()),
            steam_label: Some(self.steam_label.clone()),
            special: self.special.clone(),
            wine: Some(self.wine.clone()),
            mount: Some(self.mount.clone()),
            steam_id: Some(self.steam_id),
        };
    }
    pub fn edit(&mut self) {
        match UIHelper::prompt_edit_game(Some(self.to_game_partial())) {
            Some(v) => self.update(v),
            None => (),
        }
    }
    pub fn add_mod(&mut self) {
        match UIHelper::prompt_install_mod(self.mofl_game_path.clone(), self.list_store.clone()) {
            Some(m) => self.mods.push(m),
            None => (),
        }
    }
    pub fn update(&mut self, data: GamePartial) {
        match data.label {
            Some(v) => self.label = v,
            None => (),
        }
        self.special = data.special;
        match data.steam_label {
            Some(v) => self.steam_label = v,
            None => (),
        }
        match data.wine {
            Some(v) => self.wine = v,
            None => (),
        }
        self.save();
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
    fn compare_treeiter(&self, first: &TreeIter, second: &TreeIter) -> bool {
        let list_store = self.list_store.as_ref().unwrap().clone();
        debug!(
            "First: {:?}",
            list_store.get_string_from_iter(first).unwrap()
        );
        debug!(
            "Second: {:?}",
            list_store.get_string_from_iter(second).unwrap()
        );
        if list_store.get_string_from_iter(first) == list_store.get_string_from_iter(second) {
            return true;
        } else {
            return false;
        }
    }
    pub fn toggle_mod_enable(&mut self, path: TreePath) {
        let mut mod_index: Option<usize> = None;
        let treeiter_path = self.list_store.as_ref().unwrap().get_iter(&path).unwrap();
        for m in &self.mods {
            debug!("Path is {:?}", &treeiter_path);
            debug!("Mod path is {:?}", m.tree_iter.as_ref().unwrap());
            if self.compare_treeiter(m.tree_iter.as_ref().unwrap(), &treeiter_path) {
                info!("Toggling mod {} enabled", &m.get_label());
                mod_index = Some(
                    self.list_store
                        .as_ref()
                        .unwrap()
                        .clone()
                        .get_string_from_iter(&treeiter_path)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                );
            //m.toggle_enabled();
            } else {
                info!("Mod doesn't match");
            }
        }
        self.mods[mod_index.unwrap()].toggle_enabled();
    }
    pub fn update_active_exe_ui(&self) {
        match &self.menu_button {
            Some(ref bt) => match &self.active_executable {
                Some(ref v) => {
                    &bt.set_label(v.label.as_str());
                }
                None => warn!("Cannot get active executable"),
            },
            None => error!("Cannot get menu button"),
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
                        debug!("{:?}", &new_item);
                        &menu.prepend(&new_item);
                        i.set_menu_item(new_item);
                    }
                    None => {
                        warn!("Failed to convert path {:?} to string.", &i.path);
                        warn!("Does it contain non UTF-8 characters?");
                    }
                },
            }
        }
        &menu.show_all(); // IMPORTANT!
    }
    pub fn print_mod_folders(&self) {
        for ref m in &self.mods {
            debug!("{:?}", m.get_folders());
        }
    }
    /// Adds mods from the game folder
    pub fn add_mods_from_folder(&mut self) {
        debug!("Game path: {:?}", self.mofl_game_path.as_ref());
        let mut game_cfg_path: PathBuf = Environment::get_home();
        game_cfg_path.push(DEFAULT_PATH);
        game_cfg_path.push("games");
        game_cfg_path.push(&self.label);
        game_cfg_path.push("mods");
        fs::create_dir_all(&game_cfg_path);
        for entry in WalkDir::new(&game_cfg_path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            debug!("Found mod {:?}", entry.path());
            let mut mod_json: PathBuf = entry.path().to_path_buf();
            mod_json.push("mod.json");
            match self.list_store {
                Some(ref l) => {
                    match Mod::from_path(mod_json.as_path(), self.mofl_game_path.clone(), l.clone())
                    {
                        Some(m) => self.mods.push(m),
                        None => (),
                    }
                }
                None => panic!("Game: list_store missing"),
            }
        }
    }
    /// Updates the base path for the game
    /// This is usually called by Environment
    /*pub fn update_base_path(&mut self, input: PathBuf) -> () {
        self.base_path = input;
    }*/
    /// Imports a mod, taking its path as an argument
    /*pub fn import(&mut self, file: PathBuf) -> bool {
        let new_mod = self.mod_from_archive(file);
        match new_mod {
            Some(v) => {
                self.mods.push(v);
                return true;
            }
            None => return false,
        }
    }*/
    fn get_plugins_txt_path(&self) -> Option<PathBuf> {
        match self.wine.wine_type {
            WineType::PROTON => {
                let mut result = self.wine.prefix.clone();
                result.push("pfx/drive_c/users/steamuser/Local Settings/Application Data/");
                result.push(&self.steam_label);
                result.push("Plugins.txt");
                debug!("Returning plugins txt path for proton {:?}", &result);
                return Some(result);
            }
            _ => return None,
        }
    }
    fn write_plugins_txt(&self) {
        match fs::File::create(self.get_plugins_txt_path().unwrap()) {
            Ok(mut file) => {
                let list = vfs::generate_plugins_txt(&self);
                for m in list {
                    writeln!(file, "{}", m);
                }
            }
            Err(e) => error!("{:?}", e),
        }
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
    pub fn start(&self) -> Option<Child> {
        let mut game_data_path = self.path.clone();
        game_data_path.push("Data/");
        vfs::fuse_overlay_unmount(&game_data_path);
        // check if file exists
        // spawn child process
        match vfs::generate_vfs(&self) {
            Ok(path) => {
                self.write_plugins_txt();
                let mut cmd = self.wine.command(&self);
                match cmd.spawn() {
                    Ok(mut child) => {
                        match child.try_wait() {
                            Ok(Some(status)) => debug!("Exited quickly"),
                            Ok(None) => {
                                let res = child.wait();
                                debug!("Exited");
                                vfs::fuse_overlay_unmount(&game_data_path);
                            }
                            Err(e) => debug!("Process error"),
                        }
                        return Some(child);
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        return None;
                    }
                }
            }
            Err(e) => return None,
        }
    }
    /// stub - Stop a process
    pub fn stop(&self, exe: PathBuf) -> bool {
        // check if file exists
        // stop child process
        return true;
    }
}

impl Save for Game {
    fn save(&self) -> Result<PathBuf, std::io::Error> {
        // TODO - Also save mods
        let mut game_cfg_path: PathBuf = Environment::get_home();
        game_cfg_path.push(DEFAULT_PATH);
        game_cfg_path.push("games");
        game_cfg_path.push(&self.label);
        fs::create_dir_all(&game_cfg_path)?;
        game_cfg_path.push("game.json");
        match serde_json::to_string_pretty(&self) {
            Ok(v) => match fs::write(&game_cfg_path.as_path(), v) {
                Ok(v) => return Ok(game_cfg_path),
                Err(e) => {
                    error!("Failed to write game config: {:?}", &e);
                    return Err(e);
                }
            },
            Err(e) => {
                UIHelper::serde_err(game_cfg_path.as_path(), &e);
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
    }
}
