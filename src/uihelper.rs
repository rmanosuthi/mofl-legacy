use crate::gamepartial::GamePartial;
use crate::mogame::Game;
use crate::momod::Mod;
use crate::steam::Steam;
use crate::wine::Wine;
use crate::wine::WineType;
use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::CheckButton;
use gtk::ComboBoxText;
use gtk::Dialog;
use gtk::Entry;
use gtk::Notebook;
use gtk::{
    Application, ApplicationWindow, Builder, ButtonsType, DialogFlags, FileChooserAction,
    FileChooserDialog, ListStore, MessageDialog, MessageType, ResponseType, Window,
};
use std::cell::RefCell;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

pub struct UIHelper {}

impl UIHelper {
    // stub
    pub fn prompt_new_game(steam: &Steam, known_info: Option<GamePartial>) -> Option<Game> {
        return None;
    }
    pub fn prompt_edit_game(
        steam: Rc<Steam>,
        known_info: Option<GamePartial>,
    ) -> Option<GamePartial> {
        let c_steam = steam.clone();
        let dialog: Dialog = Dialog::new_with_buttons::<&'static str, Window>(
            "Edit game",
            None,
            DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        let builder = Builder::new_from_string(include_str!("game_editor_new.glade"));
        let notebook: Notebook = builder.get_object("edit_game_notebook").unwrap();
        let field_name = builder.get_object::<Entry>("edit_game_name").unwrap();
        let field_steam_name = builder.get_object::<Entry>("edit_game_steam_name").unwrap();
        let field_wine_type = builder
            .get_object::<ComboBoxText>("edit_game_wine_type")
            .unwrap();
        let field_wine_version = builder
            .get_object::<ComboBoxText>("edit_game_wine_version")
            .unwrap();
        let field_wine_prefix = builder
            .get_object::<Entry>("edit_game_wine_prefix")
            .unwrap();
        let field_esync = builder
            .get_object::<CheckButton>("edit_game_esync")
            .unwrap();
        let field_staging_memory = builder
            .get_object::<CheckButton>("edit_game_staging_memory")
            .unwrap();
        let f_w_t = field_wine_type.clone();
        let f_w_v = field_wine_version.clone();
        dialog.get_content_area().add(&notebook);
        field_wine_type.connect_changed(move |e| {
            debug!("{:?}", e.get_active());
            f_w_v.remove_all();
            for entry in
                Wine::get_versions(&c_steam, UIHelper::get_wine_type(&f_w_t).unwrap()).unwrap()
            {
                f_w_v.append_text(&entry.0);
            }
        });
        match known_info {
            Some(v) => {
                field_name.set_text(&v.label.unwrap_or_default());
                field_steam_name.set_text(&v.steam_label.unwrap_or_default());
                field_wine_type.remove_all();
                field_wine_version.remove_all();
                match v.wine {
                    Some(wine) => {
                        //field_wine_type.append_text(&format!("{:?}", wine.wine_type));
                        //field_wine_version.append_text(&format!("{:?}", wine.path));
                        for wine_type in Wine::get_types() {
                            field_wine_type.append_text(&wine_type);
                        }
                        field_wine_type.set_active(wine.type_to_idx());
                        //field_wine_type.set_active()
                        let mut counter = 0;
                        for entry in Wine::get_versions(
                            &steam,
                            UIHelper::get_wine_type(&field_wine_type).unwrap(),
                        )
                        .unwrap()
                        {
                            field_wine_version.append_text(&entry.0);
                            if entry.0 == wine.version {
                                field_wine_version.set_active(counter);
                            }
                            counter += 1;
                        }
                        field_wine_prefix.set_text(wine.prefix.to_str().unwrap());
                        field_esync.set_active(wine.esync);
                        field_staging_memory.set_active(wine.staging_memory);
                    }
                    None => (),
                }
            }
            None => (),
        }
        //field_wine_type.set_active(Some(0));
        match dialog.run() {
            -5 => {
                let result = Some(GamePartial {
                    label: Some(field_name.get_text().unwrap().to_string()),
                    steam_label: Some(field_steam_name.get_text().unwrap().to_string()),
                    special: None,
                    wine: Some(Wine {
                        prefix: PathBuf::from(field_wine_prefix.get_text().unwrap().as_str()),
                        version: UIHelper::get_wine_version(&field_wine_version),
                        //path: Wine::get_path(&steam, &UIHelper::get_wine_type(&field_wine_type).unwrap(), &UIHelper::get_wine_version(&field_wine_version)).unwrap(),
                        esync: field_esync.get_active(),
                        staging_memory: field_staging_memory.get_active(),
                        wine_type: UIHelper::get_wine_type(&field_wine_type).unwrap(),
                    }),
                });
                dialog.destroy();
                return result;
            }
            _ => {
                dialog.destroy();
                return None;
            }
        }
    }
    // TODO: Extract mod and create config
    pub fn prompt_install_mod(
        game_path: Rc<PathBuf>,
        list_store: Option<Rc<ListStore>>,
    ) -> Option<Mod> {
        let file_path = UIHelper::dialog_path("Please select a mod to install")?;
        // Threading magic
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        thread::spawn(move || {
            //thread::sleep(Duration::from_secs(10));
            let mut dest = crate::moenv::Environment::get_home();
            dest.push(".config/mofl/.tmp_mod_install");
            dest.push(file_path.file_name().unwrap());
            std::fs::create_dir_all(&dest);
            let extract_process = std::process::Command::new("7z")
                .arg("x")
                .arg(&file_path)
                .arg("-o".to_string() + dest.to_str().unwrap())
                .stdout(std::process::Stdio::inherit())
                .spawn();
            // Sending fails if the receiver is closed
            let _ = sender.send("");
        });
        // End of threading magic
        let dialog: Dialog = Dialog::new_with_buttons::<&'static str, Window>(
            "Install mod",
            None,
            DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        let builder = Builder::new_from_string(include_str!("mod_editor.glade"));
        let notebook: Notebook = builder.get_object("edit_mod_notebook").unwrap();
        let field_label = builder.get_object::<Entry>("edit_mod_label").unwrap();
        let field_version = builder.get_object::<Entry>("edit_mod_version").unwrap();
        let field_category = builder
            .get_object::<ComboBoxText>("edit_mod_category")
            .unwrap();
        let field_updated = builder.get_object::<Entry>("edit_mod_updated").unwrap();
        let field_nexus_id = builder.get_object::<Entry>("edit_mod_nexus_id").unwrap();
        let field_enabled = builder
            .get_object::<CheckButton>("edit_mod_enabled")
            .unwrap();
        dialog.get_content_area().add(&notebook);
        match dialog.run() {
            -5 => {
                let mut result = Mod {
                    enabled: field_enabled.get_active(),
                    load_order: None,
                    label: field_label.get_text().unwrap().as_str().to_string(),
                    version: field_version.get_text().unwrap().as_str().to_string(),
                    category: -1,
                    updated: field_updated
                        .get_text()
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap(),
                    nexus_id: field_nexus_id
                        .get_text()
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap(),
                    game_path: game_path,
                    list_store: list_store,
                    tree_iter: None,
                };
                result.set_tree_iter();
                dialog.destroy();
                return Some(result);
            }
            _ => {
                dialog.destroy();
                return None;
            }
        }
    }
    fn get_wine_type(field: &ComboBoxText) -> Option<WineType> {
        match field.get_active_text().unwrap().as_str() {
            "System" => return Some(WineType::SYSTEM),
            "Lutris" => return Some(WineType::LUTRIS),
            "Proton" => return Some(WineType::PROTON),
            _ => return None,
        }
    }
    fn get_wine_version(field: &ComboBoxText) -> String {
        return field.get_active_text().unwrap().as_str().to_string();
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
