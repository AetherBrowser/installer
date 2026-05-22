use std::{fs, path::PathBuf};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InstallOptionKind {
    Taskbar,
    StartMenu,
    DesktopShortcut,
    ApplicationsMenu,
    DesktopEntry,
    Dock,
    Launchpad,
    MenuBarIcon,
}

#[derive(Clone)]
pub struct InstallOptionItem {
    pub kind: InstallOptionKind,
    pub enabled: bool,
}

pub fn detect_os() -> &'static str {
    std::env::consts::OS
}

pub fn label_key(kind: InstallOptionKind) -> &'static str {
    match kind {
        InstallOptionKind::Taskbar => "opt_taskbar",
        InstallOptionKind::StartMenu => "opt_start_menu",
        InstallOptionKind::DesktopShortcut => "opt_desktop_shortcut",
        InstallOptionKind::ApplicationsMenu => "opt_apps_menu",
        InstallOptionKind::DesktopEntry => "opt_desktop_entry",
        InstallOptionKind::Dock => "opt_dock",
        InstallOptionKind::Launchpad => "opt_launchpad",
        InstallOptionKind::MenuBarIcon => "opt_menu_bar",
    }
}

pub fn default_options() -> Vec<InstallOptionItem> {
    match detect_os() {
        "windows" => vec![
            (InstallOptionKind::Taskbar, true),
            (InstallOptionKind::StartMenu, true),
            (InstallOptionKind::DesktopShortcut, true),
        ],
        "linux" => vec![
            (InstallOptionKind::ApplicationsMenu, true),
            (InstallOptionKind::DesktopShortcut, true),
            (InstallOptionKind::DesktopEntry, true),
        ],
        "macos" => vec![
            (InstallOptionKind::Dock, true),
            (InstallOptionKind::Launchpad, true),
            (InstallOptionKind::MenuBarIcon, false),
        ],
        _ => vec![(InstallOptionKind::DesktopShortcut, true)],
    }
    .into_iter()
    .map(|(kind, enabled)| InstallOptionItem { kind, enabled })
    .collect()
}

fn install_dir() -> Result<PathBuf, String> {
    crate::release::default_install_dir()
}

pub fn apply(options: &[InstallOptionItem]) -> Result<(), String> {
    let install_dir = install_dir()?;

    match detect_os() {
        "linux" => apply_linux(options, &install_dir),
        "windows" => apply_windows(options, &install_dir),
        "macos" => apply_macos(options, &install_dir),
        _ => apply_generic(options, &install_dir),
    }
}

fn enabled(options: &[InstallOptionItem], kind: InstallOptionKind) -> bool {
    options
        .iter()
        .find(|o| o.kind == kind)
        .is_some_and(|o| o.enabled)
}

fn apply_linux(options: &[InstallOptionItem], install_dir: &PathBuf) -> Result<(), String> {
    let want_entry = enabled(options, InstallOptionKind::DesktopEntry)
        || enabled(options, InstallOptionKind::ApplicationsMenu);
    let want_desktop = enabled(options, InstallOptionKind::DesktopShortcut);

    if !want_entry && !want_desktop {
        return Ok(());
    }

    let exec = install_dir.join("app");
    let desktop_content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Aether\n\
         Exec={}\n\
         Path={}\n\
         Terminal=false\n\
         Categories=Utility;\n",
        exec.display(),
        install_dir.display()
    );

    if want_entry
        && let Some(apps_dir) = dirs::data_local_dir().map(|d| d.join("applications"))
    {
        fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;
        fs::write(apps_dir.join("aether.desktop"), &desktop_content)
            .map_err(|e| e.to_string())?;
    }

    if want_desktop && let Some(desktop_dir) = dirs::desktop_dir() {
        fs::write(desktop_dir.join("aether.desktop"), &desktop_content)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn apply_windows(options: &[InstallOptionItem], install_dir: &PathBuf) -> Result<(), String> {
    let _ = (
        enabled(options, InstallOptionKind::Taskbar),
        enabled(options, InstallOptionKind::StartMenu),
        enabled(options, InstallOptionKind::DesktopShortcut),
        install_dir,
    );
    Ok(())
}

fn apply_macos(options: &[InstallOptionItem], install_dir: &PathBuf) -> Result<(), String> {
    let _ = (
        enabled(options, InstallOptionKind::Dock),
        enabled(options, InstallOptionKind::Launchpad),
        enabled(options, InstallOptionKind::MenuBarIcon),
        install_dir,
    );
    Ok(())
}

fn apply_generic(options: &[InstallOptionItem], install_dir: &PathBuf) -> Result<(), String> {
    if enabled(options, InstallOptionKind::DesktopShortcut) {
        let _ = install_dir;
    }
    Ok(())
}
