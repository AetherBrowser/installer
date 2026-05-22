use eframe::egui;
use crate::i18n::I18n;

pub fn show(ctx: &egui::Context, i18n: &I18n, message: Option<&str>) {
  egui::CentralPanel::default().show(ctx, |ui| {
    ui.vertical_centered(|ui| {
      ui.add_space(70.0);
      ui.heading(message.unwrap_or(i18n.t("loading")));
      ui.add_space(10.0);
      ui.add(egui::Spinner::new());
    });
  });
}
