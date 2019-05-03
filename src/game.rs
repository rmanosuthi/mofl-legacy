use relm::{Component, ContainerWidget, EventStream, Relm, Update, Widget, execute, init};
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, ListStore, TreeIter};

use crate::executable::{Executable, ExecutableModel};
use crate::momod::Mod;
use crate::uihelper::UIHelper;
use crate::special_game::SpecialGame;
use crate::moenv::Environment;
use crate::load::Load;
use crate::wine::Wine;
use crate::mount::Mount;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;

use walkdir::WalkDir;

#[derive(Msg)]
enum GameMsg {
    Load(String),
    Unload,
    Modify(GameModel),
    AddMod(Option<Mod>),
    RemoveMod(TreeIter),
    UpdateMod(TreeIter),
    EditMod(TreeIter),
    ImportMo2(PathBuf)
}

#[derive(Serialize, Deserialize)]
pub struct GameModel {
    label: String,
    steam_label: String,

    #[serde(skip)]
    executables: Vec<Component<Executable>>,

    #[serde(skip)]
    mods: HashMap<TreeIter, Mod>, // don't make Mod composited because of GTK's stupid way of doing lists

    steam_id: i64,
    special: Option<SpecialGame>,
    wine: Wine,
    mount: Mount
}

impl Load for GameModel {   
    fn load(path: &Path) -> Result<GameModel, std::io::Error> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(v) => return Ok(v),
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
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
            Ok(game_model) => {
                let mods = game_model.get_mods()?;
                for m in mods {
                    game_model.mods.insert(list_store.append(), m);
                    // TODO: ListStore update
                    //game_model.mods.push(execute::<Mod>((m, self.gtk_list_store.clone())));
                }
                return Ok(game_model);
            },
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
        }
    }
    pub fn save(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = Environment::get_mofl_path();
        path.push("games");
        path.push(&self.label);
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
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
        }
    }
}

pub struct Game {
    model: GameModel,
    view: ApplicationWindow,
    list_store: ListStore
}

impl Update for Game {
    type Model = GameModel;
    type ModelParam = &'static str;
    type Msg = GameMsg;

    fn model(_: &Relm<Self>, p: Self::ModelParam) -> Self::Model {
        match GameModel::load_from_name(
            &p,
            &Builder::new_from_string(include_str!("window.glade")).get_object::<ListStore>("liststore-mod-list").unwrap()
        ) {
            Ok(model) => return model,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {},
                std::io::ErrorKind::InvalidInput => {},
                _ => panic!()
            }
        }
    }

    fn update(&mut self, msg: GameMsg) {
        match msg {
            GameMsg::Modify(v) => {},
            GameMsg::AddMod(m) => {
                match m {
                    Some(m) => {
                        // TODO: Mod, insert_with_values
                        let m_display = m.clone();
                        let tree_iter = self.list_store.append();
                        self.list_store.set(&tree_iter, &[0, 1, 2, 4], &[
                            &m_display.enabled,
                            &m_display.label,
                            &m_display.version,
                            &m_display.updated.naive_local().to_string(),
                        ]);
                        match m_display.category {
                            Some(category) => self.list_store.set(&tree_iter, &[3], &[&category]),
                            None => self.list_store.set(&tree_iter, &[3], &[&"-"])
                        }
                        match m_display.nexus_id {
                            Some(nexus_id) => self.list_store.set(&tree_iter, &[5], &[&nexus_id]),
                            None => self.list_store.set(&tree_iter, &[5], &[&"-"])
                        }
                        self.model.mods.insert(tree_iter, m);
                    },
                    None => ()
                }
            },
            GameMsg::RemoveMod(iter) => {
                let m_delete = self.model.mods.remove(&iter);
                self.list_store.remove(&iter);
            },
            GameMsg::EditMod(iter) => {
                let mod_to_edit: Mod = self.model.mods.get(&iter).unwrap().clone();
                match UIHelper::prompt_edit_mod(mod_to_edit) {
                    Some(changed_mod) => {},
                    None => ()
                }
            }
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
        window.show_all();
        connect!(relm, bt_add_mod, connect_clicked(_), GameMsg::AddMod(UIHelper::prompt_install_mod()));
        return Game {
            model: model,
            view: window,
            list_store: builder.get_object::<ListStore>("liststore-mod-list").unwrap()
        };
    }
}