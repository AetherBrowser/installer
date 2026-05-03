use eframe::egui;
use crate::i18n::I18n;

pub fn show(ctx : &egui::Context, i18n : &I18n, progress : f32, status : &str) {
  egui::CentralPanel::default().show(
      ctx, | ui | {
        ui.heading(i18n.t("downloading"));
        ui.label(status);

        ui.add(egui::ProgressBar::new (progress).show_percentage().animate(
                   true), );
      });
}
