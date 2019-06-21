use crate::{momod::ModModel};
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

pub fn generate_plugins_txt(mods: Vec<ModModel>) -> Vec<String> {
    debug!("Arr len {}", mods.len());
    let mut result = Vec::new();
    let mut list: BTreeMap<u64, Vec<String>> = BTreeMap::new();
    let mut index = 0;
    for m in mods {
        if m.enabled == true {
            //let mut index: u64 = lo;
            //debug!("{}", &lo);
            list.insert(index, Vec::new());
            let mut mod_data_path = m.get_path();
            mod_data_path.push("Data/");
            debug!("Mod data path: {:?}", &mod_data_path);
            for entry in WalkDir::new(mod_data_path)
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                match entry
                    .path()
                    .extension()
                    .unwrap_or(std::ffi::OsStr::new(""))
                    .to_str()
                {
                    Some("esm") => {
                        list.get_mut(&index).unwrap().push(format!(
                            "*{}",
                            entry
                                .path()
                                .file_name()
                                .unwrap()
                                .to_os_string()
                                .into_string()
                                .unwrap()
                        ));
                    }
                    Some("esp") => {
                        list.get_mut(&index).unwrap().push(format!(
                            "*{}",
                            entry
                                .path()
                                .file_name()
                                .unwrap()
                                .to_os_string()
                                .into_string()
                                .unwrap()
                        ));
                    }
                    _ => {}
                }
            }
        }
        index += 1;
    }
    for (k, v) in list {
        for m in v {
            result.push(m);
        }
    }
    debug!("Generated plugins.txt: {:?}", &result);
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
