use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Builder, Button, CellRendererToggle, CssProvider, FileChooserAction, Grid, Label, ListStore,
    Menu, MenuItem, TextView, ToolButton, Toolbar, TreeIter,
};
use relm::{
    create_component, execute, init, Component, ContainerWidget, EventStream, Relm, Update, Widget,
};

use crate::executable::{Executable, ExecutableModel, ExecutableMsg, ExecutableStatus};
use crate::exe::executablemanager::ExecutableManager;
use crate::load::Load;
use crate::moenv::Environment;
use crate::game::momod::{Mod, ModModel};
use crate::game::starter::GameStarter;
use crate::mount::Mount;
use crate::game::special::SpecialGame;
use crate::uihelper::UIHelper;
use crate::vfs;
use crate::wine::Wine;
use crate::wine::WineType;
use crate::worker::WorkerManager;

use crate::worker::{WorkerReply, WorkerSend};
use std::thread;

use std::collections::{BTreeMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

use walkdir::WalkDir;

pub mod esp;
pub mod partial;
pub mod starter;
pub mod momod;
pub mod special;

#[derive(Msg, Debug)]
pub enum Msg {
    Init,
    Modify(GameModel),

    AddMod,
    RemoveMod(TreeIter),
    UpdateMod(TreeIter),
    EditMod(TreeIter),
    ToggleMod(TreeIter),

    LoadEsps,
    ToggleEsp(TreeIter),

    OrderImportMo2,
    ImportMo2Start,
    ImportMo2(ModModel),
    ImportMo2Done,
    EditExes,
    Start,
    Stop,
    Quit,
}

type Esp = String;

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn get_data_path(&self) -> PathBuf {
        let mut data_path = self.path.clone();
        data_path.push("Data/");
        return data_path;
    }
    pub fn load_from_name(name: &str, list_store: &ListStore) -> Result<Self, std::io::Error> {
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
            Err(e) => {
                error!("{:?}", e);
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
            }
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
    list_store: Rc<ListStore>,     // yes, I know I don't need an Rc here
    esp_list_store: Rc<ListStore>, // it's for clarity to show that it's shared
    executables: Component<ExecutableManager>,
    console_log: TextView,
    run_bt: ToolButton,
    menuitem_import_mo2: MenuItem,
    bottom_bar: Grid,
    bottom_bar_game_status: Label,
    mods: BTreeMap<String, Mod>, // don't make Mod composited because of GTK's stupid way of doing lists

    crt_mods: CellRendererToggle,
    crt_esps: CellRendererToggle,

    worker_manager: WorkerManager, //local_sender: std::sync::mpsc::Sender<WorkerSend>,
    //local_receiver: glib::Receiver<WorkerReply>
    relm_channel: relm::Channel<WorkerReply>,
}

impl Game {
    pub fn load_mods(&mut self) -> () {
        self.mods = Mod::load_all(
            &self.model.label,
            self.list_store.clone(),
            self.esp_list_store.clone(),
        );
    }
    pub fn to_game_starter(&self) -> GameStarter {
        let mut mods = Vec::with_capacity(self.mods.len());
        for m in self.mods.values() {
            mods.push(m.model.clone());
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
                let list = vfs::generate_plugins_txt(&self.mods);
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
            &Builder::new_from_string(include_str!("../window.glade"))
                .get_object::<ListStore>("liststore-mod-list")
                .unwrap(),
        ) {
            Ok(model) => {
                //model.save();
                return model;
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    error!("Game not found");
                    let game = UIHelper::prompt_new_game().unwrap();
                    game.save();
                    return game;
                }
                std::io::ErrorKind::InvalidInput => {
                    error!("Game cfg is invalid: {:?}", e);
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
            Msg::LoadEsps => {}
            Msg::ToggleMod(iter) => {
                debug!("Toggled mod {:?}", &iter);
                let iter_string = self
                    .list_store
                    .get_string_from_iter(&iter)
                    .unwrap()
                    .to_string();
                let mut m: &mut Mod = self.mods.get_mut(&iter_string).unwrap();
                m.toggle();
            }
            Msg::ToggleEsp(iter) => {
                debug!("Toggled esp {:?}", &iter);
                let mut change_to: Option<bool> = None;
                // let iter_string = self.esp_list_store.get_string_from_iter(&iter).unwrap().to_string();
                for (_iter, m) in self.mods.iter_mut() {
                    if let Some(changed_to) = m.toggle_esp(&iter) {
                        break;
                    }
                }
            }
            Msg::AddMod => {
                if let Some(new_mod) = UIHelper::prompt_install_mod(
                    &self.model.label,
                    self.list_store.clone(),
                    self.esp_list_store.clone(),
                ) {
                    self.mods.insert(new_mod.get_iter_string(), new_mod);
                }
            }
            Msg::RemoveMod(iter) => {
                let iter_string = self
                    .list_store
                    .get_string_from_iter(&iter)
                    .unwrap()
                    .to_string();
                let m_delete = self.mods.remove(&iter_string);
                self.list_store.remove(&iter);
            }
            Msg::EditMod(iter) => {
                let iter_string = self
                    .list_store
                    .get_string_from_iter(&iter)
                    .unwrap()
                    .to_string();
                let mut mod_to_edit: &mut Mod = self.mods.get_mut(&iter_string).unwrap();
                match UIHelper::prompt_edit_mod(&mut mod_to_edit) {
                    true => {}
                    false => {}
                }
            }
            Msg::Start => {
                let style_context = self.bottom_bar.get_style_context();
                let css_provider = CssProvider::new();
                let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
                self.write_plugins_txt();
                self.executables.emit(crate::exe::executablemanager::Msg::Start(
                    self.to_game_starter(),
                    sender,
                ));
                let console_buffer = self.console_log.get_buffer().unwrap();
                let gst = self.bottom_bar_game_status.clone();
                receiver.attach(None, move |exe_status| {
                    /*match text.as_ref() {
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
                    }*/
                    match exe_status {
                        ExecutableStatus::Started => {
                            css_provider
                                .load_from_data(b".game_running { background-color: #c66c37; }");
                            style_context.add_provider(&css_provider, 0);
                            style_context.add_class("game_running");
                            gst.set_text("Game is running");
                        }
                        ExecutableStatus::Output(output) => {
                            console_buffer.insert(&mut console_buffer.get_end_iter(), &output);
                        }
                        ExecutableStatus::Stopped(exit_status) => {
                            style_context.remove_class("game_running");
                            style_context.remove_provider(&css_provider);
                            gst.set_text(&format!("Game is not running: {:?}", exit_status));
                        }
                    }
                    glib::Continue(true)
                });
            }
            Msg::Stop => {}
            Msg::OrderImportMo2 => {
                if let Some(path) = UIHelper::dialog_path("Please locate the MO2 game folder", FileChooserAction::SelectFolder) {
                    self.worker_manager.add_task(WorkerSend::ImportMo2(path));
                }
            },
            Msg::ImportMo2Start => {

            },
            Msg::ImportMo2(m) => {
                    let imported_mod = Mod::new(m, self.list_store.clone(), self.esp_list_store.clone());
                    self.mods
                        .insert(imported_mod.get_iter_string(), imported_mod);
            },
            Msg::ImportMo2Done => {

            },
            Msg::Init => {
                self.view.set_title(&format!(
                    "{} - Mod Organizer for Linux",
                    &self.model.steam_label
                ));
                self.executables.emit(crate::exe::executablemanager::Msg::Init);
                self.load_mods();
            }
            Msg::Quit => {
                //self.local_sender.send(WorkerSend::StopWorker);
                gtk::main_quit();
            }
            other => error!("Unhandled signal {:?}", other),
        }
    }
}

impl Widget for Game {
    type Root = ApplicationWindow;

    fn root(&self) -> Self::Root {
        return self.view.clone();
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let stream = relm.stream().clone();
        let (relm_channel, send_to_relm) =
            relm::Channel::new(move |worker_reply| match worker_reply {
                WorkerReply::ImportMo2Start => {
                    stream.emit(Msg::ImportMo2Start);
                }
                WorkerReply::ImportMo2(m) => {
                    stream.emit(Msg::ImportMo2(m));
                },
                WorkerReply::ImportMo2Done => {
                    stream.emit(Msg::ImportMo2Done);
                }
                _ => (),
            });
        let builder = gtk::Builder::new_from_string(include_str!("../window.glade"));
        let window = builder.get_object::<ApplicationWindow>("mowindow").unwrap();
        let bt_add_mod = builder.get_object::<ToolButton>("bt_add_mod").unwrap();
        let run_bt = builder.get_object::<ToolButton>("bt_run_exe").unwrap();
        let crt_mods = builder
            .get_object::<CellRendererToggle>("modview_toggle_column")
            .unwrap();
        let crt_esps = builder
            .get_object::<CellRendererToggle>("crt_esps")
            .unwrap();
        let mut exe_json_path = model.get_cfg_path();
        let mods_list_store = builder
            .get_object::<ListStore>("liststore-mod-list")
            .unwrap();
        let esp_list_store = builder
            .get_object::<ListStore>("liststore-load-order")
            .unwrap();
        let menuitem_import_mo2 = builder
                .get_object::<MenuItem>("menuitem_import_mo2")
                .unwrap();
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

        let worker_manager = WorkerManager::new(&model.label, send_to_relm, 4);

        connect!(relm, window, connect_show(_), Msg::Init);

        // TODO: connect mo2 import bt

        let mls = mods_list_store.clone();
        connect!(
            relm,
            crt_mods,
            connect_toggled(s, tree_path),
            Msg::ToggleMod(mls.get_iter(&tree_path).unwrap())
        );
        let els = esp_list_store.clone();
        connect!(
            relm,
            crt_esps,
            connect_toggled(s, tree_path),
            Msg::ToggleEsp(els.get_iter(&tree_path).unwrap())
        );
        connect!(
            relm,
            menuitem_import_mo2,
            connect_activate(_),
            Msg::OrderImportMo2
        );
        window.show_all();
        return Game {
            model: model,
            view: window,
            list_store: Rc::new(mods_list_store),
            esp_list_store: Rc::new(esp_list_store),
            console_log: builder.get_object::<TextView>("textview_output").unwrap(),
            run_bt: run_bt,
            bottom_bar: builder.get_object::<Grid>("bottom_bar").unwrap(),
            bottom_bar_game_status: builder
                .get_object::<Label>("bottom_bar_game_status")
                .unwrap(),
            menuitem_import_mo2: menuitem_import_mo2,
            executables: exes,
            mods: BTreeMap::new(),
            crt_mods: crt_mods,
            crt_esps: crt_esps,

            worker_manager: worker_manager, //local_sender: local_sender
            relm_channel: relm_channel,
        };
    }
}
