use gtk::ListStore;
use crate::steam::Steam;
use crate::special_game::SpecialGame;
use crate::wine::Wine;
use std::rc::Rc;

pub struct GamePartial {
    pub label: Option<String>,
    pub steam_label: Option<String>,
    pub wine: Option<Wine>,
    pub special: Option<SpecialGame>
}