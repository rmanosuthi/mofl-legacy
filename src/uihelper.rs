use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, ButtonsType, DialogFlags, FileChooserAction, FileChooserDialog,
    MessageDialog, MessageType, ResponseType, Window,
};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use crate::mogame::Game;
use crate::steam::Steam;

pub struct UIHelper {}
impl UIHelper {
    pub fn prompt_new_game(steam: Rc<Steam>) -> Game {
        // TODO - Actually return a proper Game
        return Game::new("".to_string(), steam, None);
    }
    pub fn serde_err(path: &Path, err: &serde_json::error::Error) {
        let err_message_1 = format!("(De)serialization error from file {:?}", &path);
        let err_message_2 = format!("{:?}", &err);
        let err_dialog: MessageDialog = MessageDialog::new::<MessageDialog>(
            None,
            DialogFlags::MODAL,
            MessageType::Error,
            ButtonsType::Close,
            &format!("\n{}:\n\n{}", &err_message_1, &err_message_2),
        );
        error!("{}: {}", &err_message_1, &err_message_2);
        err_dialog.run();
        panic!();
    }
    pub fn dialog_path_crit(title: &str, on_err: Option<&str>) -> PathBuf {
        let dialog_choose_mod = FileChooserDialog::with_buttons::<Window>(
            Some(&title),
            None,
            FileChooserAction::Open,
            &[
                ("_Cancel", ResponseType::Cancel),
                ("_Open", ResponseType::Accept),
            ],
        );
        match dialog_choose_mod.run() {
            -3 => {
                // -3 is open, -6 is cancel
                match dialog_choose_mod.get_filename() {
                    Some(v) => {
                        debug!("{:?}", &v);
                        dialog_choose_mod.destroy();
                        return v;
                    }
                    None => dialog_choose_mod.destroy(),
                }
            }
            -6 => dialog_choose_mod.destroy(),
            other => {
                warn!("Unknown FileChooserDialog response code: {}", other);
                dialog_choose_mod.destroy();
            }
        }
        let err_dialog: MessageDialog = MessageDialog::new::<MessageDialog>(
            None,
            DialogFlags::MODAL,
            MessageType::Error,
            ButtonsType::Close,
            on_err.unwrap_or(""),
        );
        err_dialog.run();
        panic!("A file/folder has to be selected!");
    }
    pub fn dialog_path(title: &str) -> Option<PathBuf> {
        let dialog_choose_mod = FileChooserDialog::with_buttons::<Window>(
            Some(&title),
            None,
            FileChooserAction::Open,
            &[
                ("_Cancel", ResponseType::Cancel),
                ("_Open", ResponseType::Accept),
            ],
        );
        match dialog_choose_mod.run() {
            -3 => {
                // -3 is open, -6 is cancel
                match dialog_choose_mod.get_filename() {
                    Some(v) => {
                        debug!("{:?}", &v);
                        dialog_choose_mod.destroy();
                        return Some(v);
                    }
                    None => dialog_choose_mod.destroy(),
                }
            }
            -6 => dialog_choose_mod.destroy(),
            other => {
                warn!("Unknown FileChooserDialog response code: {}", other);
                dialog_choose_mod.destroy();
            }
        }
        return None;
    }
    // This should never be called anyway
    pub fn blank_app_window() -> Rc<ApplicationWindow> {
        panic!("blank_app_window() called (serde tried to deserialize a skipped field)");
    }
}
