use crate::state::AppState;
use aether_audio::AudioEngineHandle;
use aether_core::{PlayState, PlayerCommand};
use egui::{Align, Color32, Layout, RichText, Vec2};
use std::time::Duration;

pub fn render_player_bar(
    ui: &mut egui::Ui,
    state: &mut AppState,
    audio_handle: Option<&AudioEngineHandle>,
) {
    let palette = state.design_system.palette.clone();
    let is_rtl = state.settings.is_rtl;

    ui.vertical(|ui| {
        // Separator top line
        ui.painter().line_segment(
            [ui.max_rect().left_top(), ui.max_rect().right_top()],
            egui::Stroke::new(1.0_f32, palette.borders),
        );

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.add_space(10.0);

            // Left: Playback Controls (Prev, Play/Pause, Next)
            ui.allocate_ui_with_layout(
                Vec2::new(160.0, 36.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    // Prev
                    if ui.button(RichText::new("⏮").size(16.0)).clicked() {
                        if let Some(handle) = audio_handle {
                            let _ = handle.send_command(PlayerCommand::PreviousTrack);
                        }
                    }

                    // Play / Pause
                    let is_playing = state.play_state == PlayState::Playing;
                    let play_icon = if is_playing { "⏸" } else { "▶" };

                    let play_btn = egui::Button::new(
                        RichText::new(play_icon)
                            .size(16.0)
                            .strong()
                            .color(Color32::WHITE),
                    )
                    .fill(palette.primary)
                    .rounding(18.0)
                    .min_size(Vec2::new(36.0, 36.0));

                    if ui.add(play_btn).clicked() {
                        if is_playing {
                            state.play_state = PlayState::Paused;
                            if let Some(handle) = audio_handle {
                                let _ = handle.send_command(PlayerCommand::Pause);
                            }
                        } else {
                            state.play_state = PlayState::Playing;
                            if let Some(handle) = audio_handle {
                                let _ = handle.send_command(PlayerCommand::Play);
                            }
                        }
                    }

                    // Next
                    if ui.button(RichText::new("⏭").size(16.0)).clicked() {
                        if let Some(handle) = audio_handle {
                            let _ = handle.send_command(PlayerCommand::NextTrack);
                        }
                    }
                },
            );

            // Center: Interactive Progress Bar with Click Seeking
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add_space(15.0);

                let current_sec = state.position.current_ms / 1000;
                let total_sec = state.position.total_ms / 1000;

                ui.label(
                    RichText::new(format!("{:02}:{:02}", current_sec / 60, current_sec % 60))
                        .size(12.0)
                        .color(palette.text_secondary),
                );

                ui.add_space(8.0);

                let progress_width = (ui.available_width() - 180.0).max(100.0);
                let (rect, response) = ui.allocate_exact_size(
                    Vec2::new(progress_width, 10.0),
                    egui::Sense::click_and_drag(),
                );

                // Background track
                ui.painter().rect_filled(rect, 5.0, palette.borders);

                // Handle click or drag to seek
                if (response.clicked() || response.dragged()) && state.position.total_ms > 0 {
                    if let Some(mouse_pos) = response.interact_pointer_pos() {
                        let click_x = (mouse_pos.x - rect.min.x).clamp(0.0, rect.width());
                        let ratio = click_x / rect.width();
                        let target_ms = (ratio as f64 * state.position.total_ms as f64) as u64;
                        state.position.progress_ratio = ratio;
                        state.position.current_ms = target_ms;

                        if let Some(handle) = audio_handle {
                            let _ = handle.send_command(PlayerCommand::SeekTo(
                                Duration::from_millis(target_ms),
                            ));
                        }
                    }
                }

                // Filled progress
                let filled_width = rect.width() * state.position.progress_ratio.clamp(0.0, 1.0);
                if filled_width > 0.0 {
                    let filled_rect =
                        egui::Rect::from_min_size(rect.min, Vec2::new(filled_width, rect.height()));
                    ui.painter().rect_filled(filled_rect, 5.0, palette.primary);

                    // Scrub handle circle on hover / active
                    let handle_pos = egui::Pos2::new(rect.min.x + filled_width, rect.center().y);
                    ui.painter().circle_filled(handle_pos, 6.0, Color32::WHITE);
                }

                ui.add_space(8.0);

                ui.label(
                    RichText::new(format!("{:02}:{:02}", total_sec / 60, total_sec % 60))
                        .size(12.0)
                        .color(palette.text_secondary),
                );
            });

            // Right: Volume Control Slider
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(10.0);

                let volume_icon = if state.is_muted || state.volume == 0.0 {
                    "🔇"
                } else if state.volume < 0.5 {
                    "🔉"
                } else {
                    "🔊"
                };

                if ui.button(RichText::new(volume_icon).size(15.0)).clicked() {
                    state.is_muted = !state.is_muted;
                    if let Some(handle) = audio_handle {
                        let _ = handle.send_command(PlayerCommand::SetMute(state.is_muted));
                    }
                }

                let slider = egui::Slider::new(&mut state.volume, 0.0..=1.0)
                    .show_value(false)
                    .trailing_fill(true);

                if ui.add_sized(Vec2::new(90.0, 20.0), slider).changed() {
                    if let Some(handle) = audio_handle {
                        let _ = handle.send_command(PlayerCommand::SetVolume(state.volume));
                    }
                }
            });
        });

        ui.add_space(4.0);

        // Status Bar
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            let state_str = match (state.play_state, is_rtl) {
                (PlayState::Playing, true) => "▶ מנגן",
                (PlayState::Playing, false) => "▶ Playing",
                (PlayState::Paused, true) => "⏸ מושהה",
                (PlayState::Paused, false) => "⏸ Paused",
                (PlayState::Stopped, true) => "⏹ מופסק",
                (PlayState::Stopped, false) => "⏹ Stopped",
            };
            let status_prefix = if is_rtl { "סטטוס" } else { "Status" };
            ui.label(
                RichText::new(format!(
                    "{}: {} | {}",
                    status_prefix, state_str, state.status_text
                ))
                .size(11.0)
                .color(palette.text_secondary),
            );
        });
    });
}
