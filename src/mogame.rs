use gtk::prelude::*;
use gtk::ListStore;
use momod::Mod;
use moui::DEFAULT_PATH;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub label: String,
    pub executables: Vec<PathBuf>,

    #[serde(skip)]
    pub mods: Vec<Mod>,

    pub folder_layout: Vec<PathBuf>,
    pub last_load_order: i64,
    pub categories: Vec<(u64, String)>,
}
impl Game {
    /// Creates an empty Game
    pub fn new(label: String) -> Game {
        Game {
            label: label,
            executables: Vec::new(),
            mods: Vec::new(),
            folder_layout: Vec::new(),
            last_load_order: -1,
            categories: Vec::new(),
        }
    }
    pub fn add_categories_to_view(&self, list: &ListStore) {
        for ref category in &self.categories {
            list.insert_with_values(None, &[0], &[&category.1]);
        }
    }
    pub fn add_exes_to_menu(&self, menu: &gtk::Menu) {
        for ref i in &self.executables {
            let new_item = gtk::MenuItem::new_with_label(i.to_str().unwrap());
            println!("{:?}", &new_item);
            &menu.append(&new_item);
        }
        &menu.show_all(); // IMPORTANT!
    }
    pub fn add_mods_from_folder(&mut self) {
        let mut game_cfg_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        game_cfg_path.push(DEFAULT_PATH);
        game_cfg_path.push("games");
        game_cfg_path.push(&self.label);
        game_cfg_path.push("mods");
        fs::create_dir_all(&game_cfg_path);
        match fs::read_dir(&game_cfg_path) {
            Ok(v) => {
                for ref entry in v {
                    match entry {
                        Ok(v) => {
                            let mut mod_json: PathBuf = v.path();
                            mod_json.push("mod.json");
                            match fs::read_to_string(&mod_json.as_path()) {
                                Ok(v) => {
                                    self.mods.push(serde_json::from_str(&v).unwrap());
                                }
                                Err(e) => println!("Failed to read mod.json, skipping"),
                            }
                        }
                        Err(e) => {}
                    }
                }
            }
            Err(e) => println!("Failed to read game dir, aborting"),
        }
    }
    /// Updates the base path for the game
    /// This is usually called by Environment
    /*pub fn update_base_path(&mut self, input: PathBuf) -> () {
        self.base_path = input;
    }*/
    /// Imports a mod, taking its path as an argument
    /*pub fn import(&mut self, file: PathBuf) -> bool {
        let new_mod = self.mod_from_archive(file);
        self.mods.push(new_mod);
        return true;
    }*/
    /*fn mod_from_archive(&self, file: PathBuf) -> Mod {
        // file must exist
        let mut result: Mod = Mod::new();
        match file.file_name() {
            Some(v) => {
                result.set_label(
                    v.to_str()
                        .expect("Cannot convert file name into string")
                        .to_string(),
                );
            }
            None => (),
        }
        // extract archive
        let label = result.get_label().to_owned();
        let mut path = PathBuf::from(&self.base_path);
        path.push("games");
        path.push(&self.label);
        path.push("mods");
        path.push(&self.gen_uuid().to_string());
        let cmd = Command::new("7z")
            .current_dir(path)
            .arg("x")
            .arg(
                file.canonicalize()
                    .expect("Cannot convert file path into absolute path"),
            )
            .arg("-o".to_owned() + "Data/")
            .output()
            .expect("Extract failed");
        println!("{:?}", cmd.stdout);
        result.set_dir(PathBuf::from(label));
        result.update();
        return result;
    }*/
    fn gen_uuid(&self) -> u64 {
        return 0;
    }
    /*fn sanitize(&self, input: Mod) -> bool {
        // holy error handling Batman!
        for entry in fs::read_dir(input.get_dir()).expect("Cannot read mod dir") {
            let entry: fs::DirEntry = entry.expect("Also cannot read dir");
            for str_comp in &self.folder_layout {
                if entry.metadata().expect("Cannot read metadata").is_dir() == true {
                    let entry_file_name = entry.file_name();
                    let entry_name = entry_file_name
                        .to_str()
                        .expect("Cannot convert file name into string");
                    //.to_str().expect("Cannot convert file name into string");
                    if entry_name == str_comp.to_str().expect("") {
                        fs::rename(entry_name, str_comp).expect("Cannot rename folder");
                    }
                }
            }
        }
        return true;
    }*/
    /// stub - Validates mod
    fn check_sanity(input: Mod) -> bool {
        return true;
    }
    /// stub - Start a process
    pub fn start(&self, exe: PathBuf) -> bool {
        // check if file exists
        // spawn child process
        return true;
    }
    /// stub - Stop a process
    pub fn stop(&self, exe: PathBuf) -> bool {
        // check if file exists
        // stop child process
        return true;
    }
}
