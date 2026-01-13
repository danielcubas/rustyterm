use gtk4::prelude::*;
use gtk4::{gio, glib, Application, CssProvider};
use gtk4::gdk::Display;

use crate::window::RustyTermWindow;

const APP_ID: &str = "com.github.rustyterm";

pub struct RustyTermApp {
    app: Application,
}

impl RustyTermApp {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id(APP_ID)
            .flags(gio::ApplicationFlags::empty())
            .build();

        app.connect_startup(Self::on_startup);
        app.connect_activate(Self::on_activate);

        Self { app }
    }

    fn on_startup(_app: &Application) {
        // Load custom CSS to fix window control button hit areas
        let provider = CssProvider::new();
        provider.load_from_data(
            r#"
            /* Reduce window control buttons padding/margins */
            windowcontrols button {
                min-width: 24px;
                min-height: 24px;
                padding: 0px;
                margin: 0;
            }
            windowcontrols {
                margin: 0px;
                padding: 0px;
            }

            /* Add tab button styling */
            .add-tab-button {
                min-width: 24px;
                min-height: 24px;
                padding: 4px;
                margin: -4px;
                border-radius: 4px;
                transition: background-color 150ms ease-in-out;
            }
            .add-tab-button:hover {
                background-color: rgba(255, 255, 255, 0.12);
            }
            .add-tab-button:active {
                background-color: rgba(255, 255, 255, 0.2);
            }
            "#,
        );

        if let Some(display) = Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn on_activate(app: &Application) {
        let window = RustyTermWindow::new(app);
        window.present();
    }

    pub fn run(&self) -> glib::ExitCode {
        self.app.run()
    }
}
