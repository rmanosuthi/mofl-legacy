use std::path::PathBuf;

use crate::momod::{Mod, ModModel};
use crate::special_game::SpecialGame;
use crate::wine::Wine;
use crate::mount::Mount;

pub struct GameStarter {
    pub label: String,
    pub steam_label: String,
    pub working_dir: PathBuf,
    pub mods: Vec<ModModel>,
    pub steam_id: i64,
    pub special: Option<SpecialGame>,
    pub wine: Wine,
    pub mount: Mount
}