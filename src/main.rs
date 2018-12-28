extern crate gtk;
extern crate gio;
extern crate serde;
extern crate serde_json;
mod momod;
mod mogame;
mod moenv;
mod moui;
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;
use gio::prelude::*;
use std::env::args;
use gtk::prelude::*;

fn main() {
        let application = gtk::Application::new("net.mpipo.mofl",
                                                gio::ApplicationFlags::empty())
                                           .expect("Initialization failed...");

        application.connect_startup(move |app| {
            moui::UI::build_ui(app);
        });
        application.connect_activate(|_| {});

        application.run(&args().collect::<Vec<_>>());
}
