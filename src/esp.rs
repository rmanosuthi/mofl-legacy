use gtk::{ListStore, TreeIter};
use gtk::prelude::*;

use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

pub struct Esp {
    model: EspModel,
    tree_iter: TreeIter,
    master_list: Rc<Mutex<ListStore>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EspModel {
    pub enabled: bool,
    pub file_name: String
}

impl Esp {
    pub fn new(model: EspModel, master_list: Rc<Mutex<ListStore>>) -> Self {
        let tree_iter = master_list.lock().unwrap().append();
        return Self {
            model: model,
            tree_iter: tree_iter,
            master_list: master_list
        };
    }
    pub fn toggle(&mut self) {
        self.model.enabled = !self.model.enabled;
        self.master_list.lock().unwrap().set(
            &self.tree_iter,
            &[0],
            &[&self.model.enabled]
        );
    }
}
impl Drop for Esp {
    fn drop(&mut self) {
        let list_store: &ListStore = &self.master_list.lock().unwrap();
        list_store.remove(&self.tree_iter);
    }
}