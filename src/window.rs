use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, HeaderBar, MenuButton,
    Notebook, Orientation,
};
use gtk4::gio::{Menu, MenuItem, SimpleAction};
use std::cell::RefCell;
use std::rc::Rc;

use crate::config::Config;
use crate::tab::Tab;
use crate::theme::{get_theme_by_name, get_themes};

pub struct RustyTermWindow {
    pub window: ApplicationWindow,
    notebook: Notebook,
    config: Rc<RefCell<Config>>,
    tabs: Rc<RefCell<Vec<Rc<RefCell<Tab>>>>>,
}

impl RustyTermWindow {
    pub fn new(app: &Application) -> Self {
        let config = Rc::new(RefCell::new(Config::load()));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("RustyTerm")
            .default_width(config.borrow().window_width)
            .default_height(config.borrow().window_height)
            .build();

        let notebook = Notebook::new();
        notebook.set_scrollable(true);
        notebook.set_show_border(false);
        notebook.popup_enable();

        // New tab button
        let new_tab_btn = Button::from_icon_name("list-add-symbolic");
        new_tab_btn.set_tooltip_text(Some("New Tab (Ctrl+Shift+T)"));
        notebook.set_action_widget(&new_tab_btn, gtk4::PackType::End);

        // Header bar with menu
        let header = HeaderBar::new();
        let menu_button = Self::create_menu_button();
        header.pack_end(&menu_button);
        window.set_titlebar(Some(&header));

        // Main container
        let main_box = Box::new(Orientation::Vertical, 0);
        main_box.append(&notebook);
        window.set_child(Some(&main_box));

        let tabs: Rc<RefCell<Vec<Rc<RefCell<Tab>>>>> = Rc::new(RefCell::new(Vec::new()));

        let win = Self {
            window,
            notebook,
            config,
            tabs,
        };

        // Add first tab
        win.add_tab();

        // Setup actions and signals
        win.setup_actions(app);
        win.setup_new_tab_button(&new_tab_btn);
        win.setup_notebook_signals();

        win
    }

    fn create_menu_button() -> MenuButton {
        let menu = Menu::new();

        // Theme submenu
        let theme_menu = Menu::new();
        for theme in get_themes() {
            let item = MenuItem::new(Some(&theme.name), Some(&format!("win.set-theme::{}", theme.name)));
            theme_menu.append_item(&item);
        }
        menu.append_submenu(Some("Themes"), &theme_menu);

        let menu_button = MenuButton::new();
        menu_button.set_icon_name("open-menu-symbolic");
        menu_button.set_menu_model(Some(&menu));

        menu_button
    }

