use crate::mogame::Executable;
use std::process::Command;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wine {
    pub prefix: PathBuf,
    pub path: PathBuf,
    pub esync: bool,
    pub staging_memory: bool
}

impl Wine {
    pub fn command(&self, exe: Executable) -> Command {
        let mut result = Command::new(&self.path);
        result.envs(self.to_env_args());
        return result;
    }
    fn to_env_args(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        result.insert("WINEPREFIX".to_string(), self.prefix.to_str().unwrap().to_string());
        result.insert("WINEESYNC".to_string(), Wine::bool_to_env_string(self.esync));
        result.insert("STAGING_SHARED_MEMORY".to_string(), Wine::bool_to_env_string(self.staging_memory));
        return result;
    }
    fn bool_to_env_string(input: bool) -> String {
        match input {
            true => return String::from("1"),
            false => return String::from("0")
        }
    }
}