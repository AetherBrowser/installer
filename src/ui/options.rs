use eframe::egui;

use crate::i18n::I18n;
use crate::install_options::{label_key, InstallOptionItem};

pub fn show(
    ctx: &egui::Context,
    i18n: &I18n,
    options: &mut [InstallOptionItem],
) -> bool {
    let mut continue_clicked = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(i18n.t("options_title"));
        ui.add_space(8.0);
        ui.label(i18n.t("options_subtitle"));
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        egui::Frame::none()
            .outer_margin(egui::Margin::symmetric(40.0, 0.0))
            .show(ui, |ui| {
                ui.label(i18n.t("options_explain"));
                ui.add_space(16.0);

                for item in options.iter_mut() {
                    let label = i18n.t(label_key(item.kind));
                    ui.checkbox(&mut item.enabled, label);
                }
            });
    });

    egui::TopBottomPanel::bottom("options_buttons")
        .exact_height(50.0)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(i18n.t("continue")).clicked() {
                    continue_clicked = true;
                }
            });
        });

    continue_clicked
}
