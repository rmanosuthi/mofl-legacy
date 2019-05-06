use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, ListStore, ToolButton, TreeIter};
use relm::{execute, init, Component, ContainerWidget, EventStream, Relm, Update, Widget};

use crate::executable::{Executable, ExecutableModel, ExecutableMsg};
use crate::executablemanager::ExecutableManager;
use crate::load::Load;
use crate::moenv::Environment;
use crate::momod::Mod;
use crate::mount::Mount;
use crate::special_game::SpecialGame;
use crate::uihelper::UIHelper;
use crate::vfs;
use crate::wine::Wine;
use crate::wine::WineType;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Msg)]
pub enum Msg {
    Init,
    Modify(GameModel),
    AddMod,
    RemoveMod(TreeIter),
    UpdateMod(TreeIter),
    EditMod(TreeIter),
    ImportMo2(PathBuf),
    EditExes,
    Start,
    Stop,
    Quit,
}

#[derive(Serialize, Deserialize)]
pub struct GameModel {
    pub label: String,
    pub steam_label: String,
    pub path: PathBuf, // this is the path to the game's Program Files itself
    // (i.e. "C:\Program Files (x86)\Fallout 3\" on Windows or "~/.steam/steam/steamapps/common/Fallout 3/" on Linux)

    #[serde(skip)]
    pub mods: HashMap<TreeIter, Mod>, // don't make Mod composited because of GTK's stupid way of doing lists

    pub steam_id: i64,
    pub special: Option<SpecialGame>,
    pub wine: Wine,
    pub mount: Mount,
}

