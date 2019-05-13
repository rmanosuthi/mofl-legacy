use chrono::prelude::*;
use gtk::prelude::*;
use gtk::{ListStore, TreeIter};

use crate::load::Load;
use crate::moenv::Environment;
use crate::esp::Esp;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;

use walkdir::WalkDir;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Mod {
    pub enabled: bool,
    pub label: String,
    pub version: String,
    pub category: Option<i64>,
    pub updated: DateTime<Utc>,
    pub nexus_id: Option<i64>,
    pub esps: Vec<Esp>,
    #[serde(skip)]
    pub game_name: String
}

impl Mod {
    pub fn get_esps(&self) -> Vec<Esp> {
        let esps = Vec::with_capacity(256);
        for entry in walkdir::WalkDir::new(self.get_path())
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok()) {
            // TODO - set priority properly
            if let Some(ext) = entry.path().extension() {
                if ext == "esp" {
                    esps.push(Esp {
                        enabled: true,
                        file_name: entry.path().file_name().unwrap().to_str().unwrap().to_string()
                    });
                }
            }
        }
        return esps;
    }
    pub fn load(path: &Path, game_name: String) -> Result<Mod, std::io::Error> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        match serde_json::from_reader::<BufReader<File>, Mod>(reader) {
            Ok(mut v) => {
                v.game_name = game_name;
                return Ok(v);
            },
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
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
                Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
            },
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
        }
    }
    pub fn from_mo2(
        game_path: &Path,
        path_from: &Path) -> Option<Mod> {
        let mut result = Mod {
            enabled: false,
            label: String::new(),
            version: String::new(),
            category: None,
            updated: chrono::offset::Utc::now(),
            nexus_id: None,
            game_name: String::new(),
            esps: Vec::new()
        };
        let mut mo2_ini_path = PathBuf::from(&path_from);
        mo2_ini_path.push("meta.ini");
        match ini::Ini::load_from_file_noescape(&mo2_ini_path) {
            Ok(ini) => {
                match ini.section(Some("General")) {
                    Some(v) => {match path_from.file_name() {
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
                        for entry in WalkDir::new(&path_from).into_iter().filter_map(|e| e.ok()) {
                            let mut dest = PathBuf::from(&game_path);
                            dest.push("mods");
                            if result.nexus_id == None {
                                dest.push("unknown-id");
                                dest.push(&result.label);
                            } else {
                                dest.push(result.nexus_id.unwrap().to_string());
                            }
                            dest.push("Data");
                            std::fs::create_dir_all(&dest);
                            dest.push(entry.file_name());
                            debug!("Copying {:?} to {:?}", entry.path().to_path_buf(), &dest);
                            //fs::copy(&v.path(), &dest);
                        }
                    }
                    None => (),
                }
            }
            Err(e) => {
                error!("Failed to read MO2 ini {:?}", &e);
                return None;
            }
        }
        debug!(">>> returning something");
        Some(result)
    }
}

/*impl Drop for Mod {
    fn drop(&mut self) {
        debug!("Dropping mod");
    }
}*/