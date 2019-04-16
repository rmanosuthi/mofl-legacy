use gtk::CellRendererToggle;
use gtk::TreeViewColumn;
use crate::mo2;
use crate::moconfig::Config;
use crate::moenv::Environment;
use crate::mogame::Game;
use crate::momod::Mod;
use crate::steam::Steam;
use crate::uihelper::UIHelper;
use crate::vfs;
use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::ResponseType;
use gtk::{
    ApplicationWindow, Builder, Button, Dialog, FileChooserAction, FileChooserDialog, ListStore,
    Menu, MenuItem, MenuToolButton, ToolButton, TreeStore, Window, WindowType,
};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
//use moenv::Environment;
pub const DEFAULT_PATH: &'static str = ".config/mofl";

pub struct UI {
    game: Rc<RefCell<Game>>,
    config: Config,
    builder: Rc<Builder>,
    main_window: Rc<ApplicationWindow>
}
impl UI {
    pub fn new(builder: gtk::Builder) -> UI {
        match Config::init_folders() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        };
        let mut tmp_path = Environment::get_home();
        tmp_path.push(DEFAULT_PATH);
        info!("Config path is {:?}", &tmp_path);
        tmp_path.push("config.json");
        let mut config: Config = match Config::load(&tmp_path) {
            Some(v) => v,
            None => panic!("Failed to create new config"),
        };
        debug!("{:?}", &config);
        let mut tmp_game = match Game::from(&mut config, Rc::new(builder.get_object::<ListStore>("liststore-mod-list").unwrap())) {
            Some(v) => v,
            None => panic!("No active game defined"),
        };
        tmp_game.add_mods_from_folder();
        //tmp_game.print_mod_folders();
        info!("Loaded game {} with {} mods", &tmp_game.label, tmp_game.mods.len());
        let game = Rc::new(RefCell::new(tmp_game));
        UI {
            game: game,
            config: config,
            builder: Rc::new(builder.clone()),
            main_window: Rc::new(builder.get_object("mowindow").unwrap())
        }
    }
    pub fn register_events(&self) {
        let pref_window = self
            .builder
            .get_object::<Dialog>("window-preferences")
            .unwrap();
        let bt_run_exe: ToolButton = self.builder.get_object::<ToolButton>("bt-run-exe").unwrap();

        let window_preferences_bt_close = self
            .builder
            .get_object::<Button>("window-preferences-bt-close")
            .unwrap();
        {
            let pref_window = pref_window.clone();
            window_preferences_bt_close.connect_clicked(move |_| {
                debug!("Closing preferences");
                pref_window.emit_close();
            });
        }
        let bt_add_mod = self.builder.get_object::<ToolButton>("bt-add-mod").unwrap();
        let local_game = self.game.clone();
        bt_add_mod.connect_clicked(move |_| {
            debug!("Showing mod select dialog");
            let dialog_choose_mod = FileChooserDialog::with_buttons::<&str, Window>(
                "Open File",
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
                            debug!("{:?}", v);
                            local_game.as_ref().borrow_mut().import(v);
                            dialog_choose_mod.destroy();
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
            //Window::new(WindowType::Toplevel).show();
        });
        let exe_edit: MenuItem = self
            .builder
            .get_object::<MenuItem>("menu-sel-exe-edit")
            .unwrap();
        let edit_pref: MenuItem = self.builder.get_object("gtk-preferences").unwrap();

        exe_edit.connect_activate(move |_| {});
        pref_window.connect_delete_event(move |win, _| {
            win.hide();
            Inhibit(true)
        });
        edit_pref.connect_activate(move |_| {
            debug!("Preferences clicked");
            &pref_window.show();
        });
        let handle = self.game.clone();
        bt_run_exe.connect_clicked(move |_| {
            handle.borrow_mut().start();
        });
        let handle = self.game.clone();
        let modview_toggle_column = self.builder.get_object::<CellRendererToggle>("modview_toggle_column").unwrap();
        modview_toggle_column.connect_toggled(move |e, t| {
            println!("{:?}", e);
            println!("{:?}", &t);
            handle.borrow_mut().toggle_mod_enable(t);
        });
        let handle = self.game.clone();
        self.builder.get_object::<ToolButton>("bt_edit_game").unwrap().connect_clicked(move |_| {
            UIHelper::prompt_new_game(None);
        });
    }
    pub fn build_ui(&self, application: &gtk::Application) {
        self.register_events();
        self.main_window.set_application(application);
        self.main_window.set_title("Mod Organizer for Linux");
        self.main_window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });
        self.main_window.show_all();
        let category_list: ListStore = self.builder.get_object("liststore-mod-categories").unwrap();
        //let mut tmp_path: PathBuf = PathBuf::from(env::var_os("HOME").unwrap());
        //let mut exe_list: ListStore = self.builder.get_object("liststore-runtimes").unwrap();
        self.game.as_ref().borrow().save();
        let menu_exe_list = self.builder.get_object::<Menu>("menu-exe-list").unwrap();
        self.game
            .as_ref()
            .borrow_mut()
            .add_exes_to_menu(&menu_exe_list);
        debug!("{:?}", &menu_exe_list);
        self.game.as_ref().borrow_mut().set_menu_button(
            &self
                .builder
                .get_object::<MenuToolButton>("menu-sel-exe")
                .unwrap(),
        );
        self.game.as_ref().borrow().update_active_exe_ui();
        self.game
            .as_ref()
            .borrow()
            .add_categories_to_view(&category_list);
        // DEBUG
        vfs::generate_plugins_txt(&self.game.as_ref().borrow());
    }
}
