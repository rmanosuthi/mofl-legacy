use relm::{Relm, Update, Widget};
use gtk::prelude::*;

use gtk::{ListBoxRow, Menu, MenuItem};

use std::path::PathBuf;
use std::process::{Child, Command};

use crate::game::GameModel;
use crate::wine::Wine;
use crate::vfs;

#[derive(Msg)]
pub enum ExecutableMsg {
    Modify(ExecutableModel),
    Start(GameModel, Wine),
    Stop
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutableModel {
    pub label: String,
    pub path: PathBuf,
    pub arguments: Vec<String>
}

pub struct Executable {
    model: ExecutableModel,
    view: MenuItem,
    menu: Menu,
    view_list: ListBoxRow
}

/*impl Update for Executable {
    type Model = ExecutableModel;
    type ModelParam = ExecutableModel;
    type Msg = ExecutableMsg;

    fn model(_: &Relm<Self>, p: Self::ModelParam) -> Self::Model {
        return p;
    }

    fn update(&mut self, msg: ExecutableMsg) {
        match msg {
            ExecutableMsg::Modify(e) => self.model = e,
            ExecutableMsg::Start(g, w) => {self.model.start(&g, &w);},
            ExecutableMsg::Stop => {}
        }
    }
}*/

/*impl Widget for Executable {
    type Root = MenuItem;

    fn root(&self) -> Self::Root {
        return self.view.clone();
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let widget = gtk::MenuItem::new_with_label(&model.label);
        return Executable {
            model: model,
            view: widget,
            view_list: gtk::ListBoxRow::new()
        };
    }
}*/

impl ExecutableModel {
    pub fn start(&self, game: &GameModel, wine: &Wine) -> Option<Child> {
        let mut game_data_path = game.path.clone();
        game_data_path.push("Data/");
        vfs::fuse_overlay_unmount(&game_data_path);
        // check if file exists
        // spawn child process
        match vfs::generate_vfs(&game) {
            Ok(path) => {
                game.write_plugins_txt();
                let mut cmd = self.command(&game, &wine);
                match cmd.spawn() {
                    Ok(mut child) => {
                        match child.try_wait() {
                            Ok(Some(status)) => debug!("Exited quickly"),
                            Ok(None) => {
                                let res = child.wait();
                                debug!("Exited");
                                vfs::fuse_overlay_unmount(&game_data_path);
                            }
                            Err(e) => debug!("Process error"),
                        }
                        return Some(child);
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        return None;
                    }
                }
            }
            Err(e) => return None,
        }
    }
    fn command(&self, game: &GameModel, wine: &Wine) -> Command {
        let mut result = Command::new(wine.type_version_to_path().unwrap().to_str().unwrap().to_string());
        debug!("working dir {:?}", &game.path);
        result.current_dir(&game.path);
        result.arg("run".to_string());
        result.arg(&self.path);
        result.envs(wine.to_env_args(game.steam_id));
        result.stdout(std::process::Stdio::inherit());
        debug!("Returning command {:?}", &result);
        return result;
    }
}