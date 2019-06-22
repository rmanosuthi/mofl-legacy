use crate::esp::EspModel;
use crate::momod::Mod;
use crate::gamestarter::GameStarter;
use crate::game::GameModel;
use crate::moenv::Environment;
use crate::mount::Mount;
use crate::special_game::SpecialGame;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use walkdir::WalkDir;

pub fn generate_plugins_txt(mods: &BTreeMap<String, Mod>) -> Vec<String> {
    debug!("Arr len {}", mods.len());
    let mut result = Vec::new();
    let mut map: BTreeMap<String, EspModel> = BTreeMap::new();
    for (_, m) in mods.iter() {
        for (idx, esp) in m.esps.iter() {
            map.insert(idx.to_string(), esp.model.clone());
        }
    }
    for (_, model) in map {
        result.push(model.file_name);
    }
    return result;
}

pub fn generate_vfs(gs: &GameStarter) -> Result<PathBuf, std::io::Error> {
    match gs.special {
        Some(SpecialGame::ESO) => (),
        None => match gs.mount {
            Mount::FUSE_OVERLAYFS => {
                let mut mod_data_paths: Vec<PathBuf> = Vec::with_capacity(gs.mods.len() + 1);
                let mut game_data_path = PathBuf::from(&gs.working_dir);
                game_data_path.push("Data/");
                mod_data_paths.push(game_data_path.clone());
                for ref m in &gs.mods {
                    let mut mod_data_path = m.get_path();
                    mod_data_path.push("Data/");
                    debug!("VFS mod data path {:?}", &mod_data_path);
                    mod_data_paths.push(mod_data_path);
                }
                match fuse_overlay_mount(
                    mod_data_paths,
                    PathBuf::from("/tmp/mofl/upper"),
                    PathBuf::from("/tmp/mofl/work"),
                    game_data_path,
                ) {
                    Ok(child) => (),
                    Err(e) => return Err(e),
                }
            }
            _ => return Err(std::io::Error::from(std::io::ErrorKind::Other)),
        },
    }
    return Ok(gs.working_dir.clone());
}

fn fuse_overlay_mount(
    lower: Vec<PathBuf>,
    upper: PathBuf,
    workdir: PathBuf,
    merged: PathBuf,
) -> Result<Child, std::io::Error> {
    std::fs::create_dir_all(&upper);
    std::fs::create_dir_all(&workdir);
    let mut command = Command::new("fuse-overlayfs");
    let mut lower_concat = String::from("lowerdir=");
    for path in lower {
        let mut escaped_path = path.to_str().unwrap().to_string();
        //escaped_path = escaped_path.replace(" ", r#" "#);
        lower_concat.push_str(&escaped_path);
        lower_concat.push(':');
    }
    lower_concat.pop();
    info!("Mounting {:?}", &merged);
    command
        .arg("-o")
        .arg(lower_concat)
        .arg("-o")
        .arg("upperdir=".to_owned() + upper.to_str().unwrap())
        .arg("-o")
        .arg("workdir=".to_owned() + workdir.to_str().unwrap())
        .arg(merged.to_str().unwrap());
    return command.spawn();
}

pub fn fuse_overlay_unmount(merged: &PathBuf) -> Result<Child, std::io::Error> {
    info!("Unmounting {:?}", &merged);
    return Command::new("fusermount").arg("-u").arg(&merged).spawn();
}
