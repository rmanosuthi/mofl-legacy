use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wine {
    pub path: PathBuf
}