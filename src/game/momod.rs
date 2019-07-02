use chrono::prelude::*;
use gtk::prelude::*;
use gtk::{ListStore, TreeIter};

use crate::game::esp::{Esp, EspModel};
use crate::load::Load;
use crate::moenv::Environment;

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use walkdir::WalkDir;

pub struct Mod {
    pub model: ModModel,
    pub tree_iter: TreeIter,
    list_store: Rc<ListStore>,
    esp_list_store: Rc<ListStore>,
    pub esps: BTreeMap<String, Esp>,
}

impl Mod {
    pub fn get_iter_string(&self) -> String {
        return self
            .list_store
            .get_string_from_iter(&self.tree_iter)
            .unwrap()
            .to_string();
    }
    pub fn load_all(
        game_name: &str,
        list_store: Rc<ListStore>,
        esp_list_store: Rc<ListStore>,
    ) -> BTreeMap<String, Mod> {
        let mut result = BTreeMap::new();
        let mut path = Environment::get_mofl_path();
        path.push("games");
        path.push(&game_name);
        path.push("mods");
        for entry in WalkDir::new(&path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let mut mod_json: PathBuf = entry.path().to_path_buf();
            mod_json.push("mod.json");
            match ModModel::load(&mod_json, game_name) {
                Ok(m) => {
                    let new_mod = Mod::new(m, list_store.clone(), esp_list_store.clone());
                    result.insert(
                        list_store
                            .get_string_from_iter(&new_mod.tree_iter)
                            .unwrap()
                            .to_string(),
                        new_mod,
                    );
                }
                Err(e) => error!("Mod failed to load: {:?}", e),
            }
        }
        return result;
    }
    pub fn new(model: ModModel, list_store: Rc<ListStore>, esp_list_store: Rc<ListStore>) -> Self {
        let mut map_esps: BTreeMap<String, Esp> = BTreeMap::new();
        for esp_model in model.get_esps() {
            let esp = Esp::new(esp_model, esp_list_store.clone());
            map_esps.insert(esp.get_iter_string(), esp);
        }
        let len: i32 = list_store.iter_n_children(None);
        let tree_iter: TreeIter = list_store.insert_with_values(
            Some(len as u32),
            &[0, 1, 2, 3, 4, 5],
            &[
                &model.enabled,
                &model.label,
                &model.version,
                &model.category.unwrap_or(-1),
                &model.updated.naive_local().to_string(),
                &model.nexus_id.unwrap_or(-1),
            ],
        );
        return Self {
            model: model,
            tree_iter: tree_iter,
            list_store: list_store,
            esp_list_store: esp_list_store,
            esps: map_esps,
        };
    }
    pub fn toggle(&mut self) {
        self.model.enabled = !self.model.enabled;
        self.list_store
            .set(&self.tree_iter, &[0], &[&self.model.enabled]);
        self.model.save();
    }
    pub fn toggle_esp(&mut self, esp_iter: &TreeIter) -> Option<bool> {
        let iter_string = self
            .esp_list_store
            .get_string_from_iter(&esp_iter)
            .unwrap()
            .to_string();
        match self.esps.get_mut(&iter_string) {
            Some(esp) => return esp.toggle(),
            None => return None,
        }
    }
}

