use crate::state::AppState;
use aether_audio::AudioEngineHandle;
use aether_core::PlayerCommand;
use egui::{Align, Color32, Layout, RichText, Sense, Vec2};
use std::path::PathBuf;

pub fn render_home_view(
    ui: &mut egui::Ui,
    state: &mut AppState,
    audio_handle: Option<&AudioEngineHandle>,
) {
    let palette = state.design_system.palette.clone();
    let is_rtl = state.settings.is_rtl;

    ui.add_space(20.0);

    // Main Drag & Drop / File Upload Card
    let card_height = 220.0;
    let (rect, _response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), card_height), Sense::hover());

    ui.painter().rect_filled(rect, 16.0, palette.cards);
    ui.painter()
        .rect_stroke(rect, 16.0, egui::Stroke::new(1.5_f32, palette.borders));

    // Contents inside card
    ui.allocate_ui_at_rect(rect, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(35.0);

            // Icon
            ui.label(RichText::new("📁").size(44.0));
            ui.add_space(10.0);

            // Main Label
            let drag_text = if is_rtl {
                "גרור קבצי מוזיקה לכאן"
            } else {
                "Drag Music Files Here"
            };
            ui.label(
                RichText::new(drag_text)
                    .size(18.0)
                    .strong()
                    .color(palette.text_primary),
            );

            ui.add_space(4.0);
            let or_text = if is_rtl { "או" } else { "or" };
            ui.label(
                RichText::new(or_text)
                    .size(13.0)
                    .color(palette.text_secondary),
            );
            ui.add_space(10.0);

            // Open File Button
            let btn_text = if is_rtl {
                "📂 פתח קובץ שמע"
            } else {
                "📂 Open File"
            };
            let btn = egui::Button::new(
                RichText::new(btn_text)
                    .size(14.0)
                    .strong()
                    .color(Color32::WHITE),
            )
            .fill(palette.primary)
            .rounding(10.0)
            .min_size(Vec2::new(160.0, 38.0));

            if ui.add(btn).clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter(
                        "Audio Files",
                        &[
                            "mp3", "flac", "wav", "aac", "ogg", "opus", "m4a", "wma", "aiff",
                        ],
                    )
                    .pick_file()
                {
                    load_file(path, state, audio_handle);
                }
            }
        });
    });

    ui.add_space(25.0);

    // Current Playing / Loaded Track Details or Empty State
    if let Some(track) = &state.current_track {
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            let layout = if is_rtl {
                Layout::right_to_left(Align::Center)
            } else {
                Layout::left_to_right(Align::Center)
            };
            ui.with_layout(layout, |ui| {
                ui.label(RichText::new("🎵").size(24.0));
                ui.add_space(10.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(&track.title)
                            .size(16.0)
                            .strong()
                            .color(palette.text_primary),
                    );
                    ui.label(
                        RichText::new(&track.artist)
                            .size(13.0)
                            .color(palette.text_secondary),
                    );
                });
            });
        });
    } else {
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            let empty_title = if is_rtl {
                "עדיין לא נטען קובץ מוזיקה"
            } else {
                "No music loaded yet"
            };
            let empty_sub = if is_rtl {
                "גרור קבצים לכאן או לחץ על פתח קובץ להתחלת האזנה"
            } else {
                "Drag files here or click Open File to start listening"
            };
            ui.label(
                RichText::new(empty_title)
                    .size(16.0)
                    .strong()
                    .color(palette.text_secondary),
            );
            ui.label(
                RichText::new(empty_sub)
                    .size(13.0)
                    .color(palette.text_secondary),
            );
        });
    }
}

pub fn load_file(path: PathBuf, state: &mut AppState, audio_handle: Option<&AudioEngineHandle>) {
    let ext_str = match path.extension().and_then(|s| s.to_str()) {
        Some(s) => s.to_lowercase(),
        None => String::new(),
    };
    let format = aether_core::AudioFormat::from_extension(&ext_str);
    let title = match path.file_stem() {
        Some(s) => s.to_string_lossy().to_string(),
        None => "Audio File".to_string(),
    };
    let artist_str = if state.settings.is_rtl {
        "קובץ שמע מקומי".to_string()
    } else {
        "Local Audio File".to_string()
    };
    let track = aether_core::Track {
        id: aether_core::TrackId::new(),
        file_path: path.clone(),
        title,
        artist: artist_str,
        album: "Just Music".to_string(),
        genre: None,
        year: None,
        track_number: None,
        duration_ms: 0,
        bitrate: Some(320),
        sample_rate: 44100,
        channels: 2,
        format,
        replaygain_track_gain: None,
        replaygain_track_peak: None,
        play_count: 0,
        rating: 0,
    };

    state.current_track = Some(track.clone());
    if !state.playlist.iter().any(|t| t.file_path == path) {
        state.playlist.push(track.clone());
    }
    if !state.recent_tracks.iter().any(|t| t.file_path == path) {
        state.recent_tracks.insert(0, track);
    }

    if let Some(handle) = audio_handle {
        let _ = handle.send_command(PlayerCommand::LoadTrack(path));
    }
    let msg = if state.settings.is_rtl {
        "הקובץ נטען בהצלחה"
    } else {
        "File Loaded Successfully"
    };
    state.toast_manager.notify(msg);
}
