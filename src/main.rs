extern crate gtk;
mod momod;
mod mogame;
mod moenv;
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    /*let mut test: momod::momod = momod::momod::new();
    let mut loader: moloader::moloader = moloader::moloader::new();
    test.set_label("Another Skyrim Mod".to_string());
    test.set_load_order(0);
    test.set_nexus_id(69);
    test.set_dir(PathBuf::from("anotherskyrimmod/"));
    println!("{}", test);
    sleep(Duration::new(2, 0));
    test.update();
    println!("{}", test);
    let mut game: mogame::mogame = mogame::mogame::new();
    game.mods.push(test);*/
    let mut env = moenv::moenv::new();
    let mut skyrim = mogame::mogame::new(env.get_base_path());
    env.add_game(skyrim);
    if gtk::init().is_err() {
        println!("Failed to initialize gtk");
        return;
    }
    let window = Window::new(WindowType::Toplevel);
    window.set_title("mofl");
    window.set_default_size(800, 600);
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}
