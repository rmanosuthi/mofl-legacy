use momod::momod;
pub struct mogame {
    pub label: String,
    pub executables: Vec<String>,
    pub mods: Vec<momod>
}
impl mogame {
    pub fn new() -> mogame {
        mogame {
            label: "".to_string(),
            executables: Vec::new(),
            mods: Vec::new()
        }
    }
}