    fn setup_actions(&self, app: &Application) {
        let config = self.config.clone();
        let notebook = self.notebook.clone();
        let tabs = self.tabs.clone();
        let window = self.window.clone();

        // New tab action
        let new_tab_action = SimpleAction::new("new-tab", None);
        let config_clone = config.clone();
        let notebook_clone = notebook.clone();
        let tabs_clone = tabs.clone();
        let window_clone = window.clone();
        new_tab_action.connect_activate(move |_, _| {
            Self::create_new_tab(&notebook_clone, &config_clone, &tabs_clone, &window_clone);
        });
        window.add_action(&new_tab_action);

        // Close tab action
        let close_tab_action = SimpleAction::new("close-tab", None);
        let notebook_clone = notebook.clone();
        let tabs_clone = tabs.clone();
        let window_clone = window.clone();
        close_tab_action.connect_activate(move |_, _| {
            let page = notebook_clone.current_page();
            if let Some(idx) = page {
                Self::close_tab_at(&notebook_clone, &tabs_clone, &window_clone, idx as usize);
            }
        });
        window.add_action(&close_tab_action);

        // Theme action
        let set_theme_action = SimpleAction::new("set-theme", Some(glib::VariantTy::STRING));
        let config_clone = config.clone();
        let tabs_clone = tabs.clone();
        set_theme_action.connect_activate(move |_, param| {
            if let Some(theme_name) = param.and_then(|p| p.str()) {
                let theme = get_theme_by_name(theme_name);
                for tab in tabs_clone.borrow().iter() {
                    tab.borrow().terminal.apply_theme(&theme);
                }
                config_clone.borrow_mut().theme = theme_name.to_string();
                let _ = config_clone.borrow().save();
            }
        });
        window.add_action(&set_theme_action);

        // Copy action
        let copy_action = SimpleAction::new("copy", None);
        let tabs_clone = tabs.clone();
        let notebook_clone = notebook.clone();
        copy_action.connect_activate(move |_, _| {
            if let Some(idx) = notebook_clone.current_page() {
                if let Some(tab) = tabs_clone.borrow().get(idx as usize) {
                    tab.borrow().terminal.copy_clipboard();
                }
            }
        });
        window.add_action(&copy_action);

        // Paste action
        let paste_action = SimpleAction::new("paste", None);
        let tabs_clone = tabs.clone();
        let notebook_clone = notebook.clone();
        paste_action.connect_activate(move |_, _| {
            if let Some(idx) = notebook_clone.current_page() {
                if let Some(tab) = tabs_clone.borrow().get(idx as usize) {
                    tab.borrow().terminal.paste_clipboard();
                }
            }
        });
        window.add_action(&paste_action);

        // Keyboard shortcuts
        app.set_accels_for_action("win.new-tab", &["<Ctrl><Shift>t"]);
        app.set_accels_for_action("win.close-tab", &["<Ctrl><Shift>w"]);
        app.set_accels_for_action("win.copy", &["<Ctrl><Shift>c"]);
        app.set_accels_for_action("win.paste", &["<Ctrl><Shift>v"]);
    }

    fn setup_new_tab_button(&self, button: &Button) {
        let config = self.config.clone();
        let notebook = self.notebook.clone();
        let tabs = self.tabs.clone();
        let window = self.window.clone();

        button.connect_clicked(move |_| {
            Self::create_new_tab(&notebook, &config, &tabs, &window);
        });
    }

    fn setup_notebook_signals(&self) {
        // Handle page switch for focus
        self.notebook.connect_switch_page(move |nb, _, page_num| {
            if let Some(page) = nb.nth_page(Some(page_num)) {
                page.grab_focus();
            }
        });
    }

    fn add_tab(&self) {
        Self::create_new_tab(&self.notebook, &self.config, &self.tabs, &self.window);
    }

    fn create_new_tab(
        notebook: &Notebook,
        config: &Rc<RefCell<Config>>,
        tabs: &Rc<RefCell<Vec<Rc<RefCell<Tab>>>>>,
        window: &ApplicationWindow,
    ) {
        let tab = Tab::new(&config.borrow());

        let page_num = notebook.append_page(
            &tab.borrow().container,
            Some(&tab.borrow().label_box),
        );

        notebook.set_tab_reorderable(&tab.borrow().container, true);
        notebook.set_current_page(Some(page_num));
        tab.borrow().terminal.widget().grab_focus();

        // Setup close callback
        let notebook_clone = notebook.clone();
        let tabs_clone = tabs.clone();
        let window_clone = window.clone();
        let tab_weak = Rc::downgrade(&tab);
        tab.borrow().set_close_callback(move || {
            if let Some(tab) = tab_weak.upgrade() {
                let idx = tabs_clone.borrow().iter()
                    .position(|t| Rc::ptr_eq(t, &tab));
                if let Some(idx) = idx {
                    Self::close_tab_at(&notebook_clone, &tabs_clone, &window_clone, idx);
                }
            }
        });

        tabs.borrow_mut().push(tab);
    }

    fn close_tab_at(
        notebook: &Notebook,
        tabs: &Rc<RefCell<Vec<Rc<RefCell<Tab>>>>>,
        window: &ApplicationWindow,
        idx: usize,
    ) {
        if tabs.borrow().len() <= 1 {
            window.close();
            return;
        }

        notebook.remove_page(Some(idx as u32));
        tabs.borrow_mut().remove(idx);
    }

    pub fn present(&self) {
        self.window.present();
    }
}
