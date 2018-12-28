use gio;
use gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, ListStore, TreeStore, Window, WindowType};
//use moenv::Environment;
pub struct UI {
    window: gtk::Window,
    //pub env: Environment,
    store_mod_categories: TreeStore,
}
impl UI {
    pub fn build_ui(application: &gtk::Application) {
        let glade_src = include_str!("window.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: ApplicationWindow = builder
            .get_object("mowindow")
            .expect("Couldn't get mowindow");
        window.set_application(application);
        window.set_title("mofl - Skyrim SE");
        window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });
        window.show_all();
        let mod_vec = super::momod::Mod::from(&builder.get_object::<ListStore>("treestore-mod-list").expect("Cannot load object")).expect("from return failed");
        println!("{}", mod_vec.get(0).unwrap());
    }
}
