use crate::game::GameModel;
use crate::game::partial::GamePartial;
use crate::moconfig::Config;
use crate::game::momod::{Mod, ModModel};
use crate::mount::Mount;
use crate::setupinstance::SetupInstance;
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
use gtk::InputPurpose;
use gtk::Notebook;
use gtk::{
    Application, ApplicationWindow, Assistant, Builder, ButtonsType, DialogFlags,
    FileChooserAction, FileChooserDialog, ListStore, MessageDialog, MessageType, ResponseType,
    Window,
};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use std::io::{BufRead, BufReader, Read};

use chrono::DateTime;

type Esp = String;

pub struct UIHelper {}

impl UIHelper {
    /* In an ideal world, gtk::Assistant would be used instead of gtk::Dialog.
     * However, gtk::Assistant doesn't have run(), which is supposed to wait for input.
     * Implementing a manual wait is a nightmare (working with gtk-rs is, actually) and so gtk::Dialog shall be used for the time being. */
    pub fn first_setup() -> Option<SetupInstance> {
        let mut result_games: Rc<RefCell<Vec<GameModel>>> = Rc::new(RefCell::new(Vec::new()));
        let builder = Builder::new_from_string(include_str!("setup.glade"));
        let dialog: Dialog = Dialog::new_with_buttons::<&'static str, Window>(
            "Edit game",
            None,
            DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        let setup_box: gtk::Box = builder.get_object::<gtk::Box>("setup_box").unwrap();
        let bt_add_game: gtk::ToolButton = builder
            .get_object::<gtk::ToolButton>("bt_add_game")
            .unwrap();
        dialog.get_content_area().add(&setup_box);
        // events
        let r_g = result_games.clone();
        bt_add_game.connect_clicked(move |m| match UIHelper::prompt_new_game() {
            Some(game) => r_g.as_ref().borrow_mut().push(game),
            None => (),
        });
        // also save before returning!
        match dialog.run() {
            -5 => {
                // Unnecessary allocation but the alternative is painful, trust me
                let games: Vec<GameModel> = result_games.as_ref().replace(Vec::new());
                dialog.destroy();
                return Some(SetupInstance {
                    games: games,
                    steam: Steam::new_from_config(), // FIX USE INPUT
                    config: Config {
                        active_game: None,
                        mofl_version: "0.1.0".to_string(),
                        steam: Rc::new(Steam::new_from_config()), // FIX USE INPUT
                    },
                    active_idx: 0,
                });
            }
            _ => {
                dialog.destroy();
                return None;
            }
        }
    }
    /// Prompts a dialog for a new game.
    pub fn prompt_new_game() -> Option<GameModel> {
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
        let field_steam_id = builder.get_object::<Entry>("edit_game_steam_id").unwrap();
        field_steam_id.connect_changed(|f| {});
        let field_working_dir = builder
            .get_object::<Entry>("edit_game_working_dir")
            .unwrap();
        let field_mount = builder
            .get_object::<ComboBoxText>("edit_game_mount")
            .unwrap();
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
            for entry in Wine::get_versions(
                &Steam::new_from_config(),
                UIHelper::get_wine_type(&f_w_t).unwrap(),
            )
            .unwrap()
            {
                f_w_v.append_text(&entry.0);
            }
            f_w_v.set_active(0); // CHECK
        });
        //field_name.set_text(&v.label.unwrap_or_default());
        //field_steam_name.set_text(&v.steam_label.unwrap_or_default());
        field_wine_type.remove_all();
        field_wine_version.remove_all();
        //field_wine_type.append_text(&format!("{:?}", wine.wine_type));
        //field_wine_version.append_text(&format!("{:?}", wine.path));
        for wine_type in Wine::get_types() {
            field_wine_type.append_text(&wine_type);
        }
        field_wine_type.set_active(0);
        //field_wine_type.set_active()
        let mut counter = 0;
        for entry in Wine::get_versions(
            &Steam::new_from_config(),
            UIHelper::get_wine_type(&field_wine_type).unwrap(),
        )
        .unwrap()
        {
            field_wine_version.append_text(&entry.0);
            field_wine_version.set_active(0);
            counter += 1;
        }
        field_wine_prefix.set_text("");
        field_esync.set_active(false);
        field_staging_memory.set_active(false);
        field_mount.append_text("FUSE Overlayfs (default)");
        field_mount.append_text("System Overlayfs");
        field_mount.append_text("Ignore");
        field_mount.set_active(0);
        match dialog.run() {
            -5 => {
                let result = Some(GameModel {
                    label: field_name.get_text().unwrap().to_string(),
                    steam_label: field_steam_name.get_text().unwrap().to_string(),
                    special: None,
                    path: PathBuf::from(field_working_dir.get_text().unwrap().to_string()),
                    wine: Wine {
                        prefix: PathBuf::from(field_wine_prefix.get_text().unwrap().as_str()),
                        version: UIHelper::get_wine_version(&field_wine_version),
                        //path: Wine::get_path(&steam, &UIHelper::get_wine_type(&field_wine_type).unwrap(), &UIHelper::get_wine_version(&field_wine_version)).unwrap(),
                        esync: field_esync.get_active(),
                        staging_memory: field_staging_memory.get_active(),
                        wine_type: UIHelper::get_wine_type(&field_wine_type).unwrap(),
                    },
                    mount: UIHelper::get_mount(&field_mount),
                    steam_id: field_steam_id
                        .get_text()
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap()
                });
                dialog.destroy();
                return result;
            }
            rt => {
                debug!("{:?}", rt);
                dialog.destroy();
                return None;
            }
        }
    }
    /// Prompts a dialog to edit an existing game.
    pub fn prompt_edit_game(game: &mut GameModel) -> bool {
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
        let field_steam_id = builder.get_object::<Entry>("edit_game_steam_id").unwrap();
        field_steam_id.set_input_purpose(InputPurpose::Number);
        let field_mount = builder
            .get_object::<ComboBoxText>("edit_game_mount")
            .unwrap();
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
            for entry in Wine::get_versions(
                &Steam::new_from_config(),
                UIHelper::get_wine_type(&f_w_t).unwrap(),
            )
            .unwrap()
            {
                f_w_v.append_text(&entry.0);
            }
        });
        field_mount.append_text("FUSE Overlayfs (default)");
        field_mount.append_text("System Overlayfs");
        field_mount.append_text("Ignore");
        field_name.set_text(&game.label);
        field_steam_name.set_text(&game.steam_label);
        field_wine_type.remove_all();
        field_wine_version.remove_all();
        for wine_type in Wine::get_types() {
            field_wine_type.append_text(&wine_type);
        }
        field_wine_type.set_active(game.wine.type_to_idx());
        //field_wine_type.set_active()
        let mut counter = 0;
        for entry in Wine::get_versions(
            &Steam::new_from_config(),
            UIHelper::get_wine_type(&field_wine_type).unwrap(),
        )
        .unwrap()
        {
            field_wine_version.append_text(&entry.0);
            if entry.0 == game.wine.version {
                field_wine_version.set_active(counter);
            }
            counter += 1;
        }
        field_wine_prefix.set_text(game.wine.prefix.to_str().unwrap());
        field_esync.set_active(game.wine.esync);
        field_staging_memory.set_active(game.wine.staging_memory);
        field_steam_id.set_text(&game.steam_id.to_string());
        field_mount.set_active(UIHelper::mount_to_sel(&game.mount));
        /*match known_info {
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
                            &Steam::new_from_config(),
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
                match v.steam_id {
                    Some(id) => field_steam_id.set_text(&id.to_string()),
                    None => warn!("Got an empty steam_id from an existing Game, which should never happen!")
                }
                field_mount.set_active(UIHelper::mount_to_sel(&v.mount.unwrap()));
            }
            None => (),
        }*/
        //field_wine_type.set_active(Some(0));
        match dialog.run() {
            -5 => {
                /*let result = Some(GamePartial {
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
                    mount: UIHelper::get_mount(&field_mount),
                    steam_id: Some(
                        field_steam_id
                            .get_text()
                            .unwrap()
                            .as_str()
                            .parse::<i64>()
                            .unwrap(),
                    ),
                });*/
                game.label = field_name.get_text().unwrap().to_string();
                game.steam_label = field_steam_name.get_text().unwrap().to_string();
                //game.special = ();
                game.wine = Wine {
                    prefix: PathBuf::from(field_wine_prefix.get_text().unwrap().as_str()),
                    version: UIHelper::get_wine_version(&field_wine_version),
                    //path: Wine::get_path(&steam, &UIHelper::get_wine_type(&field_wine_type).unwrap(), &UIHelper::get_wine_version(&field_wine_version)).unwrap(),
                    esync: field_esync.get_active(),
                    staging_memory: field_staging_memory.get_active(),
                    wine_type: UIHelper::get_wine_type(&field_wine_type).unwrap(),
                };
                game.steam_id = field_steam_id
                    .get_text()
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
                game.mount = UIHelper::get_mount(&field_mount);
                dialog.destroy();
                return true;
            }
            _ => {
                dialog.destroy();
                return false;
            }
        }
    }
    // stub
    pub fn prompt_edit_mod(m: &mut Mod) -> bool {
        return false;
    }
    // TODO: Extract mod and create config
    pub fn prompt_install_mod(game_name: &str, list_store: Rc<ListStore>, esp_list_store: Rc<ListStore>) -> Option<Mod> {
        let file_path = UIHelper::dialog_path("Please select a mod to install", FileChooserAction::Open)?;
        let fp = file_path.clone();
        let builder = Builder::new_from_string(include_str!("mod_editor.glade"));
        let console = builder
            .get_object::<gtk::TextView>("edit_mod_console")
            .unwrap();
        let console_buffer = console.get_buffer().unwrap();
        // Threading magic
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        thread::spawn(move || {
            //thread::sleep(Duration::from_secs(10));
            let mut dest = crate::moenv::Environment::get_home();
            dest.push(".config/mofl/.tmp_mod_install");
            dest.push(fp.file_name().unwrap());
            std::fs::create_dir_all(&dest);
            let mut extract_process = std::process::Command::new("7z")
                .arg("x")
                .arg(&fp)
                .arg("-o".to_string() + dest.to_str().unwrap())
                .arg("-aoa") // overwrite all files
                .stdout(std::process::Stdio::piped())
                .spawn()
                .unwrap();
            // https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process
            let stdout = extract_process.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();
            for line in stdout_lines {
                let s = sender.send(line.unwrap() + "\n");
            }
            match extract_process.wait() {
                Ok(v) => {sender.send(String::from("/////MOFL_EXTRACT_DONE/////"))},
                Err(e) => sender.send(String::from("/////MOFL_EXTRACT_ERROR/////"))
            };
            
        });
        receiver.attach(None, move |text| {
            match text.as_ref() {
                "/////MOFL_EXTRACT_DONE/////" => info!("Extract successful!"),
                "/////MOFL_EXTRACT_ERROR/////" => error!("Extract failed"),
                _ => console_buffer.insert_at_cursor(&text)
            }

            glib::Continue(true)
        });
        // End of threading magic
        let dialog: Dialog = Dialog::new_with_buttons::<&'static str, Window>(
            "Install mod",
            None,
            DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        let notebook: Notebook = builder.get_object("edit_mod_notebook").unwrap();
        let field_label = builder.get_object::<Entry>("edit_mod_label").unwrap();
        let field_version = builder.get_object::<Entry>("edit_mod_version").unwrap();
        let field_category = builder
            .get_object::<ComboBoxText>("edit_mod_category")
            .unwrap();
        let field_last_updated = builder.get_object::<Entry>("field_last_updated").unwrap();
        let field_nexus_id = builder.get_object::<Entry>("edit_mod_nexus_id").unwrap();
        let field_enabled = builder
            .get_object::<CheckButton>("edit_mod_enabled")
            .unwrap();
        dialog.get_content_area().add(&notebook);
        match dialog.run() {
            -5 => {
                let new_model = ModModel {
                    enabled: field_enabled.get_active(),
                    label: field_label.get_text().unwrap().as_str().to_string(),
                    version: field_version.get_text().unwrap().as_str().to_string(),
                    category: None,
                    updated: match DateTime::parse_from_rfc3339(
                        field_last_updated.get_text().unwrap().as_str(),
                    ) {
                        Ok(v) => v.with_timezone(&chrono::offset::Utc),
                        Err(_) => chrono::offset::Utc::now(),
                    },
                    nexus_id: match field_nexus_id.get_text().unwrap().as_str().parse::<i64>() {
                        Ok(id) => Some(id),
                        Err(_) => None,
                    },
                    game_name: game_name.to_string()
                };

                let mut move_src = crate::moenv::Environment::get_home();
                move_src.push(".config/mofl/.tmp_mod_install");
                move_src.push(file_path.file_name().unwrap());
                let mut move_dest = new_model.get_path();
                move_dest.push("Data");
                std::fs::create_dir_all(&move_dest);
                for entry in walkdir::WalkDir::new(&move_src)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    fs_extra::move_items(
                        &vec![entry.path()],
                        &move_dest,
                        &fs_extra::dir::CopyOptions {
                            overwrite: true,
                            skip_exist: false,
                            buffer_size: 64000,
                            copy_inside: true,
                            depth: 0,
                        },
                    );
                }
                std::fs::remove_dir_all(&move_src);
                dialog.destroy();
                return Some(Mod::new(new_model, list_store, esp_list_store));
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
        match field.get_active_text() {
            Some(ver) => return ver.as_str().to_string(),
            None => {
                warn!("Missing wine version, is wine installed?");
                return "ERR_NO_WINE".to_string();
            }
        }
        //return field.get_active_text().unwrap().as_str().to_string();
    }
    fn mount_to_sel(mount: &Mount) -> u32 {
        match mount {
            Mount::FUSE_OVERLAYFS => return 0,
            Mount::SYS_OVERLAYFS => return 1,
            Mount::IGNORE => return 2
        }
    }
    fn get_mount(field: &ComboBoxText) -> Mount {
        match field.get_active_text().unwrap().as_str() {
            "FUSE Overlayfs (default)" => return Mount::FUSE_OVERLAYFS,
            "System Overlayfs" => return Mount::SYS_OVERLAYFS,
            _ => panic!(),
        }
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
    pub fn dialog_err_gp(field: &str) -> ! {
        let message = format!(
            "Creating Game from GamePartial but a mandatory field '{}' is missing.\n\
             This should've been validated at the UI stage and is not supposed to happen.\n\
             Please file a bug report at https://github.com/mpipo/mofl/issues",
            &field
        );
        error!("Creating Game from GamePartial, missing field {}", &field);
        UIHelper::dialog_err(&message, false);
    }
    pub fn dialog_err(message: &str, log: bool) -> ! {
        let dialog = gtk::MessageDialog::new::<MessageDialog>(
            None,
            DialogFlags::MODAL,
            MessageType::Error,
            ButtonsType::Close,
            &message,
        );
        if log == true {
            error!("{}", &message)
        };
        dialog.run();
        panic!();
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
    pub fn dialog_path(title: &str, action: FileChooserAction) -> Option<PathBuf> {
        let dialog_choose_mod = FileChooserDialog::with_buttons::<&str, Window>(
            &title,
            None,
            action,
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
    pub fn dialog(text: &str) {}
    pub fn serde_dialog_text_input() -> String {
        return UIHelper::dialog_text_input(
                                          "Please provide the game's Steam name",
                                          &format!("Active game declared but cannot find configuration.\nThe game's Steam name is needed to proceed.")
                                      );
    }
}
