use eframe::egui;

use crate::i18n::I18n;

pub fn show(ctx: &egui::Context, i18n: &I18n, progress: f32, status: &str) -> bool {
    let mut cancel = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(i18n.t("downloading"));
        ui.label(status);

        ui.add(
            egui::ProgressBar::new(progress)
                .show_percentage()
                .animate(true),
        );
    });

    egui::TopBottomPanel::bottom("downloading_buttons")
        .exact_height(50.0)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(i18n.t("cancel")).clicked() {
                    cancel = true;
                }
            });
        });

    cancel
}
