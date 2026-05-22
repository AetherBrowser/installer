use eframe::egui;

use crate::i18n::I18n;
use crate::install_options::{label_key, InstallOptionItem};
use crate::release::ReleaseRecap;

pub fn show(
    ctx: &egui::Context,
    i18n: &I18n,
    recap: &ReleaseRecap,
    options: &[InstallOptionItem],
) -> (bool, bool) {
    let mut continue_clicked = false;
    let mut back = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(i18n.t("recap_title"));
        ui.add_space(8.0);
        ui.label(i18n.t("recap_subtitle"));
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        egui::Frame::none()
            .outer_margin(egui::Margin::symmetric(40.0, 0.0))
            .show(ui, |ui| {
                ui.label(format!("{} {}", i18n.t("recap_app"), recap.app));
                ui.add_space(8.0);
                ui.label(format!("{} {}", i18n.t("recap_version"), recap.version));
                ui.label(format!("{} {}", i18n.t("recap_released"), recap.released_at));
                if !recap.description.is_empty() {
                    ui.add_space(4.0);
                    ui.label(format!("{} {}", i18n.t("recap_description"), recap.description));
                }
                ui.add_space(8.0);
                ui.label(format!(
                    "{} {}",
                    i18n.t("recap_install_dir"),
                    recap.install_dir.display()
                ));

                let enabled: Vec<_> = options
                    .iter()
                    .filter(|o| o.enabled)
                    .map(|o| i18n.t(label_key(o.kind)))
                    .collect();

                if !enabled.is_empty() {
                    ui.add_space(12.0);
                    ui.label(i18n.t("recap_options"));
                    for label in enabled {
                        ui.label(format!("  • {label}"));
                    }
                }
            });
    });

    egui::TopBottomPanel::bottom("recap_buttons")
        .exact_height(50.0)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(i18n.t("continue")).clicked() {
                    continue_clicked = true;
                }
                if ui.button(i18n.t("back")).clicked() {
                    back = true;
                }
            });
        });

    (continue_clicked, back)
}
