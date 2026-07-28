use crate::state::{AppState, NavTab};
use crate::views::*;
use aether_audio::AudioEngineHandle;
use aether_core::{PlayerCommand, PlayerEvent};
use eframe::App;
use egui::{Align2, Color32, Rect, Vec2};
use std::time::Duration;

pub struct JustMusicApp {
    pub state: AppState,
    pub audio_handle: Option<AudioEngineHandle>,
}

impl JustMusicApp {
    pub fn new(cc: &eframe::CreationContext<'_>, audio_handle: Option<AudioEngineHandle>) -> Self {
        let state = AppState::load_saved();

        let palette = state.design_system.palette.clone();
        let mut visuals = match state.settings.theme_mode {
            crate::theme::ThemeMode::Light => egui::Visuals::light(),
            _ => egui::Visuals::dark(),
        };
        visuals.window_fill = palette.background;
        visuals.panel_fill = palette.background;
        visuals.override_text_color = Some(palette.text_primary);
        visuals.window_rounding = egui::Rounding::same(12.0);
        cc.egui_ctx.set_visuals(visuals);

        Self {
            state,
            audio_handle,
        }
    }

    pub fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Space -> Toggle Play/Pause
            if i.key_pressed(egui::Key::Space) {
                if let Some(handle) = &self.audio_handle {
                    let _ = handle.send_command(PlayerCommand::TogglePlayPause);
                }
            }

            // Ctrl+O -> Open File
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter(
                        "Audio Files",
                        &[
                            "mp3", "flac", "wav", "aac", "ogg", "opus", "m4a", "wma", "aiff",
                        ],
                    )
                    .pick_file()
                {
                    home_view::load_file(path, &mut self.state, self.audio_handle.as_ref());
                }
            }

            // Arrow Left / Right -> Seek -5s / +5s
            if i.key_pressed(egui::Key::ArrowLeft) {
                let cur = self.state.position.current_ms.saturating_sub(5000);
                if let Some(handle) = &self.audio_handle {
                    let _ = handle.send_command(PlayerCommand::SeekTo(Duration::from_millis(cur)));
                }
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                let cur = self.state.position.current_ms + 5000;
                if let Some(handle) = &self.audio_handle {
                    let _ = handle.send_command(PlayerCommand::SeekTo(Duration::from_millis(cur)));
                }
            }

            // M -> Toggle Mute
            if i.key_pressed(egui::Key::M) {
                self.state.is_muted = !self.state.is_muted;
                if let Some(handle) = &self.audio_handle {
                    let _ = handle.send_command(PlayerCommand::SetMute(self.state.is_muted));
                }
            }

            // Arrow Up / Down -> Volume Up / Down
            if i.key_pressed(egui::Key::ArrowUp) {
                self.state.volume = (self.state.volume + 0.05).min(1.0);
                if let Some(handle) = &self.audio_handle {
                    let _ = handle.send_command(PlayerCommand::SetVolume(self.state.volume));
                }
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.state.volume = (self.state.volume - 0.05).max(0.0);
                if let Some(handle) = &self.audio_handle {
                    let _ = handle.send_command(PlayerCommand::SetVolume(self.state.volume));
                }
            }

            // Ctrl+, -> Settings
            if i.modifiers.command && i.key_pressed(egui::Key::Comma) {
                self.state.current_tab = NavTab::Settings;
            }
        });
    }

    pub fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    if let Some(path) = &file.path {
                        home_view::load_file(
                            path.clone(),
                            &mut self.state,
                            self.audio_handle.as_ref(),
                        );
                    }
                }
            }
        });
    }

    pub fn poll_audio_events(&mut self) {
        if let Some(handle) = &self.audio_handle {
            while let Ok(event) = handle.event_receiver().try_recv() {
                match event {
                    PlayerEvent::StateChanged(state) => {
                        self.state.play_state = state;
                    }
                    PlayerEvent::PositionUpdated(pos) => {
                        self.state.position = pos;
                    }
                    PlayerEvent::TrackStarted(track) => {
                        self.state.current_track = Some(track.clone());
                        self.state.status_text = format!("מנגן כעת: {}", track.title);
                        self.state
                            .toast_manager
                            .notify(format!("מנגן: {}", track.title));
                    }
                    PlayerEvent::TrackEnded => {
                        self.state.play_state = aether_core::PlayState::Stopped;
                        self.state.status_text = "השיר הסתיים".into();
                    }
                    PlayerEvent::ErrorOccurred(err) => {
                        self.state.status_text = format!("שגיאה: {err}");
                        self.state.toast_manager.notify(format!("שגיאה: {err}"));
                    }
                    PlayerEvent::VolumeChanged(vol) => {
                        self.state.volume = vol;
                    }
                    _ => {}
                }
            }
        }
    }
}

