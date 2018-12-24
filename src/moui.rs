extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use moenv::Environment;
pub struct UI {
    window: gtk::Window,
    pub env: Environment,
}
impl UI {
    /// creates a new UI instance
    pub fn new() -> UI {
        if gtk::init().is_err() {
            println!("Failed to initialize gtk");
            panic!();
        }

        let glade_src = include_str!("window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        let window: gtk::Window = builder.get_object("mowindow").unwrap();
        window.set_title("Mod Organizer");
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        UI {
            window: window,
            env: Environment::new(),
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
}
