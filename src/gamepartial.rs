use gtk::ListStore;
use crate::mount::Mount;
use crate::steam::Steam;
use crate::special_game::SpecialGame;
use crate::wine::Wine;
use std::rc::Rc;

pub struct GamePartial {
    pub label: Option<String>,
    pub steam_label: Option<String>,
    pub wine: Option<Wine>,
    pub special: Option<SpecialGame>,
    pub mount: Option<Mount>,
    pub steam_id: Option<i64>
}

pub enum GameEdit<T> {
    Update(T),
    NoUpdate,
    NoValue
}