impl Drop for Mod {
    fn drop(&mut self) {
        self.list_store.remove(&self.tree_iter);
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModModel {
    pub enabled: bool,
    pub label: String,
    pub version: String,
    pub category: Option<i64>,
    pub updated: DateTime<Utc>,
    pub nexus_id: Option<i64>,
    #[serde(skip)]
    pub game_name: String,
}

impl ModModel {
    pub fn get_esps(&self) -> Vec<EspModel> {
        let mut esps = Vec::new();
        let mut data_path = self.get_path();
        data_path.push("Data");
        for entry in walkdir::WalkDir::new(data_path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            // TODO - set priority properly
            if let Some(ext) = entry.path().extension() {
                if ext == "esp" {
                    esps.push(EspModel {
                        enabled: true,
                        file_name: entry
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    });
                }
            }
        }
        return esps;
    }
    pub fn load(path: &Path, game_name: &str) -> Result<Self, std::io::Error> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        match serde_json::from_reader::<BufReader<File>, Self>(reader) {
            Ok(mut v) => {
                v.game_name = game_name.to_string();
                return Ok(v);
            }
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        }
    }
    pub fn get_path(&self) -> PathBuf {
        let mut path = Environment::get_mofl_path();
        path.push("games");
        path.push(&self.game_name);
        path.push("mods");
        if self.nexus_id == None {
            path.push("unknown-id");
            path.push(&self.label);
        } else {
            path.push(self.nexus_id.unwrap().to_string());
        }
        return path;
    }
    pub fn save(&self) -> Result<PathBuf, std::io::Error> {
        let mut dest = Environment::get_mofl_path();
        dest.push("games");
        dest.push(&self.game_name);
        dest.push("mods");
        if self.nexus_id == None {
            dest.push("unknown-id");
            dest.push(&self.label);
        } else {
            dest.push(self.nexus_id.unwrap().to_string());
        }
        std::fs::create_dir_all(&dest)?;
        dest.push("mod.json");
        match serde_json::to_string_pretty(&self) {
            Ok(v) => match std::fs::write(dest.as_path(), v) {
                Ok(v) => return Ok(dest),
                Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied)),
            },
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        }
    }
    pub fn from_mo2(game_name: &str, mo2_mod_path: &Path) -> Option<Self> {
        let mut result = Self {
            enabled: false,
            label: String::new(),
            version: String::new(),
            category: None,
            updated: chrono::offset::Utc::now(),
            nexus_id: None,
            game_name: String::from(game_name),
        };
        let mut mo2_ini_path = PathBuf::from(&mo2_mod_path);
        mo2_ini_path.push("meta.ini");
        match ini::Ini::load_from_file_noescape(&mo2_ini_path) {
            Ok(ini) => {
                match ini.section(Some("General")) {
                    Some(v) => {
                        match mo2_mod_path.file_name() {
                            Some(v) => match v.to_str() {
                                Some(v) => {
                                    info!("Importing mod {}", &v);
                                    result.label = String::from(v);
                                }
                                None => {
                                    error!("Failed to convert path {:?} to string.", &v);
                                    error!("Does it contain non UTF-8 characters?");
                                    return None; // Label is necessary, so return none if there's none
                                }
                            },
                            None => (),
                        };
                        match v.get("version") {
                            Some(v) => result.version = v.to_owned(),
                            None => (),
                        };
                        match v.get("category") {
                            Some(v) => match v.replace(",", "").parse::<i64>() {
                                Ok(v) => result.category = Some(v),
                                Err(e) => {
                                    warn!("Failed to parse category: {:?}", e);
                                }
                            },
                            None => (),
                        };
                        // don't set result.updated
                        match v.get("modid") {
                            Some(v) => match v.parse::<i64>() {
                                Ok(v) => result.nexus_id = Some(v),
                                Err(e) => {
                                    warn!("Failed to parse Nexus ID: {:?}", e);
                                }
                            },
                            None => (),
                        };
                        let mut src = mo2_mod_path.to_owned();
                        //src.push("Data");
                        let mut dest = result.get_path();
                        dest.push("Data");
                        fs_extra::copy_items(
                            &vec![&src],
                            &dest,
                            &fs_extra::dir::CopyOptions {
                                overwrite: true,
                                skip_exist: false,
                                buffer_size: 64000,
                                copy_inside: true,
                                depth: 0,
                            },
                        );
                    }
                    None => (),
                }
            }
            Err(e) => {
                error!("Failed to read MO2 ini {:?}", &e);
                return None;
            }
        }
        result.save();
        Some(result)
    }
}

/*impl Drop for Mod {
    fn drop(&mut self) {
        debug!("Dropping mod");
    }
}*/
