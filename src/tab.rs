use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, PopoverMenu, GestureClick};
use gtk4::gio::Menu;
use gtk4::gdk::Rectangle;
use vte4::TerminalExt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::config::Config;
use crate::terminal::TerminalWidget;

pub struct Tab {
    pub container: Box,
    pub terminal: TerminalWidget,
    pub label_box: Box,
    pub title_label: Label,
}

impl Tab {
    pub fn new(config: &Config) -> Rc<RefCell<Self>> {
        let terminal = TerminalWidget::new(config);

        // Container for the terminal
        let container = Box::new(Orientation::Vertical, 0);
        container.append(terminal.widget());
        terminal.widget().set_hexpand(true);
        terminal.widget().set_vexpand(true);

        // Tab label with close button
        let label_box = Box::new(Orientation::Horizontal, 4);
        let title_label = Label::new(Some("Terminal"));
        let close_button = Button::from_icon_name("window-close-symbolic");
        close_button.set_has_frame(false);
        close_button.add_css_class("flat");
        close_button.add_css_class("circular");

        label_box.append(&title_label);
        label_box.append(&close_button);

        let tab = Rc::new(RefCell::new(Self {
            container,
            terminal,
            label_box,
            title_label,
        }));

        // Setup right-click context menu
        Self::setup_context_menu(&tab);

        // Update title when window title changes
        let tab_weak = Rc::downgrade(&tab);
        tab.borrow().terminal.widget().connect_window_title_notify(move |term| {
            if let Some(tab) = tab_weak.upgrade() {
                let title = term.window_title()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "Terminal".to_string());
                let short_title = if title.len() > 20 {
                    format!("{}...", &title[..17])
                } else {
                    title
                };
                tab.borrow().title_label.set_text(&short_title);
            }
        });

        tab
    }

    pub fn set_close_callback<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        if let Some(close_btn) = self.label_box.last_child() {
            if let Some(button) = close_btn.downcast_ref::<Button>() {
                button.connect_clicked(move |_| callback());
            }
        }
    }

    fn setup_context_menu(tab: &Rc<RefCell<Self>>) {
        let menu = Menu::new();
        menu.append(Some("Copy"), Some("win.copy"));
        menu.append(Some("Paste"), Some("win.paste"));

        let popover = PopoverMenu::from_model(Some(&menu));
        popover.set_parent(tab.borrow().terminal.widget());
        popover.set_has_arrow(false);

        let gesture = GestureClick::new();
        gesture.set_button(3); // Right mouse button

        let popover_clone = popover.clone();
        gesture.connect_pressed(move |gesture, _, x, y| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            popover_clone.set_pointing_to(Some(&Rectangle::new(x as i32, y as i32, 1, 1)));
            popover_clone.popup();
        });

        tab.borrow().terminal.widget().add_controller(gesture);
    }
}
