use chrono::prelude::*;
use gtk::prelude::*;
use gtk::{ListStore, TreeIter};

use crate::load::Load;
use crate::moenv::Environment;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Mod {
    pub enabled: bool,
    pub label: String,
    pub version: String,
    pub category: Option<i64>,
    pub updated: DateTime<Utc>,
    pub nexus_id: Option<i64>,
    #[serde(skip)]
    game_name: String
}

impl Mod {
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
        dest.push("mod.json");
        match serde_json::to_string_pretty(&self) {
            Ok(v) => match std::fs::write(dest.as_path(), v) {
                Ok(v) => return Ok(dest),
                Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
            },
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
        }
    }
}

impl Load for Mod {
    fn load(path: &Path) -> Result<Mod, std::io::Error> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(v) => return Ok(v),
            Err(e) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
        }
    }
}

impl Drop for Mod {
    fn drop(&mut self) {
        debug!("Dropping mod");
    }
}