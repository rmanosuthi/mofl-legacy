use crate::gamestarter::GameStarter;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Builder, Button, CssProvider, Grid, Label, ListStore, Menu, MenuItem, TextView, ToolButton, Toolbar,
    TreeIter,
};
use relm::{
    create_component, execute, init, Component, ContainerWidget, EventStream, Relm, Update, Widget,
};

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
                //let mods = game_model.get_mods()?;
                //let exes = game_model.get_executables().unwrap_or_default();
                /*for m in mods {
                    game_model.mods.insert(None, m); // insert TreeIter later because ListStore hasn't been initialized yet
                                                     // TODO: ListStore update
                                                     //game_model.mods.push(execute::<Mod>((m, self.gtk_list_store.clone())));
                }*/
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
    fn get_mods(&self) -> Vec<Mod> {
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
            match Mod::load(&mod_json, self.label.clone()) {
                Ok(m) => result.push(m),
                Err(e) => error!("Mod failed to load: {:?}", e)
            }
        }
        return result;
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
    executables: Component<ExecutableManager>,
    console_log: TextView,
    run_bt: ToolButton,
    bottom_bar: Grid,
    bottom_bar_game_status: Label,
    mods: HashMap<TreeIter, Mod>, // don't make Mod composited because of GTK's stupid way of doing lists
}

impl Game {
    pub fn load_mods(&mut self) -> () {
        let mods = self.model.get_mods();
        for m in mods {
            self.mods.insert(self.list_store.append(), m); // insert TreeIter later because ListStore hasn't been initialized yet
                                                           // TODO: ListStore update
                                                           //game_model.mods.push(execute::<Mod>((m, self.gtk_list_store.clone())));
        }
    }
    pub fn mods_to_vec(&self) -> Vec<Mod> {
        let mut mods = Vec::with_capacity(self.mods.capacity());
        for m in self.mods.values() {
            mods.push(m.clone());
        }
        return mods;
    }
    pub fn to_game_starter(&self) -> GameStarter {
        let mut mods = Vec::with_capacity(self.mods.capacity());
        for m in self.mods.values() {
            mods.push(m.clone());
        }
        return GameStarter {
            label: self.model.label.clone(),
            steam_label: self.model.steam_label.clone(),
            working_dir: self.model.path.clone(),
            mods: mods,
            steam_id: self.model.steam_id,
            special: self.model.special.clone(),
            wine: self.model.wine.clone(),
            mount: self.model.mount.clone(),
        };
    }
    pub fn write_plugins_txt(&self) {
        match fs::File::create(self.model.get_plugins_txt_path().unwrap()) {
            Ok(mut file) => {
                let list = vfs::generate_plugins_txt(self.mods_to_vec());
                for m in list {
                    writeln!(file, "{}", m);
                }
            }
            Err(e) => error!("{:?}", e),
        }
    }
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
                //model.save();
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
                    self.mods.insert(tree_iter, m);
                }
                None => (),
            },
            Msg::RemoveMod(iter) => {
                let m_delete = self.mods.remove(&iter);
                self.list_store.remove(&iter);
            }
            Msg::EditMod(iter) => {
                let mut mod_to_edit: Mod = self.mods.get(&iter).unwrap().clone();
                match UIHelper::prompt_edit_mod(&mut mod_to_edit) {
                    true => {}
                    false => {}
                }
            }
            Msg::Start => {
                let style_context = self.bottom_bar.get_style_context();
                let css_provider = CssProvider::new();
                debug!("{:?}", css_provider.load_from_data(b".game_running { background-color: #c66c37; }"));
                style_context.add_provider(&css_provider, 0);
                style_context.add_class("game_running");
                self.bottom_bar_game_status.set_text("Game is running");
                let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
                self.write_plugins_txt();
                self.executables.emit(crate::executablemanager::Msg::Start(
                    self.to_game_starter(),
                    sender,
                ));
                let console_buffer = self.console_log.get_buffer().unwrap();
                let gst = self.bottom_bar_game_status.clone();
                receiver.attach(None, move |text| {
                    match text.as_ref() {
                        "/////MOFL_GAME_STOPPED/////" => {
                            style_context.remove_class("game_running");
                            style_context.remove_provider(&css_provider);
                    gst.set_text("Game is not running");
                            info!("Game exited successfully!");
                            },
                        "/////MOFL_GAME_ERROR/////" => error!("Game exited with an error"),
                        o => {
                            info!("{}", &o);
                            console_buffer.insert(&mut console_buffer.get_end_iter(), &text);
                        }
                    }
                    glib::Continue(true)
                });
            }
            Msg::Stop => {
                
            }
            Msg::Quit => gtk::main_quit(),
            Msg::Init => {
                self.view.set_title(&format!(
                    "{} - Mod Organizer for Linux",
                    &self.model.steam_label
                ));
                self.executables.emit(crate::executablemanager::Msg::Init);
                self.load_mods();
                for (t, m) in &self.mods {
                    self.list_store.set(
                        &t,
                        &[0, 1, 2, 4],
                        &[
                            &m.enabled,
                            &m.label,
                            &m.version,
                            &m.updated.naive_local().to_string(),
                        ],
                    );
                    match m.category {
                        Some(category) => self.list_store.set(&t, &[3], &[&category]),
                        None => self.list_store.set(&t, &[3], &[&"-"]),
                    }
                    match m.nexus_id {
                        Some(nexus_id) => self.list_store.set(&t, &[5], &[&nexus_id]),
                        None => self.list_store.set(&t, &[5], &[&"-"]),
                    }
                    //self.model.mods.insert(t, m);
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
        let run_bt = builder.get_object::<ToolButton>("bt_run_exe").unwrap();
        let mut exe_json_path = model.get_cfg_path();
        exe_json_path.push("executables.json");
        connect!(relm, bt_add_mod, connect_clicked(_), Msg::AddMod);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );
        connect!(relm, run_bt, connect_clicked(_), Msg::Start);

        /*let menu_exe_list = builder.get_object::<Menu>("menu_exe_list").unwrap();
        menu_exe_list.prepend(&MenuItem::new_with_label("test.exe"));
        menu_exe_list.show_all();*/

        let exes = create_component::<ExecutableManager>(exe_json_path);
        let toolbar = builder.get_object::<Toolbar>("toolbar").unwrap();
        toolbar.insert(exes.widget(), 5);

        connect!(relm, window, connect_show(_), Msg::Init);
        window.show_all();
        return Game {
            model: model,
            view: window,
            list_store: builder
                .get_object::<ListStore>("liststore-mod-list")
                .unwrap(),
            console_log: builder.get_object::<TextView>("textview_output").unwrap(),
            run_bt: run_bt,
            bottom_bar: builder
                .get_object::<Grid>("bottom_bar")
                .unwrap(),
            bottom_bar_game_status: builder
                .get_object::<Label>("bottom_bar_game_status")
                .unwrap(),
            executables: exes,
            mods: HashMap::new(),
        };
    }
}
