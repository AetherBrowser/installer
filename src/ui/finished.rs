use eframe::egui;

use crate::i18n::I18n;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FinishedAction {
    None,
    Close,
    CloseAndLaunch,
}

pub fn show(ctx: &egui::Context, i18n: &I18n) -> FinishedAction {
    let mut action = FinishedAction::None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(70.0);
            ui.heading(i18n.t("finished_title"));
        });
    });

    egui::TopBottomPanel::bottom("finished_buttons")
        .exact_height(50.0)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(i18n.t("close_and_launch")).clicked() {
                    action = FinishedAction::CloseAndLaunch;
                }
                if ui.button(i18n.t("close")).clicked() {
                    action = FinishedAction::Close;
                }
            });
        });

    action
}
