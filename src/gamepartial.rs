use gtk::ListStore;
use crate::steam::Steam;
use crate::special_game::SpecialGame;
use crate::wine::Wine;
use std::rc::Rc;

pub struct GamePartial {
    pub label: Option<String>,
    pub steam_label: Option<String>,
    pub steam: Option<Rc<Steam>>,
    pub special: Option<SpecialGame>,
    pub list_store: Option<Rc<ListStore>>,
    pub wine: Option<Wine>
}