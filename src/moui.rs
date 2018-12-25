extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, TreeStore, TreeView, Window, WindowType};
use moenv::Environment;
pub struct UI {
    window: gtk::Window,
    pub env: Environment,
    mod_categories: TreeStore
}
impl UI {
    /// Creates a new UI instance
    pub fn new() -> UI {
        if gtk::init().is_err() {
            println!("Failed to initialize gtk");
            panic!();
        }

        let glade_src = include_str!("window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        let window: gtk::Window = builder.get_object("mowindow").unwrap();
        let mod_categories: TreeStore = builder.get_object::<TreeStore>("treestore-mod-categories").expect("Couldn't get treestore-mod-categories");
        mod_categories.insert_with_values(None, None, &[0], &[&"aaaa"]);
        window.set_title("Mod Organizer");
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        UI {
            window: window,
            env: Environment::new(),
            mod_categories: mod_categories
        }
    }
    /// Displays the window
    pub fn show(&self) -> () {
        self.window.show_all();
    }
    /// Hides the window
    pub fn hide(&self) -> () {
        self.window.hide();
    }
    pub fn add_category(&mut self, name: String) -> () {
        println!("Adding {}", name);
        self.mod_categories.insert_with_values(None, None, &[0], &[&name]);
    }
}
