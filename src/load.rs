use std::path::Path;

pub trait Load {
    fn load(path: &Path) -> Result<Self, std::io::Error> where Self: std::marker::Sized;
}