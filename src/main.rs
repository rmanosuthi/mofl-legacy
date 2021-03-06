#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate glib;

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
#[macro_use]
extern crate relm_attributes;

use relm_attributes::widget;

mod esp;
mod gamepartial;
mod load;
mod mo2;
mod moconfig;
mod moenv;
mod game;
mod gamestarter;

mod executable;
mod executablemanager;
mod hasconfig;

mod momod;
mod mount;
mod save;
mod setupinstance;
mod special_game;
mod steam;
mod uihelper;
mod vfs;
mod wine;
mod worker;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;
use std::env::args;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use relm::Widget;

use crate::game::Game;

fn main() {
    #[cfg(debug_assertions)]
    {
            env_logger::Builder::from_env(
                    env_logger::Env::default().default_filter_or("debug"),
            ).default_format_timestamp(false)
            .init();
    }
    #[cfg(not(debug_assertions))]
    {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .default_format_timestamp(false)
                    .init();
    }
    /*info!("Remember: RUST_LOG=debug is your friend in case something goes wrong!");
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
    application.run(&args().collect::<Vec<_>>());*/
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(game_name) = args.get(1) {
        Game::run(game_name.to_owned()).unwrap();
    } else {
        warn!("Missing game name, assuming new one");
        Game::run("".to_owned()).unwrap();
    }
}