impl App for JustMusicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx);
        self.handle_dropped_files(ctx);
        self.poll_audio_events();
        self.state.toast_manager.update();

        let palette = self.state.design_system.palette.clone();
        let is_rtl = self.state.settings.is_rtl;

        // Custom Title Bar Header
        egui::TopBottomPanel::top("header_panel")
            .exact_height(42.0)
            .frame(egui::Frame::none().fill(palette.background))
            .show(ctx, |ui| {
                ui.add_space(4.0);
                header::render_header(ui, &mut self.state);
            });

        // Bottom Player Bar
        egui::TopBottomPanel::bottom("player_panel")
            .exact_height(94.0)
            .frame(egui::Frame::none().fill(palette.cards))
            .show(ctx, |ui| {
                player_bar::render_player_bar(ui, &mut self.state, self.audio_handle.as_ref());
            });

        // Navigation Sidebar (Right side for RTL, Left side for LTR)
        if is_rtl {
            egui::SidePanel::right("sidebar_panel")
                .exact_width(180.0)
                .frame(egui::Frame::none().fill(palette.background))
                .show(ctx, |ui| {
                    sidebar::render_sidebar(ui, &mut self.state);
                });
        } else {
            egui::SidePanel::left("sidebar_panel")
                .exact_width(180.0)
                .frame(egui::Frame::none().fill(palette.background))
                .show(ctx, |ui| {
                    sidebar::render_sidebar(ui, &mut self.state);
                });
        }

        // Central View Panel
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(palette.background))
            .show(ctx, |ui| {
                match self.state.current_tab {
                    NavTab::Home => {
                        home_view::render_home_view(ui, &mut self.state, self.audio_handle.as_ref())
                    }
                    NavTab::Playlist => playlist_view::render_playlist_view(
                        ui,
                        &mut self.state,
                        self.audio_handle.as_ref(),
                    ),
                    NavTab::Recent => recent_view::render_recent_view(
                        ui,
                        &mut self.state,
                        self.audio_handle.as_ref(),
                    ),
                    NavTab::Settings => settings_view::render_settings_view(ui, &mut self.state),
                    NavTab::About => about_view::render_about_view(ui, &mut self.state),
                }

                // Render Floating Toast Notifications
                let toasts = self.state.toast_manager.toasts.clone();
                for (idx, toast) in toasts.iter().enumerate() {
                    let opacity = toast.opacity();
                    if opacity > 0.0 {
                        let y_pos = ui.max_rect().bottom() - 110.0 - (idx as f32 * 45.0);
                        let x_pos = if is_rtl {
                            ui.max_rect().left() + 20.0
                        } else {
                            ui.max_rect().right() - 280.0
                        };
                        let toast_rect = Rect::from_min_size(
                            egui::Pos2::new(x_pos, y_pos),
                            Vec2::new(260.0, 38.0),
                        );

                        let bg =
                            Color32::from_rgba_unmultiplied(34, 34, 34, (230.0 * opacity) as u8);
                        ui.painter().rect_filled(toast_rect, 10.0, bg);
                        ui.painter().text(
                            toast_rect.center(),
                            Align2::CENTER_CENTER,
                            &toast.message,
                            egui::FontId::proportional(13.0),
                            Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * opacity) as u8),
                        );
                    }
                }
            });

        // Request continuous repaint for smooth 60+ FPS animations if playing
        ctx.request_repaint_after(Duration::from_millis(16));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.state.save_config();
    }
}
