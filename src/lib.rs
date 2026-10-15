use nexus::{
    data_link::read_nexus_link,
    gamebind::{invoke_gamebind_async, GameBind},
    gui::{register_render, render, RenderType},
    imgui::Ui,
    paths::get_addon_dir,
    AddonFlags, UpdateProvider,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{LazyLock, Mutex},
    time::{Duration, Instant},
};

const ADDON_DIR_NAME: &str = "zoom-me-out";
const MIN_INTERVAL_MS: i32 = 50;
const MAX_INTERVAL_MS: i32 = 1000;
const PRESS_DURATION_MS: i32 = 10;
const CAMERA_ZOOM_OUT_BUTTON: GameBind = GameBind::CameraZoomOut;
const AFK_TIMEOUT: Duration = Duration::from_secs(30);

nexus::export! {
    name: "Zoom Me Out",
    signature: -0x5A11_2391,
    load,
    flags: AddonFlags::None,
    provider: UpdateProvider::GitHub
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    enabled: bool,
    interval_ms: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_ms: 100,
        }
    }
}

static SETTINGS: LazyLock<Mutex<Settings>> = LazyLock::new(|| Mutex::new(Settings::default()));
static LAST_PRESS: Mutex<Option<Instant>> = Mutex::new(None);
static LAST_MOVED: Mutex<Option<Instant>> = Mutex::new(None);

fn config_path() -> Option<PathBuf> {
    let dir = get_addon_dir(ADDON_DIR_NAME).ok()?;
    fs::create_dir_all(&dir).ok()?;
    Some(dir.join("config.json"))
}

fn load_settings() -> Settings {
    let Some(path) = config_path() else {
        return Settings::default();
    };
    let mut settings: Settings = fs::read_to_string(&path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();
    settings.interval_ms = settings.interval_ms.clamp(MIN_INTERVAL_MS, MAX_INTERVAL_MS);
    settings
}

fn save_settings(settings: &Settings) {
    let Some(path) = config_path() else {
        log::error!("could not resolve config path");
        return;
    };
    match serde_json::to_string_pretty(settings) {
        Ok(text) => {
            if let Err(err) = fs::write(&path, text) {
                log::error!("failed to write config: {err}");
            }
        }
        Err(err) => log::error!("failed to serialize config: {err}"),
    }
}

/// Shown inside the Nexus addon options window.
fn options_ui(ui: &Ui) {
    let mut settings = SETTINGS.lock().unwrap();
    let mut changed = false;

    changed |= ui.checkbox("Enabled", &mut settings.enabled);

    ui.input_int("Interval (ms)", &mut settings.interval_ms)
        .step(10)
        .build();
    if ui.is_item_deactivated_after_edit() {
        settings.interval_ms = settings.interval_ms.clamp(MIN_INTERVAL_MS, MAX_INTERVAL_MS);
        changed = true;
    }
    ui.text_disabled(format!("{MIN_INTERVAL_MS} - {MAX_INTERVAL_MS} ms"));
    ui.text_disabled(format!(
        "Pauses after {} s without movement.",
        AFK_TIMEOUT.as_secs()
    ));

    if changed {
        save_settings(&settings);
    }
}

/// Runs every frame, presses the button when the interval has elapsed.
fn tick(_ui: &Ui) {
    let settings = SETTINGS.lock().unwrap();
    let mut moved = LAST_MOVED.lock().unwrap();

    if !settings.enabled {
        *moved = None; // enabling again starts a fresh AFK window
        return;
    }

    let now = Instant::now();

    // AFK check: remember the last time the player was moving
    if read_nexus_link().is_some_and(|link| link.is_moving) {
        *moved = Some(now);
    }
    let last_moved = *moved.get_or_insert(now);
    if now.duration_since(last_moved) >= AFK_TIMEOUT {
        return; // AFK: do nothing until the player moves again
    }

    let interval = Duration::from_millis(
        settings.interval_ms.clamp(MIN_INTERVAL_MS, MAX_INTERVAL_MS) as u64,
    );
    let mut last = LAST_PRESS.lock().unwrap();
    if last.map_or(true, |t| now.duration_since(t) >= interval) {
        invoke_gamebind_async(CAMERA_ZOOM_OUT_BUTTON, PRESS_DURATION_MS);
        *last = Some(now);
    }
}

fn load() {
    *SETTINGS.lock().unwrap() = load_settings();

    register_render(RenderType::Render, render!(tick)).revert_on_unload();
    register_render(RenderType::OptionsRender, render!(options_ui)).revert_on_unload();

    log::info!("Zoom Me Out loaded");
}