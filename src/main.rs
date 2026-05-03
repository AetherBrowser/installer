mod app;
mod download;
mod i18n;
mod ui;

use app::InstallerApp;

fn main() {
    let app = InstallerApp::default();
    let title = app.i18n.t("app_title").to_string();

    let options = eframe::NativeOptions::default();

    if let Err(e) = eframe::run_native(
        &title,
        options,
        Box::new(|_cc| Box::new(app)),
    ) {
        eprintln!("Error: {}", e);
    }
}
