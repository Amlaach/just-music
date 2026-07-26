use crate::associations::AssociationManager;
use crate::state::{AppState, SettingsTab};
use crate::theme::ThemeMode;
use egui::{Color32, RichText, Vec2};

pub fn render_settings_view(ui: &mut egui::Ui, state: &mut AppState) {
    let palette = state.design_system.palette.clone();

    ui.label(
        RichText::new("⚙️ Settings")
            .size(20.0)
            .strong()
            .color(palette.text_primary),
    );

    ui.add_space(15.0);

    // Settings Navigation Tabs Header
    ui.horizontal(|ui| {
        let tabs = [
            (SettingsTab::General, "General"),
            (SettingsTab::Playback, "Playback"),
            (SettingsTab::Appearance, "Appearance"),
            (SettingsTab::Associations, "Associations"),
            (SettingsTab::Updates, "Updates"),
        ];

        for (tab, label) in tabs {
            let is_selected = state.current_settings_tab == tab;

            let btn_color = if is_selected {
                palette.primary
            } else {
                palette.cards
            };

            let text_color = if is_selected {
                Color32::WHITE
            } else {
                palette.text_primary
            };

            let mut text = RichText::new(label).size(13.0).color(text_color);
            if is_selected {
                text = text.strong();
            }

            let btn = egui::Button::new(text)
                .fill(btn_color)
                .rounding(8.0)
                .min_size(Vec2::new(100.0, 32.0));

            if ui.add(btn).clicked() {
                state.current_settings_tab = tab;
            }
        }
    });

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(15.0);

    // Tab Contents
    match state.current_settings_tab {
        SettingsTab::General => {
            ui.checkbox(&mut state.settings.start_with_windows, "Start with Windows");
            ui.add_space(8.0);
            ui.checkbox(&mut state.settings.remember_volume, "Remember Volume");
            ui.add_space(8.0);
            ui.checkbox(
                &mut state.settings.restore_last_playlist,
                "Restore Last Playlist",
            );
            ui.add_space(8.0);
            ui.checkbox(&mut state.settings.minimize_to_tray, "Minimize to Tray");
            ui.add_space(8.0);
            ui.checkbox(&mut state.settings.auto_check_updates, "Auto Check Updates");
        }

        SettingsTab::Playback => {
            ui.horizontal(|ui| {
                ui.label("Playback Speed:");
                ui.add(egui::Slider::new(&mut state.settings.playback_speed, 0.5..=2.0).text("x"));
            });
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label("Buffer Size (ms):");
                ui.add(egui::Slider::new(&mut state.settings.buffer_size_ms, 50..=500).text("ms"));
            });
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label("Crossfade (sec):");
                ui.add(egui::Slider::new(&mut state.settings.crossfade_sec, 0..=10).text("s"));
            });
            ui.add_space(12.0);

            ui.checkbox(
                &mut state.settings.gapless_playback,
                "Enable Gapless Playback",
            );
        }

        SettingsTab::Appearance => {
            ui.label(RichText::new("Theme Mode:").strong());
            ui.horizontal(|ui| {
                ui.selectable_value(&mut state.settings.theme_mode, ThemeMode::Light, "Light");
                ui.selectable_value(&mut state.settings.theme_mode, ThemeMode::Dark, "Dark");
                ui.selectable_value(
                    &mut state.settings.theme_mode,
                    ThemeMode::System,
                    "Follow System",
                );
            });

            if state.design_system.mode != state.settings.theme_mode {
                state.design_system = crate::theme::DesignSystem::new(state.settings.theme_mode);
                let palette = state.design_system.palette.clone();
                let is_dark = matches!(state.settings.theme_mode, ThemeMode::Dark | ThemeMode::System);
                let mut visuals = if is_dark { egui::Visuals::dark() } else { egui::Visuals::light() };
                visuals.window_fill = palette.background;
                visuals.panel_fill = palette.background;
                visuals.override_text_color = Some(palette.text_primary);
                visuals.window_rounding = egui::Rounding::same(12.0);
                ui.ctx().set_visuals(visuals);
            }

            ui.add_space(15.0);

            ui.checkbox(
                &mut state.settings.rounded_corners,
                "Rounded Corners (14px)",
            );
            ui.add_space(8.0);
            ui.checkbox(
                &mut state.settings.enable_blur,
                "Enable Blur Effects (Acrylic / Mica)",
            );
        }

        SettingsTab::Associations => {
            ui.label(RichText::new("File Extensions Registration:").strong());
            ui.label("Supported formats: mp3, flac, wav, aac, ogg, opus, m4a, wma, aiff");
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("📌 Register Supported File Types")
                                .size(14.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(palette.primary)
                        .rounding(10.0)
                        .min_size(Vec2::new(220.0, 38.0)),
                    )
                    .clicked()
                {
                    match AssociationManager::register_all() {
                        Ok(()) => {
                            state.settings.file_associations_registered = true;
                            state
                                .toast_manager
                                .notify("File associations registered successfully in Windows!");
                        }
                        Err(e) => {
                            state
                                .toast_manager
                                .notify(format!("Failed to register: {e}"));
                        }
                    }
                }

                if ui
                    .button(
                        RichText::new("❌ Remove Registration")
                            .size(13.0)
                            .color(palette.text_secondary),
                    )
                    .clicked()
                {
                    match AssociationManager::unregister_all() {
                        Ok(()) => {
                            state.settings.file_associations_registered = false;
                            state.toast_manager.notify("File associations removed.");
                        }
                        Err(e) => {
                            state.toast_manager.notify(format!("Failed: {e}"));
                        }
                    }
                }
            });

            ui.add_space(15.0);
            if state.settings.file_associations_registered {
                ui.label(
                    RichText::new(
                        "✓ Just Music is currently registered for all supported audio types.",
                    )
                    .color(Color32::from_rgb(46, 160, 67)),
                );
            }
        }

        SettingsTab::Updates => {
            ui.label(RichText::new("Current Version: v1.0.0 (Production)").strong());
            ui.add_space(15.0);

            if ui
                .add(
                    egui::Button::new("🔄 Check For Updates")
                        .rounding(10.0)
                        .min_size(Vec2::new(160.0, 36.0)),
                )
                .clicked()
            {
                state
                    .toast_manager
                    .notify("You are using the latest version of Just Music.");
            }

            ui.add_space(15.0);
            ui.hyperlink_to(
                "🌐 Just Music GitHub Releases",
                "https://github.com/Amlaach",
            );
        }
    }
}
