use eframe::egui;

use crate::i18n::I18n;

pub fn show(ctx: &egui::Context, i18n: &I18n, error: Option<&str>) -> bool {
    let mut close = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(70.0);

            ui.heading(i18n.t("failed_title"));

            if let Some(message) = error {
                ui.add_space(20.0);
                ui.label(message);
            }
        });
    });

    egui::TopBottomPanel::bottom("failed_buttons")
        .exact_height(50.0)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(i18n.t("close")).clicked() {
                    close = true;
                }
            });
        });

    close
}
