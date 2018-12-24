use momod::momod;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

pub struct mogame {
    pub label: String,
    pub executables: Vec<PathBuf>,
    pub mods: Vec<momod>,
    pub folder_layout: Vec<PathBuf>,
    pub last_load_order: i64,
    pub base_path: PathBuf
}
impl mogame {
    pub fn new(input: PathBuf) -> mogame {
        mogame {
            label: "".to_string(),
            executables: Vec::new(),
            mods: Vec::new(),
            folder_layout: Vec::new(),
            last_load_order: -1,
            base_path: input
        }
    }
    pub fn update_base_path(&mut self, input: PathBuf) -> () {
        self.base_path = input;
    }
    pub fn import(&mut self, file: PathBuf) -> bool {
        let new_mod = self.mod_from_archive(file);
        self.mods.push(new_mod);
        return true;
    }
    fn mod_from_archive(&self, file: PathBuf) -> momod {
        // file must exist
        let mut result: momod = momod::new();
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
    }
    fn gen_uuid(&self) -> u64 {
        return 0;
    }
    fn sanitize(&self, input: momod) -> bool {
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
    }
    fn check_sanity(input: momod) -> bool {
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
