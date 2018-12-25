extern crate gtk;
mod momod;
mod mogame;
mod moenv;
mod moui;
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;

fn main() {
    let mut ui = moui::UI::new();
    let mut skyrim = mogame::Game::new(ui.env.get_base_path());
    ui.env.add_game(skyrim);
    ui.show();
    ui.add_category("Misc".to_string());
    gtk::main();
}
