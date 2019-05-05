use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;

use crate::game::GameModel;

/*pub trait HasConfig {
    fn load(path: &Path) -> Result<Self, std::io::Error> where Self: std::marker::Sized + std::io::Read + serde::Deserialize {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        return serde_json::from_reader::<BufReader, Self>(reader);
    }
    //fn load_from_name(name: &str) -> Result<Self, std::io::Error> where Self: std::marker::Sized;
    fn save(&self) -> Result<PathBuf, std::io::Error> {
        return self.path;
    }
}*/