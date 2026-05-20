use eframe::egui;
use crate::i18n::I18n;

pub fn show(ctx: &egui::Context, i18n: &I18n) -> (bool, bool) {
    let mut continue_clicked = false;
    let mut cancel = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(70.0);

            ui.heading(i18n.t("welcome_title"));
            ui.add_space(20.0);

            ui.label(i18n.t("welcome_message"));
            ui.add_space(20.0);

            ui.label(i18n.t("welcome_explain"));
        });
    });

egui::TopBottomPanel::bottom("welcome_buttons")
    .exact_height(50.0)
    .show(ctx, |ui| {
        ui.with_layout(
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                if ui.button(i18n.t("cancel")).clicked() {
                    cancel = true;
                }

                if ui.button(i18n.t("continue")).clicked() {
                    continue_clicked = true;
                }
            },
        );
    });
    (continue_clicked, cancel)
}
