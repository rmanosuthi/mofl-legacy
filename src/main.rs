#[macro_use]
extern crate serde_derive;

mod momod;
mod mogame;
//mod moenv;
mod moui;
mod moconfig;
mod vfs;
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;
use gio::prelude::*;
use std::env::args;
use gtk::prelude::*;
use gtk::Builder;

fn main() {
        let application = gtk::Application::new("net.mpipo.mofl",
                                                gio::ApplicationFlags::empty())
                                           .expect("Initialization failed...");
                                                   let glade_src = include_str!("window.glade");
        let builder = Builder::new_from_string(glade_src);
        let ui = moui::UI::new(builder);
        application.connect_startup(move |app| {
            ui.build_ui(app);
        });
        application.connect_activate(|_| {});

        application.run(&args().collect::<Vec<_>>());
}
