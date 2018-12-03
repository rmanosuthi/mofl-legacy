mod momod;
extern crate chrono;
use std::thread::sleep;

fn main() {
    let test = momod::new();
    test.set_label("Another Skyrim Mod".to_string());
    test.set_load_order(0);
    test.set_nexus_id(69);
    test.set_dir("anotherskyrimmod/".to_string());
    println!("{}", test);
        sleep(Duration::new(2, 0));
    test.update();
    println!("{}", test);
}
