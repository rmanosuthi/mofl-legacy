use glib::Sender;
use crate::gamestarter::GameStarter;
use gtk::prelude::*;
use gtk::{Builder, Menu, MenuItem, MenuToolButton};
use relm::{
    execute, init, Component, ContainerComponent, ContainerWidget, EventStream, Relm, Update,
    Widget,
};

use crate::executable::{Executable, ExecutableModel, ExecutableMsg, ExecutableStatus};
use crate::load::Load;

use std::collections::HashMap;
use std::fs::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Msg)]
pub enum Msg {
    SetActive(MenuItem),
    Init,
    Start(GameStarter, Sender<ExecutableStatus>),
}

#[derive(Clone)]
pub struct EMModel {
    exes: HashMap<MenuItem, ExecutableModel>,
}

pub struct ExecutableManager {
    model: EMModel,
    view: MenuToolButton,
    menu: Menu,
    exe_edit: MenuItem,
}

impl EMModel {
    fn load(relm: &Relm<ExecutableManager>, path: &Path) -> Result<EMModel, Error> {
        let file = std::fs::OpenOptions::new().append(true).create(true).open(&path).expect("executables.json failed to load");
        let reader = BufReader::new(file);
        match serde_json::from_reader::<BufReader<File>, Vec<ExecutableModel>>(reader) {
            Ok(vec) => {
                debug!("Serde read ok");
                let mut model = HashMap::new();
                for m in vec {
                    debug!("Inserting element {:?}", &m);
                    let item = MenuItem::new_with_label(&m.label);
                    connect!(relm, item, connect_activate(w), Msg::SetActive(w.clone()));
                    item.show();
                    model.insert(item, m);
                }
                return Ok(EMModel { exes: model });
            }
            Err(e) => {
                error!("Serde exe read failed: {:?}", e);
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
            }
        }
    }
}

impl Update for ExecutableManager {
    type Model = EMModel;
    type ModelParam = PathBuf;
    type Msg = Msg;

    // stub
    fn model(r: &Relm<Self>, p: Self::ModelParam) -> Self::Model {
        return EMModel::load(r, &p).unwrap_or(EMModel {
            exes: HashMap::new(),
        });
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::SetActive(menuitem) => {
                debug!("Set active");
                self.view.set_label(menuitem.get_label().unwrap().as_str());
            }
            Msg::Start(gs, sender) => {
                let selected_label = self.view.get_label().unwrap().as_str().to_string();
                for exe in self.model.exes.values() {
                    if exe.label == selected_label {
                        exe.start(gs, sender);
                        break;
                    }
                }
            }
            Msg::Init => {
                //self.view_list.remove_all();
                for entry in self.model.exes.keys() {
                    debug!("Prepending");
                    self.menu.prepend(entry);
                    //menu_exe_list.prepend(&MenuItem::new_with_label("SkyrimSe.exe"));
                    //connect!(self., entry.clone(), connect_activate(e), Msg::SetActive(e.clone()));
                }
                self.menu.show_all();
            }
            _ => (),
        }
    }
}

impl Widget for ExecutableManager {
    type Root = MenuToolButton;

    fn root(&self) -> Self::Root {
        return self.view.clone();
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let menu_exe: MenuToolButton = gtk::MenuToolButton::new::<Menu, _, &str>(None, "Select executable...");
        let menu_exe_list = Menu::new();
        menu_exe.set_menu(&menu_exe_list);
        let menu_exe_edit = MenuItem::new_with_label("Edit");
        //connect!(relm, menu_exe_list, connect_show(_), Msg::Init);
        /*debug!("{:?}", menu_sel_exe.get_menu());
        debug!("{:?}", menu_sel_exe.get_label());
        menu_sel_exe.set_label(Some("SkyrimSE.exe"));
        debug!("{:?}", menu_sel_exe.get_label());
        menu_exe_list.show_all();*/
        //         let menu_exe_list = builder.get_object::<gtk::Menu>("menu_exe_list").unwrap();
        // menu_exe_list.prepend(&gtk::MenuItem::new_with_label("SkyrimSE.exe"));
        // menu_exe_list.show_all();
        /*for entry in model.exes.keys() {
            debug!("Prepending");
            menu_exe_list.prepend(entry);
            //menu_exe_list.prepend(&MenuItem::new_with_label("SkyrimSe.exe"));
            //connect!(relm, entry.clone(), connect_activate(e), Msg::SetActive(e.clone()));
        }*/
        //menu_exe_list.prepend(&MenuItem::new_with_label("test.exe"));
        //menu_exe_list.show_all();
        return ExecutableManager {
            model: model,
            view: menu_exe,
            menu: menu_exe_list,
            exe_edit: menu_exe_edit,
        };
    }
}
