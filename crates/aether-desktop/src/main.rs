#![windows_subsystem = "windows"]

use aether_audio::HeadlessAudioEngine;
use aether_ui::JustMusicApp;
use eframe::NativeOptions;
use egui::{IconData, ViewportBuilder};
use std::path::PathBuf;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    // Initialize Headless Audio Engine (Headless backend thread)
    let audio_handle = HeadlessAudioEngine::spawn().ok();

    // Check CLI argument for opening a track on startup (e.g. Open With from Explorer)
    let mut initial_track = None;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let file_arg = args[1..].join(" ");
        let raw_path = file_arg.trim().trim_matches('"').trim_matches('\'');
        let file_path = PathBuf::from(raw_path);
        if file_path.exists() {
            initial_track = Some(file_path);
        }
    }

    // Window Icon
    let icon_data = load_app_icon();

    let mut viewport = ViewportBuilder::default()
        .with_title("Just Music")
        .with_decorations(false)
        .with_inner_size([920.0, 620.0])
        .with_min_inner_size([800.0, 520.0])
        .with_transparent(true);

    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(icon);
    }

    let native_options = NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Just Music",
        native_options,
        Box::new(move |cc| {
            let mut app = JustMusicApp::new(cc, audio_handle);
            if let Some(path) = initial_track {
                aether_ui::views::home_view::load_file(
                    path,
                    &mut app.state,
                    app.audio_handle.as_ref(),
                );
            }
            Ok(Box::new(app))
        }),
    )
}

fn load_app_icon() -> Option<Arc<IconData>> {
    // Attempt to load embedded icon from assets
    const ICON_BYTES: &[u8] = include_bytes!("../../../assets/icon.ico");
    if let Ok(image) = image::load_from_memory(ICON_BYTES) {
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        return Some(Arc::new(IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        }));
    }
    None
}
