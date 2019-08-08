use crate::game::GameModel;
use crate::mount::Mount;
use crate::wine::Wine;
use std::path::PathBuf;
use gtk::prelude::*;
use gtk::{Builder, Dialog};
use relm::{Channel, Relm, Widget, Update};
use relm_derive::widget;

#[derive(Msg)]
enum Msg {
    ChangedLabel(String),
    ChangedSteamLabel(String),
    ChangedPath(PathBuf),
    ChangedSteamId(i64),
    ChangedWine(Wine),
    ChangedMount(Mount)
}

struct GameEditor {
    model: GameModel,
    view: Dialog
}

impl Update for GameEditor {
    type Model = GameModel;
    type ModelParam = GameModel;
    type Msg = Msg;

    fn model(_: &Relm<Self>, p: Self::ModelParam) -> Self::Model {
        p
    }
    fn update(&mut self, event: Msg) {

    }
}

impl Widget for GameEditor {
    type Root = Dialog;

    fn root(&self) -> Self::Root {
        self.view.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let builder = Builder::new_from_string(include_str!("editor.glade"));
        let view = builder.get_object::<Dialog>("").unwrap();
        Self {
            view: view,
            model: model
        }
    }
}