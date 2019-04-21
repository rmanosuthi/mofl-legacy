use std::path::PathBuf;

pub trait Save {
    fn save(&self) -> Result<PathBuf, std::io::Error>;
}