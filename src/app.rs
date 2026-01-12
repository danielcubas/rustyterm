use gtk4::prelude::*;
use gtk4::{gio, glib, Application};

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

        app.connect_activate(Self::on_activate);

        Self { app }
    }

    fn on_activate(app: &Application) {
        let window = RustyTermWindow::new(app);
        window.present();
    }

    pub fn run(&self) -> glib::ExitCode {
        self.app.run()
    }
}
