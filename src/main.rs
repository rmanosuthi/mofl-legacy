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
        #[cfg(debug_assertions)]
        {
                env_logger::Builder::from_env(
                        env_logger::Env::default().default_filter_or("debug"),
                )
                .init();
        }
        #[cfg(not(debug_assertions))]
        {
                env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
                        .init();
        }
        info!("Remember: RUST_LOG=debug is your friend in case something goes wrong!");
        let application = gtk::Application::new("net.mpipo.mofl", gio::ApplicationFlags::empty())
                .expect("Initialization failed...");
        let glade_src = include_str!("window.glade");
        let builder = Builder::new_from_string(glade_src);
        let ui = moui::UI::new(builder);
        application.connect_startup(move |app| {
                ui.build_ui(app);
        });
        info!("UI init complete");
        application.connect_activate(|_| {});
        application.run(&args().collect::<Vec<_>>());
}
