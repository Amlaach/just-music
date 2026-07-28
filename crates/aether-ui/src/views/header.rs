use crate::bidi::{BiDiEngine, LayoutDirection};
use crate::state::{AppState, NavTab};
use egui::{Align, Align2, Color32, Layout, RichText, Sense, Vec2, ViewportCommand};

pub fn render_header(ui: &mut egui::Ui, state: &mut AppState) {
    let palette = state.design_system.palette.clone();

    // Title bar drag region for frameless window
    let title_bar_rect = ui.max_rect();
    let title_bar_response = ui.interact(title_bar_rect, ui.id().with("title_bar"), Sense::click());
    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or_default());
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 0.0);

        // Logo & App Name
        ui.allocate_ui_with_layout(
            Vec2::new(220.0, 32.0),
            Layout::left_to_right(Align::Center),
            |ui| {
                let (rect, _response) =
                    ui.allocate_exact_size(Vec2::new(26.0, 26.0), Sense::hover());
                ui.painter()
                    .circle_filled(rect.center(), 13.0, palette.primary);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "🎵",
                    egui::FontId::proportional(13.0),
                    Color32::WHITE,
                );

                ui.label(
                    RichText::new("Just Music")
                        .size(16.0)
                        .strong()
                        .color(palette.text_primary),
                );

                ui.label(
                    RichText::new("v1.0")
                        .size(11.0)
                        .color(palette.text_secondary),
                );
            },
        );

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            // Window action buttons
            let button_size = Vec2::new(32.0, 26.0);

            // Close
            let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());
            let close_bg = if response.hovered() {
                Color32::from_rgb(227, 92, 92)
            } else {
                Color32::TRANSPARENT
            };
            ui.painter().rect_filled(rect, 6.0, close_bg);
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "✕",
                egui::FontId::proportional(12.0),
                if response.hovered() {
                    Color32::WHITE
                } else {
                    palette.text_secondary
                },
            );
            if response.clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Close);
            }

            // Maximize / Restore
            let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());
            if response.hovered() {
                ui.painter().rect_filled(rect, 6.0, palette.borders);
            }
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "□",
                egui::FontId::proportional(14.0),
                palette.text_secondary,
            );
            if response.clicked() {
                let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or_default());
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
            }

            // Minimize
            let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());
            if response.hovered() {
                ui.painter().rect_filled(rect, 6.0, palette.borders);
            }
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "─",
                egui::FontId::proportional(12.0),
                palette.text_secondary,
            );
            if response.clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
            }

            ui.add_space(12.0);

            // Language / RTL Switcher Button (עב / EN)
            let lang_label = if state.settings.is_rtl {
                "🌐 עברית"
            } else {
                "🌐 English"
            };
            if ui
                .button(RichText::new(lang_label).size(13.0).color(palette.primary))
                .clicked()
            {
                state.settings.is_rtl = !state.settings.is_rtl;
                state.bidi_engine = BiDiEngine::new(if state.settings.is_rtl {
                    LayoutDirection::Rtl
                } else {
                    LayoutDirection::Ltr
                });
                state.save_config();
            }

            ui.add_space(8.0);

            // Quick Settings & About icons
            if ui
                .button(RichText::new("⚙").size(15.0).color(palette.text_secondary))
                .clicked()
            {
                state.current_tab = NavTab::Settings;
            }
            if ui
                .button(RichText::new("ℹ").size(15.0).color(palette.text_secondary))
                .clicked()
            {
                state.current_tab = NavTab::About;
            }
        });
    });
}
