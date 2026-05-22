use eframe::egui;

use crate::i18n::I18n;

const FOOTER_HEIGHT: f32 = 50.0;

pub fn show(
    ctx: &egui::Context,
    i18n: &I18n,
    license_text: &str,
    license_read: &mut bool,
) -> (bool, bool) {
    let mut accept = false;
    let mut decline = false;

    egui::TopBottomPanel::bottom("license_buttons")
        .exact_height(FOOTER_HEIGHT)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_enabled_ui(*license_read, |ui| {
                    if ui.button(i18n.t("accept")).clicked() {
                        accept = true;
                    }
                });

                if ui.button(i18n.t("decline")).clicked() {
                    decline = true;
                }
            });
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(i18n.t("license_title"));
        ui.label(i18n.t("license_subtitle"));
        ui.add_space(15.0);
        ui.separator();
        ui.add_space(15.0);

        egui::Frame::none()
            .outer_margin(egui::Margin::symmetric(40.0, 0.0))
            .show(ui, |ui| {
                ui.label(i18n.t("license_body"));
            });

        ui.add_space(15.0);

        let scroll_height = ui.available_height().max(0.0);

        egui::Frame::none()
            .fill(ui.visuals().extreme_bg_color)
            .outer_margin(egui::Margin::symmetric(40.0, 0.0))
            .show(ui, |ui| {
                ui.set_min_height(scroll_height);
                ui.set_max_height(scroll_height);

                let scroll_output = egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.label(license_text);
                    });

                let visible_bottom =
                    scroll_output.state.offset.y + scroll_output.inner_rect.height();
                let content_height = scroll_output.content_size.y;

                if content_height > 0.0 && visible_bottom >= content_height - 2.0 {
                    *license_read = true;
                }
            });
    });

    (accept, decline)
}
