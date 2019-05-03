use relm::{Relm, Update, Widget};
use gtk::prelude::*;

use gtk::{ListBoxRow, MenuItem};

use std::path::PathBuf;

#[derive(Msg)]
pub enum ExecutableMsg {
    Modify(ExecutableModel)
}

#[derive(Serialize, Deserialize)]
pub struct ExecutableModel {
    label: String,
    path: PathBuf,
    arguments: Vec<String>
}

pub struct Executable {
    model: ExecutableModel,
    view: MenuItem,
    view_list: ListBoxRow
}

impl Update for Executable {
    type Model = ExecutableModel;
    type ModelParam = (String, PathBuf, Vec<String>);
    type Msg = ExecutableMsg;

    fn model(_: &Relm<Self>, p: Self::ModelParam) -> Self::Model {
        return ExecutableModel {
            label: p.0,
            path: p.1,
            arguments: p.2
        };
    }

    fn update(&mut self, msg: ExecutableMsg) {
        match msg {
            ExecutableMsg::Modify(e) => self.model = e
        }
    }
}

impl Widget for Executable {
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
}