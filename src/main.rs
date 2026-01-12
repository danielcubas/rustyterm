mod app;
mod config;
mod tab;
mod terminal;
mod theme;
mod window;

use app::RustyTermApp;

fn main() -> glib::ExitCode {
    let app = RustyTermApp::new();
    app.run()
}
