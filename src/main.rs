#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod mo2;
mod moconfig;
mod moenv;
mod mogame;
mod momod;
mod moui;
mod steam;
mod uihelper;
mod vfs;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;
use std::env::args;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn main() {
        env_logger::init();
        let application = gtk::Application::new("net.mpipo.mofl", gio::ApplicationFlags::empty())
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
