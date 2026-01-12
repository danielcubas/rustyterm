use gtk4::prelude::*;
use vte4::{Terminal, TerminalExt, TerminalExtManual};

use crate::config::Config;
use crate::theme::{get_theme_by_name, Theme};

pub struct TerminalWidget {
    terminal: Terminal,
}

impl TerminalWidget {
    pub fn new(config: &Config) -> Self {
        let terminal = Terminal::new();

        let widget = Self { terminal };
        widget.apply_config(config);
        widget.spawn_shell();

        widget
    }

    pub fn widget(&self) -> &Terminal {
        &self.terminal
    }

    pub fn apply_config(&self, config: &Config) {
        // Set font
        let font_desc = format!("{} {}", config.font_family, config.font_size);
        self.terminal.set_font_desc(Some(
            &gtk4::pango::FontDescription::from_string(&font_desc),
        ));

        // Set scrollback
        self.terminal.set_scrollback_lines(config.scrollback_lines);

        // Apply theme
        self.apply_theme(&get_theme_by_name(&config.theme));
    }

    pub fn apply_theme(&self, theme: &Theme) {
        let bg = theme.background_rgba();
        let fg = theme.foreground_rgba();
        let cursor = theme.cursor_rgba();
        let palette: Vec<_> = theme.palette_rgba();

        self.terminal.set_colors(
            Some(&fg),
            Some(&bg),
            &palette.iter().collect::<Vec<_>>(),
        );
        self.terminal.set_color_cursor(Some(&cursor));
        self.terminal.set_color_cursor_foreground(Some(&bg));
    }

    fn spawn_shell(&self) {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

        let pty_flags = vte4::PtyFlags::DEFAULT;
        let spawn_flags = glib::SpawnFlags::DEFAULT;

        let args: &[&str] = &[&shell];
        let envv: &[&str] = &[];

        self.terminal.spawn_async(
            pty_flags,
            None,  // working directory (None = current)
            args,
            envv,
            spawn_flags,
            || {},
            -1,    // timeout (-1 = default)
            gtk4::gio::Cancellable::NONE,
            |_result| {},
        );
    }

    pub fn get_current_directory(&self) -> Option<String> {
        self.terminal.current_directory_uri()
            .map(|uri| uri.to_string())
    }
}
