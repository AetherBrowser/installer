use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::download::download_latest;
use crate::i18n::I18n;
use crate::ui;

pub enum Step {
    Loading,
    Welcome,
    License,
    Downloading,
    Finished,
    Failed,
}

pub struct InstallerApp {
    pub step: Step,
    pub progress: Arc<Mutex<f32>>,
    pub status: Arc<Mutex<String>>,
    pub license_text: String,
    pub license_read: bool,
    pub i18n: I18n,
    pub dark_mode: bool,
}

impl Default for InstallerApp {
    fn default() -> Self {
        let lang = sys_locale::get_locale().unwrap_or_else(|| "en".into());
        let lang = lang.split('-').next().unwrap_or("en");
        let i18n = I18n::new(lang);

        let license_path = format!("assets/license_{}.txt", lang);
        let license_text_lang = std::fs::read_to_string(&license_path).unwrap_or_else(|_| {
            std::fs::read_to_string("assets/license_en.txt")
                .unwrap_or_else(|_| "License missing".into())
        });

        let license_extra = std::fs::read_to_string("assets/LICENSE").unwrap_or_default();

        let license_text = if license_extra.is_empty() {
            license_text_lang
        } else {
            format!(
                "{}\n\n====================================\n\n{}",
                license_text_lang, license_extra
            )
        };

        let dark_mode = match dark_light::detect() {
            dark_light::Mode::Light => false,
            dark_light::Mode::Dark => true,
        };

        Self {
            step: Step::Loading,
            progress: Arc::new(Mutex::new(0.0)),
            status: Arc::new(Mutex::new(i18n.t("loading").into())),
            license_text,
            license_read: false,
            i18n,
            dark_mode,
        }
    }
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        match self.step {
            Step::Loading => {
                ui::loading::show(ctx, &self.i18n);
                self.step = Step::Welcome;
            }

            Step::Welcome => {
                let (continue_clicked, cancel) =
                    ui::welcome::show(ctx, &self.i18n);

                if continue_clicked {
                    self.step = Step::License;
                } else if cancel {
                    self.step = Step::Failed;
                }
            }

            Step::License => {
                let (accept, decline) = ui::license::show(
                    ctx,
                    &self.i18n,
                    &self.license_text,
                    &mut self.license_read,
                );

                if accept {
                    self.start_download(ctx.clone());
                    self.step = Step::Downloading;
                } else if decline {
                    self.step = Step::Failed;
                }
            }

            Step::Downloading => {
                let progress = *self.progress.lock().unwrap();
                let status = self.status.lock().unwrap().clone();

                ui::downloading::show(ctx, &self.i18n, progress, &status);

                if progress >= 1.0 {
                    self.step = Step::Finished;
                }
            }

            Step::Finished => {
                if ui::finished::show(ctx, &self.i18n) {
                    std::process::exit(0);
                }
            }

            Step::Failed => {
                if ui::failed::show(ctx, &self.i18n) {
                    std::process::exit(0);
                }
            }
        }
    }
}

impl InstallerApp {
    pub fn start_download(&self, ctx: egui::Context) {
        let progress = self.progress.clone();
        let status = self.status.clone();

        let str_trying    = self.i18n.t("trying").to_string();
        let str_verifying = self.i18n.t("verifying").to_string();
        let str_ok        = self.i18n.t("download_ok").to_string();

        thread::spawn(move || {
            let result = download_latest(
                progress.clone(),
                status.clone(),
                &str_trying,
                &str_verifying,
            );

            let mut status_lock = status.lock().unwrap();

            match result {
                Ok(_) => {
                    *status_lock = str_ok;
                    *progress.lock().unwrap() = 1.0;
                }
                Err(e) => {
                    *status_lock = format!("Error: {}", e);
                }
            }

            ctx.request_repaint();
        });
    }
}
