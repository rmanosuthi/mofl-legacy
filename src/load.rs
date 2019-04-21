use std::path::PathBuf;

pub trait Load {
    fn load(path: &PathBuf) -> Result<Self, std::io::Error> where Self: std::marker::Sized;
}