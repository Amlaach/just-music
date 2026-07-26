use crate::state::{AppState, NavTab};
use egui::{Align, Color32, Layout, RichText, Sense, Vec2};

pub fn render_sidebar(ui: &mut egui::Ui, state: &mut AppState) {
    let palette = &state.design_system.palette;

    ui.add_space(8.0);

    let tabs = [
        (NavTab::Home, "🏠", "Home"),
        (NavTab::Playlist, "🎵", "Playlist"),
        (NavTab::Recent, "🕒", "Recent"),
        (NavTab::Settings, "⚙️", "Settings"),
        (NavTab::About, "ℹ️", "About"),
    ];

    for (tab, icon, label) in tabs {
        let is_selected = state.current_tab == tab;

        let (rect, response) = ui.allocate_exact_size(Vec2::new(160.0, 38.0), Sense::click());

        let bg = if is_selected {
            palette.primary
        } else if response.hovered() {
            palette.borders
        } else {
            Color32::TRANSPARENT
        };

        let text_color = if is_selected {
            Color32::WHITE
        } else {
            palette.text_primary
        };

        ui.painter().rect_filled(rect, 10.0, bg);

        ui.allocate_ui_at_rect(rect, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add_space(14.0);
                ui.label(RichText::new(icon).size(16.0));
                ui.add_space(8.0);
                let mut text = RichText::new(label).size(14.0).color(text_color);
                if is_selected {
                    text = text.strong();
                }
                ui.label(text);
            });
        });

        if response.clicked() {
            state.current_tab = tab;
        }

        ui.add_space(4.0);
    }
}
