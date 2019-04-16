use gtk::ComboBoxText;
use gtk::Entry;
use gtk::Dialog;
use crate::gamepartial::GamePartial;
use crate::wine::Wine;
use crate::mogame::Game;
use crate::steam::Steam;
use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, ButtonsType, DialogFlags, FileChooserAction, FileChooserDialog,
    ListStore, MessageDialog, MessageType, ResponseType, Window,
};
use std::cell::RefCell;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

pub struct UIHelper {}

impl UIHelper {
    pub fn prompt_new_game(known_info: Option<GamePartial>) -> Option<Game> {
        // TODO - Actually return a proper Game
        let builder = gtk::Builder::new_from_string(include_str!("game_editor.glade"));
        let dialog: Dialog = builder.get_object("dialog_edit_game").unwrap();
        builder.get_object::<Entry>("edit_game_name").unwrap();
        builder.get_object::<Entry>("edit_game_steam_name").unwrap();
        builder.get_object::<ComboBoxText>("edit_game_wine_type").unwrap();
        builder.get_object::<ComboBoxText>("edit_game_wine_name").unwrap();
        builder.get_object::<Entry>("edit_game_wine_prefix").unwrap();
        debug!("New game dialog exit code {}", dialog.run()); // -4 is closed
        return None;
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
    pub fn dialog_text_input(title: &str, message: &str) -> String {
        let mut result = Rc::new(RefCell::new(String::new()));
        let dialog = gtk::MessageDialog::new::<MessageDialog>(
            None,
            DialogFlags::MODAL,
            MessageType::Question,
            ButtonsType::OkCancel,
            message,
        );
        let text_entry = gtk::Entry::new();
        let mut r = result.clone();
        text_entry.connect_activate(move |s| {
            r.borrow_mut().push_str(&s.get_text().unwrap());
        });
        dialog.run();
        return result.clone().borrow().clone();
    }
    pub fn dialog_path_crit(title: &str, on_err: Option<&str>) -> PathBuf {
        let dialog_choose_mod = FileChooserDialog::with_buttons::<&str, Window>(
            &title,
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
        let dialog_choose_mod = FileChooserDialog::with_buttons::<&str, Window>(
            &title,
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
    pub fn serde_dialog_text_input() -> String {
        return UIHelper::dialog_text_input(
                                          "Please provide the game's Steam name",
                                          &format!("Active game declared but cannot find configuration.\nThe game's Steam name is needed to proceed.")
                                      );
    }
}
