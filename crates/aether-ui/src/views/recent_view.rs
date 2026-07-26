use crate::state::AppState;
use aether_audio::AudioEngineHandle;
use aether_core::PlayerCommand;
use egui::{RichText, Vec2};

pub fn render_recent_view(
    ui: &mut egui::Ui,
    state: &mut AppState,
    audio_handle: Option<&AudioEngineHandle>,
) {
    let palette = state.design_system.palette.clone();

    ui.horizontal(|ui| {
        ui.label(
            RichText::new("🕒 Recent Songs")
                .size(20.0)
                .strong()
                .color(palette.text_primary),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button(
                    RichText::new("🗑 Clear History")
                        .size(13.0)
                        .color(palette.text_secondary),
                )
                .clicked()
            {
                state.recent_tracks.clear();
                state.toast_manager.notify("Recent history cleared");
            }
        });
    });

    ui.add_space(15.0);

    if state.recent_tracks.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(RichText::new("⏳").size(48.0));
            ui.add_space(10.0);
            ui.label(
                RichText::new("No recent history")
                    .size(16.0)
                    .strong()
                    .color(palette.text_secondary),
            );
            ui.label(
                RichText::new("Played tracks will automatically appear here")
                    .size(13.0)
                    .color(palette.text_secondary),
            );
        });
        return;
    }

    let mut play_track = None;

    egui::ScrollArea::vertical().show(ui, |ui| {
        for track in state.recent_tracks.iter() {
            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 44.0), egui::Sense::click());

            let bg = if response.hovered() {
                palette.borders
            } else {
                palette.cards
            };

            ui.painter().rect_filled(rect, 10.0, bg);

            ui.allocate_ui_at_rect(rect, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    ui.label(RichText::new("🎵").size(18.0));
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(&track.title)
                                .strong()
                                .color(palette.text_primary),
                        );
                        ui.label(
                            RichText::new(&track.artist)
                                .size(12.0)
                                .color(palette.text_secondary),
                        );
                    });
                });
            });

            if response.clicked() {
                play_track = Some(track.clone());
            }
        }
    });

    if let Some(track) = play_track {
        state.current_track = Some(track.clone());
        if let Some(handle) = audio_handle {
            let _ = handle.send_command(PlayerCommand::LoadTrack(track.file_path));
        }
        state.toast_manager.notify("Playback Started");
    }
}
