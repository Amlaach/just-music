use crate::state::AppState;
use aether_audio::AudioEngineHandle;
use aether_core::{AudioFormat, PlayerCommand};
use egui::{Color32, RichText, Vec2};

pub fn render_playlist_view(
    ui: &mut egui::Ui,
    state: &mut AppState,
    audio_handle: Option<&AudioEngineHandle>,
) {
    let palette = state.design_system.palette.clone();

    ui.horizontal(|ui| {
        ui.label(
            RichText::new("🎵 Playlist")
                .size(20.0)
                .strong()
                .color(palette.text_primary),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button(
                    RichText::new("+ Add Files")
                        .size(13.0)
                        .strong()
                        .color(Color32::WHITE),
                )
                .clicked()
            {
                if let Some(files) = rfd::FileDialog::new()
                    .add_filter(
                        "Audio Files",
                        &[
                            "mp3", "flac", "wav", "aac", "ogg", "opus", "m4a", "wma", "aiff",
                        ],
                    )
                    .pick_files()
                {
                    for path in files {
                        crate::views::home_view::load_file(path, state, audio_handle);
                    }
                }
            }

            if ui
                .button(
                    RichText::new("🗑 Clear All")
                        .size(13.0)
                        .color(palette.text_secondary),
                )
                .clicked()
            {
                state.playlist.clear();
                state.toast_manager.notify("Playlist cleared");
            }
        });
    });

    ui.add_space(15.0);

    if state.playlist.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(RichText::new("🎼").size(48.0));
            ui.add_space(10.0);
            ui.label(
                RichText::new("Playlist is empty")
                    .size(16.0)
                    .strong()
                    .color(palette.text_secondary),
            );
            ui.label(
                RichText::new("Add songs using the button above or drag files into Home")
                    .size(13.0)
                    .color(palette.text_secondary),
            );
        });
        return;
    }

    // Table Header
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.allocate_ui_with_layout(
            Vec2::new(300.0, 20.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label(RichText::new("Name").strong().color(palette.text_secondary));
            },
        );
        ui.allocate_ui_with_layout(
            Vec2::new(90.0, 20.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label(
                    RichText::new("Duration")
                        .strong()
                        .color(palette.text_secondary),
                );
            },
        );
        ui.allocate_ui_with_layout(
            Vec2::new(80.0, 20.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label(
                    RichText::new("Format")
                        .strong()
                        .color(palette.text_secondary),
                );
            },
        );
        ui.allocate_ui_with_layout(
            Vec2::new(90.0, 20.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label(RichText::new("Size").strong().color(palette.text_secondary));
            },
        );
    });

    ui.separator();

    // Track Items List
    let mut remove_index = None;
    let mut play_track = None;

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (idx, track) in state.playlist.iter().enumerate() {
            let is_current = state
                .current_track
                .as_ref()
                .map(|t| t.file_path == track.file_path)
                .unwrap_or(false);

            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 36.0), egui::Sense::click());

            let bg = if is_current {
                Color32::from_rgba_unmultiplied(217, 74, 74, 40)
            } else if response.hovered() {
                palette.borders
            } else {
                Color32::TRANSPARENT
            };

            ui.painter().rect_filled(rect, 8.0, bg);

            ui.allocate_ui_at_rect(rect, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    // Name
                    ui.allocate_ui_with_layout(
                        Vec2::new(300.0, 32.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            let mut text = RichText::new(&track.title).color(if is_current {
                                palette.primary
                            } else {
                                palette.text_primary
                            });
                            if is_current {
                                text = text.strong();
                            }
                            ui.label(text);
                        },
                    );

                    // Duration
                    ui.allocate_ui_with_layout(
                        Vec2::new(90.0, 32.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            let sec = track.duration_ms / 1000;
                            ui.label(
                                RichText::new(format!("{:02}:{:02}", sec / 60, sec % 60))
                                    .color(palette.text_secondary),
                            );
                        },
                    );

                    // Format
                    ui.allocate_ui_with_layout(
                        Vec2::new(80.0, 32.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            let fmt = format_name(track.format);
                            ui.label(RichText::new(fmt).color(palette.text_secondary));
                        },
                    );

                    // Size
                    ui.allocate_ui_with_layout(
                        Vec2::new(90.0, 32.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            let size_mb = std::fs::metadata(&track.file_path)
                                .map(|m| m.len() as f32 / (1024.0 * 1024.0))
                                .unwrap_or(0.0);
                            ui.label(
                                RichText::new(format!("{:.1} MB", size_mb))
                                    .color(palette.text_secondary),
                            );
                        },
                    );
                });
            });

            // Double Click / Click to play
            if response.double_clicked() || (response.clicked() && !is_current) {
                play_track = Some(track.clone());
            }

            // Context Menu (Right Click)
            response.context_menu(|ui| {
                if ui.button("▶ Play").clicked() {
                    play_track = Some(track.clone());
                    ui.close_menu();
                }
                if ui.button("🗑 Remove").clicked() {
                    remove_index = Some(idx);
                    ui.close_menu();
                }
                if ui.button("📁 Open Folder").clicked() {
                    if let Some(parent) = track.file_path.parent() {
                        let _ = std::process::Command::new("explorer").arg(parent).spawn();
                    }
                    ui.close_menu();
                }
            });
        }
    });

    if let Some(idx) = remove_index {
        state.playlist.remove(idx);
        state.toast_manager.notify("Track removed from playlist");
    }

    if let Some(track) = play_track {
        state.current_track = Some(track.clone());
        if let Some(handle) = audio_handle {
            let _ = handle.send_command(PlayerCommand::LoadTrack(track.file_path));
        }
        state.toast_manager.notify("Playback Started");
    }
}

fn format_name(fmt: AudioFormat) -> &'static str {
    match fmt {
        AudioFormat::Mp3 => "MP3",
        AudioFormat::Flac => "FLAC",
        AudioFormat::Wav => "WAV",
        AudioFormat::Aac => "AAC",
        AudioFormat::Ogg => "OGG",
        AudioFormat::Opus => "OPUS",
        AudioFormat::Aiff => "AIFF",
        AudioFormat::Alac => "ALAC",
        AudioFormat::Unknown => "AUDIO",
    }
}
