use std::path::PathBuf;
use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, ButtonsType, DialogFlags, MessageDialog, MessageType, FileChooserAction, FileChooserDialog, ResponseType, Window};

pub struct UIHelper {
}
impl UIHelper {
        pub fn dialog_path_crit(title: &str, main_window: &ApplicationWindow) -> PathBuf {
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
                        println!("{:?}", &v);
                        dialog_choose_mod.destroy();
                        return v;
                    }
                    None => dialog_choose_mod.destroy(),
                }
            }
            -6 => dialog_choose_mod.destroy(),
            other => {
                println!("Unknown FileChooserDialog response code: {}", other);
                dialog_choose_mod.destroy();
            }
        }
        let err_dialog: MessageDialog = MessageDialog::new(Some(main_window), DialogFlags::MODAL, MessageType::Error, ButtonsType::Close, "");
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
                        println!("{:?}", &v);
                        dialog_choose_mod.destroy();
                        return Some(v);
                    }
                    None => dialog_choose_mod.destroy(),
                }
            }
            -6 => dialog_choose_mod.destroy(),
            other => {
                println!("Unknown FileChooserDialog response code: {}", other);
                dialog_choose_mod.destroy();
            }
        }
        return None;
    }
}