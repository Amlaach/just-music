use crate::associations::AssociationManager;
use crate::bidi::{BiDiEngine, LayoutDirection};
use crate::state::{AppState, SettingsTab};
use crate::theme::ThemeMode;
use egui::{Color32, RichText, Vec2};

pub fn render_settings_view(ui: &mut egui::Ui, state: &mut AppState) {
    let palette = state.design_system.palette.clone();
    let is_rtl = state.settings.is_rtl;

    let header_title = if is_rtl {
        "⚙️ הגדרות"
    } else {
        "⚙️ Settings"
    };
    ui.label(
        RichText::new(header_title)
            .size(20.0)
            .strong()
            .color(palette.text_primary),
    );

    ui.add_space(15.0);

    // Settings Navigation Tabs Header
    ui.horizontal(|ui| {
        let tabs = if is_rtl {
            vec![
                (SettingsTab::General, "כללי"),
                (SettingsTab::Playback, "נגינה"),
                (SettingsTab::Appearance, "מראה ותשתיות"),
                (SettingsTab::Associations, "שיוך קבצים"),
                (SettingsTab::Updates, "עדכונים"),
            ]
        } else {
            vec![
                (SettingsTab::General, "General"),
                (SettingsTab::Playback, "Playback"),
                (SettingsTab::Appearance, "Appearance"),
                (SettingsTab::Associations, "Associations"),
                (SettingsTab::Updates, "Updates"),
            ]
        };

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
            let chk1 = if is_rtl {
                "הפעל עם Windows"
            } else {
                "Start with Windows"
            };
            let chk2 = if is_rtl {
                "זכור עוצמת שמע"
            } else {
                "Remember Volume"
            };
            let chk3 = if is_rtl {
                "שחזר רשימת השמעה אחרונה"
            } else {
                "Restore Last Playlist"
            };
            let chk4 = if is_rtl {
                "מזער למגש המערכת (Tray)"
            } else {
                "Minimize to Tray"
            };
            let chk5 = if is_rtl {
                "בדוק עדכונים באופן אוטומטי"
            } else {
                "Auto Check Updates"
            };

            ui.checkbox(&mut state.settings.start_with_windows, chk1);
            ui.add_space(8.0);
            ui.checkbox(&mut state.settings.remember_volume, chk2);
            ui.add_space(8.0);
            ui.checkbox(&mut state.settings.restore_last_playlist, chk3);
            ui.add_space(8.0);
            ui.checkbox(&mut state.settings.minimize_to_tray, chk4);
            ui.add_space(8.0);
            ui.checkbox(&mut state.settings.auto_check_updates, chk5);
            state.save_config();
        }

        SettingsTab::Playback => {
            let label_speed = if is_rtl {
                "מהירות נגינה:"
            } else {
                "Playback Speed:"
            };
            let label_buffer = if is_rtl {
                "גודל אוגר שמע (ms):"
            } else {
                "Buffer Size (ms):"
            };
            let label_crossfade = if is_rtl {
                "מעבר עדין (שניות):"
            } else {
                "Crossfade (sec):"
            };
            let label_gapless = if is_rtl {
                "אפשר נגינה ללא מרווחים (Gapless)"
            } else {
                "Enable Gapless Playback"
            };

            ui.horizontal(|ui| {
                ui.label(label_speed);
                ui.add(egui::Slider::new(&mut state.settings.playback_speed, 0.5..=2.0).text("x"));
            });
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label(label_buffer);
                ui.add(egui::Slider::new(&mut state.settings.buffer_size_ms, 50..=500).text("ms"));
            });
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label(label_crossfade);
                ui.add(egui::Slider::new(&mut state.settings.crossfade_sec, 0..=10).text("s"));
            });
            ui.add_space(12.0);

            ui.checkbox(&mut state.settings.gapless_playback, label_gapless);
            state.save_config();
        }

        SettingsTab::Appearance => {
            let theme_label = if is_rtl {
                "ערכת נושא:"
            } else {
                "Theme Mode:"
            };
            let light_label = if is_rtl { "בהיר" } else { "Light" };
            let dark_label = if is_rtl { "כהה" } else { "Dark" };
            let night_red_label = if is_rtl {
                "מצב לילה מוגבר (Deep Dark Red)"
            } else {
                "Deep Dark Night Red"
            };
            let sys_label = if is_rtl {
                "לפי מערכת"
            } else {
                "Follow System"
            };

            ui.label(RichText::new(theme_label).strong());
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut state.settings.theme_mode,
                    ThemeMode::Light,
                    light_label,
                );
                ui.selectable_value(&mut state.settings.theme_mode, ThemeMode::Dark, dark_label);
                ui.selectable_value(
                    &mut state.settings.theme_mode,
                    ThemeMode::DeepNightRed,
                    night_red_label,
                );
                ui.selectable_value(&mut state.settings.theme_mode, ThemeMode::System, sys_label);
            });

            if state.design_system.mode != state.settings.theme_mode {
                state.design_system = crate::theme::DesignSystem::new(state.settings.theme_mode);
                let palette = state.design_system.palette.clone();
                let is_dark = matches!(
                    state.settings.theme_mode,
                    ThemeMode::Dark | ThemeMode::System
                );
                let mut visuals = if is_dark {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                };
                visuals.window_fill = palette.background;
                visuals.panel_fill = palette.background;
                visuals.override_text_color = Some(palette.text_primary);
                visuals.window_rounding = egui::Rounding::same(12.0);
                ui.ctx().set_visuals(visuals);
                state.save_config();
            }

            ui.add_space(15.0);

            let rtl_label = if is_rtl {
                "ממשק מימין לשמאל (RTL / עברית)"
            } else {
                "Right-to-Left Layout (RTL / Hebrew)"
            };
            if ui.checkbox(&mut state.settings.is_rtl, rtl_label).changed() {
                state.bidi_engine = BiDiEngine::new(if state.settings.is_rtl {
                    LayoutDirection::Rtl
                } else {
                    LayoutDirection::Ltr
                });
                state.save_config();
            }

            ui.add_space(8.0);
            let round_label = if is_rtl {
                "פינות מעוגלות בסגנון Fluent"
            } else {
                "Rounded Corners (Fluent Style)"
            };
            ui.checkbox(&mut state.settings.rounded_corners, round_label);

            ui.add_space(8.0);
            let blur_label = if is_rtl {
                "אפקט שקיפות וטשטוש (Acrylic / Mica)"
            } else {
                "Blur Effects (Acrylic / Mica)"
            };
            ui.checkbox(&mut state.settings.enable_blur, blur_label);
            state.save_config();
        }

        SettingsTab::Associations => {
            let assoc_title = if is_rtl {
                "רישום שיוכי קבצים ב-Windows:"
            } else {
                "File Extensions Registration:"
            };
            let assoc_sub = if is_rtl {
                "פורמטים נתמכים: mp3, flac, wav, ogg, m4a, aac, opus, wma, aiff"
            } else {
                "Supported formats: mp3, flac, wav, ogg, m4a, aac, opus, wma, aiff"
            };

            ui.label(RichText::new(assoc_title).strong());
            ui.label(assoc_sub);
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                let reg_btn_text = if is_rtl {
                    "📌 רשום שיוכי קבצים ב-Windows"
                } else {
                    "📌 Register Supported File Types"
                };
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(reg_btn_text)
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
                            state.save_config();
                            let msg = if is_rtl {
                                "שיוכי הקבצים נרשמו בהצלחה ב-Windows!"
                            } else {
                                "File associations registered successfully in Windows!"
                            };
                            state.toast_manager.notify(msg);
                        }
                        Err(e) => {
                            state.toast_manager.notify(format!("שגיאה ברישום: {e}"));
                        }
                    }
                }

                let unreg_text = if is_rtl {
                    "❌ הסר רישום"
                } else {
                    "❌ Remove Registration"
                };
                if ui
                    .button(
                        RichText::new(unreg_text)
                            .size(13.0)
                            .color(palette.text_secondary),
                    )
                    .clicked()
                {
                    match AssociationManager::unregister_all() {
                        Ok(()) => {
                            state.settings.file_associations_registered = false;
                            state.save_config();
                            let msg = if is_rtl {
                                "שיוכי הקבצים הוסרו"
                            } else {
                                "File associations removed."
                            };
                            state.toast_manager.notify(msg);
                        }
                        Err(e) => {
                            state.toast_manager.notify(format!("שגיאה: {e}"));
                        }
                    }
                }
            });

            ui.add_space(15.0);
            if state.settings.file_associations_registered {
                let ok_msg = if is_rtl {
                    "✓ Just Music מוגדר כנגן ברירת המחדל עבור כל פורמטי השמע הנתמכים."
                } else {
                    "✓ Just Music is currently registered for all supported audio types."
                };
                ui.label(RichText::new(ok_msg).color(Color32::from_rgb(46, 160, 67)));
            }
        }

        SettingsTab::Updates => {
            let ver_text = if is_rtl {
                "גרסה נוכחית: v1.1.0 (ייצור)"
            } else {
                "Current Version: v1.1.0 (Production)"
            };
            ui.label(RichText::new(ver_text).strong());
            ui.add_space(15.0);

            let check_btn = if is_rtl {
                "🔄 בדוק גרסאות חדשות"
            } else {
                "🔄 Check For Updates"
            };
            if ui
                .add(
                    egui::Button::new(check_btn)
                        .rounding(10.0)
                        .min_size(Vec2::new(160.0, 36.0)),
                )
                .clicked()
            {
                let latest_msg = if is_rtl {
                    "אתה משתמש בגרסה העדכנית ביותר של Just Music."
                } else {
                    "You are using the latest version of Just Music."
                };
                state.toast_manager.notify(latest_msg);
            }

            ui.add_space(15.0);
            ui.hyperlink_to(
                "🌐 Just Music GitHub Releases",
                "https://github.com/Amlaach",
            );
        }
    }
}
