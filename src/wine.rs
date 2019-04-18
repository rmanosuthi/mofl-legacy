use crate::moenv::Environment;
use crate::mogame::Executable;
use crate::mogame::Game;
use crate::steam::Steam;
use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;
use std::process::Command;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum WineType {
    SYSTEM,
    LUTRIS,
    PROTON,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Wine {
    pub prefix: PathBuf,
    pub version: String,
    pub esync: bool,
    pub staging_memory: bool,
    pub wine_type: WineType,
}

impl Wine {
    pub fn type_to_idx(&self) -> Option<u32> {
        match self.wine_type {
            WineType::SYSTEM => return Some(0),
            WineType::LUTRIS => return Some(1),
            WineType::PROTON => return Some(2)
        }
    }
    pub fn get_types() -> Vec<String> {
        let mut result = Vec::new();
        result.push("System".to_string());
        result.push("Lutris".to_string());
        result.push("Proton".to_string());
        return result;
    }
    pub fn get_versions(
        steam: &Steam,
        wine_type: WineType,
    ) -> Result<Vec<(String, PathBuf)>, std::io::Error> {
        let mut result: Vec<(String, PathBuf)> = Vec::new();
        match wine_type {
            WineType::SYSTEM => {}
            WineType::LUTRIS => {
                let mut lutris_wine_runner_path = Environment::get_home();
                lutris_wine_runner_path.push(".local/share/lutris/runners/wine/");
                for runner in walkdir::WalkDir::new(lutris_wine_runner_path)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok()) {
                    result.push((runner.file_name().to_str().unwrap().to_string(), runner.path().to_owned()));
                }
            }
            WineType::PROTON => return Ok(steam.get_proton_versions()),
        }
        return Ok(result);
    }
    fn type_version_to_path(&self) -> Option<PathBuf> {
        let steam = Steam::new_from_config();
        match self.wine_type {
            WineType::SYSTEM => {
                let try_paths = vec!["/bin/wine", "/usr/bin/wine", "/usr/local/bin/wine"];
                for try_path in try_paths {
                    let path = PathBuf::from(try_path);
                    if path.exists() == true {
                        return Some(path);
                    }
                }
                return None;
            },
            WineType::LUTRIS => {
                let mut try_path = Environment::get_home();
                try_path.push(".local/share/lutris/runners/wine/");
                try_path.push(&self.version);
                try_path.push("bin/wine");
                if try_path.exists() {
                    return Some(try_path);
                } else {
                    return None;
                }
            },
            WineType::PROTON => {
                for proton in steam.get_proton_versions() {
                    if proton.0 == self.version {
                        let mut result = proton.1;
                        result.push("proton");
                        debug!("Command: {:?}", &result);
                        return Some(result);
                    }
                }
                return None;
            }
        }
    }
    pub fn command(&self, game: &Game) -> Command {
        let mut result = Command::new(self.type_version_to_path().unwrap().to_str().unwrap().to_string());
        debug!("working dir {:?}", &game.path);
        result.current_dir(&game.path);
        result.arg("run".to_string());
        result.arg(&game.active_executable.as_ref().unwrap().path);
        result.envs(self.to_env_args(game.steam_id));
        result.stdout(std::process::Stdio::inherit());
        debug!("Returning command {:?}", &result);
        return result;
    }
    pub fn get_path(
        steam: &Steam,
        wine_type: &WineType,
        version: &str,
    ) -> Result<PathBuf, std::io::Error> {
        match wine_type {
            WineType::PROTON => {
                return steam.get_proton_path(version);
            }
            _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        }
    }
    fn to_env_args(&self, app_id: i64) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        match self.wine_type {
            WineType::PROTON => {
                result.insert(
                    "STEAM_COMPAT_DATA_PATH".to_string(),
                    self.prefix.to_str().unwrap().to_string(),
                );
                result.insert("SteamAppId".to_string(), app_id.to_string());
                result.insert("SteamGameId".to_string(), app_id.to_string());
            }
            _ => {
                result.insert(
                    "WINEPREFIX".to_string(),
                    self.prefix.to_str().unwrap().to_string(),
                );
                result.insert(
                    "WINEESYNC".to_string(),
                    Wine::bool_to_env_string(self.esync),
                );
                result.insert(
                    "STAGING_SHARED_MEMORY".to_string(),
                    Wine::bool_to_env_string(self.staging_memory),
                );
            }
        }
        debug!("args: {:?}", &result);
        return result;
    }
    fn bool_to_env_string(input: bool) -> String {
        match input {
            true => return String::from("1"),
            false => return String::from("0"),
        }
    }
}
