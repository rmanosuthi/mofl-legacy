use crate::steam::Steam;
use crate::mogame::Executable;
use crate::mogame::Game;
use std::collections::HashMap;
use std::path::PathBuf;
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
    pub path: PathBuf,
    pub esync: bool,
    pub staging_memory: bool,
    pub wine_type: WineType,
}

impl Wine {
    pub fn command(&self, game: &Game) -> Command {
        let mut result = match self.wine_type {
            WineType::SYSTEM => Command::new(format!(
                "{}/bin/wine",
                &self.path.to_str().unwrap().to_string()
            )),
            WineType::LUTRIS => Command::new(format!(
                "{}/bin/wine",
                &self.path.to_str().unwrap().to_string()
            )),
            WineType::PROTON => Command::new(format!(
                "{}/proton",
                &self.path.to_str().unwrap().to_string()
            )),
        };
        debug!("working dir {:?}", &game.path);
        result.current_dir(&game.path);
        result.arg("run".to_string());
        result.arg(&game.active_executable.as_ref().unwrap().path);
        result.envs(self.to_env_args(game.steam_id));
        result.stdout(std::process::Stdio::inherit());
        debug!("Returning command {:?}", &result);
        return result;
    }
    pub fn get_path(steam: &Steam, wine_type: &WineType, version: &str) -> Result<PathBuf, std::io::Error> {
        match wine_type {
            WineType::PROTON => {
                return steam.get_proton_path(version);
            },
            _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
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
