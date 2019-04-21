use crate::save::Save;
use crate::steam::Steam;
use crate::moconfig::Config;
use crate::mogame::Game;
use crate::moenv::Environment;

use std::path::PathBuf;

pub struct SetupInstance {
    pub games: Vec<Game>,
    pub steam: Steam,
    pub config: Config,
    pub active_idx: u64
}

impl Save for SetupInstance {
    fn save(&self) -> Result<PathBuf, std::io::Error> {
        let base_path = Environment::get_home();
        for game in &self.games {
            game.save()?;
        }
        self.steam.save()?;
        self.config.save()?;
        return Ok(PathBuf::new());
    }
}