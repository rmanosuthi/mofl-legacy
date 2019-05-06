use gtk::prelude::*;
use gtk::{Builder, Menu, MenuItem, MenuToolButton};
use relm::{execute, init, Component, ContainerComponent, ContainerWidget, EventStream, Relm, Update, Widget};

use crate::executable::{Executable, ExecutableModel, ExecutableMsg};
use crate::load::Load;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::fs::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Msg)]
pub enum Msg {
    SetActive(MenuItem),
    Start
}

#[derive(Clone)]
pub struct EMModel {
    exes: HashMap<MenuItem, ExecutableModel>
}

pub struct ExecutableManager {
    model: EMModel,
    view: MenuToolButton,
    view_list: Menu,
    exe_edit: MenuItem
}

impl Load for EMModel {
    fn load(path: &Path) -> Result<EMModel, Error> {
        let file = File::open(&path).expect("{:?}");
        let reader = BufReader::new(file);
        match serde_json::from_reader::<BufReader<File>, Vec<ExecutableModel>>(reader) {
            Ok(vec) => {
                debug!("Serde read ok");
                let mut model = HashMap::new();
                for m in vec {
                    debug!("Inserting element {:?}", &m);
                    let item = MenuItem::new_with_label(&m.label);
                    item.show();
                    model.insert(item, m);
                }
                return Ok(EMModel {exes: model});
            },
            Err(e) => {
                error!("Serde exe read failed");
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
            },
        }
    }
}

impl Update for ExecutableManager {
    type Model = EMModel;
    type ModelParam = PathBuf;
    type Msg = Msg;

    // stub
    fn model(_: &Relm<Self>, p: Self::ModelParam) -> Self::Model {
        return EMModel::load(&p).unwrap_or(EMModel {exes: HashMap::new()});
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::SetActive(name) => {},
            Msg::Start => {},
            _ => ()
        }
    }
}

impl Widget for ExecutableManager {
    type Root = MenuToolButton;

    fn root(&self) -> Self::Root {
        return self.view.clone();
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let builder = Builder::new_from_string(include_str!("window.glade"));
        let menu_sel_exe = builder.get_object::<MenuToolButton>("menu_sel_exe").unwrap();
        let menu_exe_list = builder.get_object::<Menu>("menu_exe_list").unwrap();
        let menu_exe_edit = builder.get_object::<MenuItem>("menu_exe_edit").unwrap();
        for entry in model.exes.keys() {
            debug!("Prepending");
            menu_exe_list.prepend(entry);
            //menu_exe_list.prepend(&MenuItem::new_with_label("SkyrimSe.exe"));
            //connect!(relm, entry.clone(), connect_activate(e), Msg::SetActive(e.clone()));
        }
        menu_exe_list.show_all();
        return ExecutableManager {
            model: model,
            view: menu_sel_exe,
            view_list: menu_exe_list,
            exe_edit: menu_exe_edit
        };
    }
}