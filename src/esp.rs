use gtk::{ListStore, TreeIter};
use gtk::prelude::*;

use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

pub struct Esp {
    pub model: EspModel,
    tree_iter: TreeIter,
    master_list: Rc<ListStore>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EspModel {
    pub enabled: bool,
    pub file_name: String
}

impl Esp {
    pub fn new(model: EspModel, master_list: Rc<ListStore>) -> Self {
        let len: i32 = master_list.iter_n_children(None);
        let tree_iter: TreeIter = master_list.insert_with_values(
            Some(len as u32),
            &[0, 1, 2],
            &[
                &model.enabled,
                &len,
                &model.file_name
            ]
        );
        return Self {
            model: model,
            tree_iter: tree_iter,
            master_list: master_list
        };
    }
    pub fn get_iter_string(&self) -> String {
        return self.master_list.get_string_from_iter(&self.tree_iter).unwrap().to_string();
    }
    pub fn toggle(&mut self) -> Option<bool> {
        self.model.enabled = !self.model.enabled;
        self.master_list.set(
            &self.tree_iter,
            &[0],
            &[&self.model.enabled]
        );
        return Some(self.model.enabled);
    }
}
impl Drop for Esp {
    fn drop(&mut self) {
        let list_store: &ListStore = &self.master_list;
        list_store.remove(&self.tree_iter);
    }
}