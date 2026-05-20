mod app;
mod config_download;
mod download;
mod i18n;
mod ui;
mod helper;

use app::InstallerApp;

fn main() {
    let app = InstallerApp::default();
    let title = app.i18n.t("app_title").to_string();

    let icon = load_icon("assets/icon.png");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(icon),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        &title,
        options,
        Box::new(|_cc| Box::new(app)),
    ) {
        eprintln!("Error: {}", e);
    }
}

fn load_icon(path: &str) -> egui::IconData {
    let image = image::open(path)
        .expect("Failed to open icon path")
        .into_rgba8();

    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}
