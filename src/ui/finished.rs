use eframe::egui;
use crate::i18n::I18n;

pub fn show(ctx : &egui::Context, i18n : &I18n) -> bool {
  let mut close = false;

  egui::CentralPanel::default().show(
      ctx, | ui | {
        ui.vertical_centered(| ui | {
          ui.heading(i18n.t("finished_title"));

          if ui
            .button(i18n.t("close")).clicked() { close = true; }
        });
      });

  close
}
