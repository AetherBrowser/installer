use eframe::egui;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;

use crate::config_download::fetch_config;
use crate::download::download_and_install;
use crate::install_options::{self, InstallOptionItem};
use crate::i18n::I18n;
use crate::release::{self, ReleaseRecap};
use crate::ui;

pub enum Step {
    Loading,
    Welcome,
    License,
    FetchingConfig,
    Options,
    Recap,
    Downloading,
    Finished,
    Failed,
}

pub struct InstallerApp {
    pub step: Step,
    pub progress: Arc<Mutex<f32>>,
    pub status: Arc<Mutex<String>>,
    pub download_error: Arc<Mutex<Option<String>>>,
    pub download_cancelled: Arc<AtomicBool>,
    pub config_error: Arc<Mutex<Option<String>>>,
    pub pending_recap: Arc<Mutex<Option<ReleaseRecap>>>,
    pub release_recap: Option<ReleaseRecap>,
    pub error_message: Option<String>,
    pub install_options: Vec<InstallOptionItem>,
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
            download_error: Arc::new(Mutex::new(None)),
            download_cancelled: Arc::new(AtomicBool::new(false)),
            config_error: Arc::new(Mutex::new(None)),
            pending_recap: Arc::new(Mutex::new(None)),
            release_recap: None,
            error_message: None,
            install_options: install_options::default_options(),
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
                ui::loading::show(ctx, &self.i18n, None);
                self.step = Step::Welcome;
            }

            Step::Welcome => {
                let (continue_clicked, cancel) = ui::welcome::show(ctx, &self.i18n);

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
                    self.start_fetch_config(ctx.clone());
                    self.step = Step::FetchingConfig;
                } else if decline {
                    self.step = Step::Failed;
                }
            }

            Step::FetchingConfig => {
                ui::loading::show(ctx, &self.i18n, Some(self.i18n.t("fetching_config")));

                if let Some(err) = self.config_error.lock().unwrap().take() {
                    self.error_message = Some(err);
                    self.step = Step::Failed;
                } else if let Some(recap) = self.pending_recap.lock().unwrap().take() {
                    self.release_recap = Some(recap);
                    self.step = Step::Options;
                }
            }

            Step::Options => {
                if ui::options::show(ctx, &self.i18n, &mut self.install_options) {
                    self.step = Step::Recap;
                }
            }

            Step::Recap => {
                if self.release_recap.is_none() {
                    self.error_message = Some("Missing release info".into());
                    self.step = Step::Failed;
                } else {
                let recap = self.release_recap.as_ref().unwrap();
                let (continue_clicked, back) =
                    ui::recap::show(ctx, &self.i18n, recap, &self.install_options);

                if continue_clicked {
                    self.reset_download_progress();
                    self.start_download_and_install(ctx.clone());
                    self.step = Step::Downloading;
                } else if back {
                    self.step = Step::Options;
                }
                }
            }

            Step::Downloading => {
                let progress = *self.progress.lock().unwrap();
                let status = self.status.lock().unwrap().clone();

                if ui::downloading::show(ctx, &self.i18n, progress, &status) {
                    self.download_cancelled.store(true, Ordering::Relaxed);
                    self.error_message = Some(self.i18n.t("err_cancelled").to_string());
                    self.step = Step::Failed;
                } else if let Some(err) = self.download_error.lock().unwrap().take() {
                    self.error_message = Some(err);
                    self.step = Step::Failed;
                } else if progress >= 1.0 {
                    match install_options::apply(&self.install_options) {
                        Ok(()) => self.step = Step::Finished,
                        Err(e) => {
                            self.error_message = Some(e);
                            self.step = Step::Failed;
                        }
                    }
                }
            }

            Step::Finished => {
                match ui::finished::show(ctx, &self.i18n) {
                    ui::finished::FinishedAction::Close => std::process::exit(0),
                    ui::finished::FinishedAction::CloseAndLaunch => {
                        if let Some(recap) = &self.release_recap {
                            let _ = release::launch_app(&recap.install_dir);
                        }
                        std::process::exit(0);
                    }
                    ui::finished::FinishedAction::None => {}
                }
            }

            Step::Failed => {
                if ui::failed::show(ctx, &self.i18n, self.error_message.as_deref()) {
                    std::process::exit(0);
                }
            }
        }
    }
}

impl InstallerApp {
    fn reset_download_progress(&self) {
        *self.progress.lock().unwrap() = 0.0;
        *self.status.lock().unwrap() = self.i18n.t("downloading").into();
        *self.download_error.lock().unwrap() = None;
        self.download_cancelled.store(false, Ordering::Relaxed);
    }

    pub fn start_fetch_config(&mut self, ctx: egui::Context) {
        *self.config_error.lock().unwrap() = None;
        *self.pending_recap.lock().unwrap() = None;

        let config_error = self.config_error.clone();
        let pending_recap = self.pending_recap.clone();

        thread::spawn(move || {
            match fetch_config().and_then(|config| release::from_config(&config)) {
                Ok(recap) => *pending_recap.lock().unwrap() = Some(recap),
                Err(e) => *config_error.lock().unwrap() = Some(e),
            }

            ctx.request_repaint();
        });
    }

    pub fn start_download_and_install(&self, ctx: egui::Context) {
        let Some(recap) = self.release_recap.clone() else {
            *self.download_error.lock().unwrap() =
                Some("Missing release info".into());
            ctx.request_repaint();
            return;
        };

        let progress = self.progress.clone();
        let status = self.status.clone();
        let download_error = self.download_error.clone();
        let cancelled = self.download_cancelled.clone();

        let str_trying = self.i18n.t("trying").to_string();
        let str_verifying = self.i18n.t("verifying").to_string();
        let str_installing = self.i18n.t("installing").to_string();
        let err_cancelled = self.i18n.t("err_cancelled").to_string();

        thread::spawn(move || {
            let result = download_and_install(
                &recap,
                progress.clone(),
                status.clone(),
                cancelled,
                &str_trying,
                &str_verifying,
                &str_installing,
                &err_cancelled,
            );

            match result {
                Ok(()) => {
                    *status.lock().unwrap() = String::new();
                    *progress.lock().unwrap() = 1.0;
                }
                Err(e) => {
                    *download_error.lock().unwrap() = Some(e);
                }
            }

            ctx.request_repaint();
        });
    }
}
