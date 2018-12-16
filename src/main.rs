mod momod;
mod mogame;
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;

fn main() {
    let mut test: momod::momod = momod::momod::new();
    test.set_label("Another Skyrim Mod".to_string());
    test.set_load_order(0);
    test.set_nexus_id(69);
    test.set_dir(PathBuf::from("anotherskyrimmod/"));
    println!("{}", test);
    sleep(Duration::new(2, 0));
    test.update();
    println!("{}", test);
    let mut game: mogame::mogame = mogame::mogame::new();
    game.mods.push(test);
}
