use eframe::egui;
use crate::i18n::I18n;

pub fn show(ctx : &egui::Context, i18n : &I18n) {
  egui::CentralPanel::default().show(
      ctx, | ui | {
        ui.vertical_centered(| ui | {
          ui.heading(i18n.t("app_title"));
          ui.add_space(10.0);
          ui.add(egui::Spinner::new ());
        });
      });
}