impl Load for GameModel {
    fn load(path: &Path) -> Result<GameModel, std::io::Error> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(v) => return Ok(v),
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        }
    }
}
impl GameModel {
    pub fn load_from_name(name: &str, list_store: &ListStore) -> Result<GameModel, std::io::Error> {
        let mut path = Environment::get_mofl_path();
        path.push("games");
        path.push(&name);
        path.push("game.json");
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        match serde_json::from_reader::<BufReader<File>, GameModel>(reader) {
            Ok(mut game_model) => {
                let mods = game_model.get_mods()?;
                let exes = game_model.get_executables().unwrap_or_default();
                for m in mods {
                    game_model.mods.insert(list_store.append(), m);
                    // TODO: ListStore update
                    //game_model.mods.push(execute::<Mod>((m, self.gtk_list_store.clone())));
                }
                /*for e in exes {
                    game_model.executables.push(init::<Executable>(e).unwrap());
                }*/
                return Ok(game_model);
            }
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        }
    }
    pub fn save(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = Environment::get_mofl_path();
        path.push("games");
        path.push(&self.label);
        std::fs::create_dir_all(&path)?;
        path.push("game.json");
        match serde_json::to_string_pretty(&self) {
            Ok(v) => match std::fs::write(&path.as_path(), v) {
                Ok(v) => return Ok(path),
                Err(e) => {
                    error!("Failed to write game config: {:?}", &e);
                    return Err(e);
                }
            },
            Err(e) => {
                UIHelper::serde_err(path.as_path(), &e);
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
    }
    fn get_mods(&self) -> Result<Vec<Mod>, std::io::Error> {
        let mut result: Vec<Mod> = Vec::new();
        let mut path = Environment::get_mofl_path();
        path.push("games");
        path.push(&self.label);
        path.push("mods");
        for entry in WalkDir::new(&path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            debug!("Found mod {:?}", entry.path());
            let mut mod_json: PathBuf = entry.path().to_path_buf();
            mod_json.push("mod.json");
            let m = Mod::load(&mod_json)?;
            result.push(m);
        }
        return Ok(result);
    }
    fn get_executables(&self) -> Result<Vec<ExecutableModel>, std::io::Error> {
        let mut path = Environment::get_mofl_path();
        path.push("games");
        path.push(&self.label);
        path.push("executables.json");
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(mods) => return Ok(mods),
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        }
    }
    pub fn get_plugins_txt_path(&self) -> Option<PathBuf> {
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
    pub fn write_plugins_txt(&self) {
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
    pub fn get_cfg_path(&self) -> PathBuf {
        let mut path = Environment::get_mofl_path();
        path.push("games");
        path.push(&self.label);
        return path;
    }
}

pub struct Game {
    model: GameModel,
    view: ApplicationWindow,
    list_store: ListStore,
    executables: Component<ExecutableManager>
}

impl Update for Game {
    type Model = GameModel;
    type ModelParam = &'static str;
    type Msg = Msg;

    fn model(_: &Relm<Self>, p: Self::ModelParam) -> Self::Model {
        match GameModel::load_from_name(
            &p,
            &Builder::new_from_string(include_str!("window.glade"))
                .get_object::<ListStore>("liststore-mod-list")
                .unwrap(),
        ) {
            Ok(model) => {
                model.save();
                return model;
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let game = UIHelper::prompt_new_game().unwrap();
                    game.save();
                    return game;
                }
                std::io::ErrorKind::InvalidInput => {
                    let game = UIHelper::prompt_new_game().unwrap();
                    game.save();
                    return game;
                }
                _ => panic!(),
            },
        }
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Modify(v) => {}
            Msg::AddMod => match UIHelper::prompt_install_mod(self.model.label.clone()) {
                Some(m) => {
                    m.save();
                    let m_display = m.clone();
                    let tree_iter = self.list_store.append();
                    self.list_store.set(
                        &tree_iter,
                        &[0, 1, 2, 4],
                        &[
                            &m_display.enabled,
                            &m_display.label,
                            &m_display.version,
                            &m_display.updated.naive_local().to_string(),
                        ],
                    );
                    match m_display.category {
                        Some(category) => self.list_store.set(&tree_iter, &[3], &[&category]),
                        None => self.list_store.set(&tree_iter, &[3], &[&"-"]),
                    }
                    match m_display.nexus_id {
                        Some(nexus_id) => self.list_store.set(&tree_iter, &[5], &[&nexus_id]),
                        None => self.list_store.set(&tree_iter, &[5], &[&"-"]),
                    }
                    self.model.mods.insert(tree_iter, m);
                }
                None => (),
            },
            Msg::RemoveMod(iter) => {
                let m_delete = self.model.mods.remove(&iter);
                self.list_store.remove(&iter);
            }
            Msg::EditMod(iter) => {
                let mut mod_to_edit: Mod = self.model.mods.get(&iter).unwrap().clone();
                match UIHelper::prompt_edit_mod(&mut mod_to_edit) {
                    true => {}
                    false => {}
                }
            },
            Msg::Start => self.executables.emit(crate::executablemanager::Msg::Start),
            Msg::Stop => {},
            Msg::Quit => gtk::main_quit(),
            Msg::Init => {
                let mut game_cfg_dir = Environment::get_mofl_path();
                game_cfg_dir.push("games");
                game_cfg_dir.push(&self.model.label);
                game_cfg_dir.push("mods");
                fs::create_dir_all(&game_cfg_dir);
                for entry in WalkDir::new(&game_cfg_dir)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    let mut mod_json: PathBuf = entry.path().to_path_buf();
                    mod_json.push("mod.json");
                    match Mod::load(&mod_json) {
                        Ok(m) => {
                            let m_display = m.clone();
                            let tree_iter = self.list_store.append();
                            self.list_store.set(
                                &tree_iter,
                                &[0, 1, 2, 4],
                                &[
                                    &m_display.enabled,
                                    &m_display.label,
                                    &m_display.version,
                                    &m_display.updated.naive_local().to_string(),
                                ],
                            );
                            match m_display.category {
                                Some(category) => {
                                    self.list_store.set(&tree_iter, &[3], &[&category])
                                }
                                None => self.list_store.set(&tree_iter, &[3], &[&"-"]),
                            }
                            match m_display.nexus_id {
                                Some(nexus_id) => {
                                    self.list_store.set(&tree_iter, &[5], &[&nexus_id])
                                }
                                None => self.list_store.set(&tree_iter, &[5], &[&"-"]),
                            }
                            self.model.mods.insert(tree_iter, m);
                        }
                        Err(_) => (),
                    }
                }
            }
            _ => (),
        }
    }
}

impl Widget for Game {
    type Root = ApplicationWindow;

    fn root(&self) -> Self::Root {
        return self.view.clone();
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let builder = gtk::Builder::new_from_string(include_str!("window.glade"));
        let window = builder.get_object::<ApplicationWindow>("mowindow").unwrap();
        let bt_add_mod = builder.get_object::<ToolButton>("bt_add_mod").unwrap();
        let mut exe_json_path = model.get_cfg_path();
        exe_json_path.push("executables.json");
        connect!(relm, bt_add_mod, connect_clicked(_), Msg::AddMod);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );
        connect!(relm, window, connect_show(_), Msg::Init);
        window.show_all();
        return Game {
            model: model,
            view: window,
            list_store: builder
                .get_object::<ListStore>("liststore-mod-list")
                .unwrap(),
            executables: init::<ExecutableManager>(exe_json_path).unwrap()
        };
    }
}
