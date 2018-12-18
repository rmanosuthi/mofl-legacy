use std::path::PathBuf;
use std::fs;
use std::process::Command;
// use std::io::prelude::*;
use momod::momod;

pub struct moloader {
    folder_layout: Vec<String>
}
impl moloader {
    pub fn new() -> moloader {
        moloader {
            folder_layout: Vec::new()
        }
    }
    pub fn extract(file: PathBuf) -> momod { // file must exist
        let mut result: momod = momod::new();
        match file.file_name() {
            Some(v) => {
                result.set_label(v.to_str().expect("Cannot convert file name into string").to_string());
                },
            None => ()
        }
        // extract archive
        let cmd = Command::new("sh").arg("-c").arg("7z x ")
        return result;
    }
    fn sanitize(&self, input: momod) -> bool { // holy error handling Batman!
        for entry in fs::read_dir(input.get_dir()).expect("Cannot read mod dir") {
            let entry: fs::DirEntry = entry.expect("Also cannot read dir");
            for str_comp in self.folder_layout {
                if entry.metadata().expect("Cannot read metadata").is_dir() == true {
                    let entry_name: String = entry.file_name().into_string().expect("Cannot convert file name into string");
                    if entry_name.to_lowercase() == str_comp {
                        fs::rename(entry_name, str_comp);
                    }
                }
            }
        }
        return true;
    }
    fn check_sanity(input: momod) -> bool {

    }
}