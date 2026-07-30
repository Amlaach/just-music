use crate::state::AppState;
use egui::{Align2, Color32, RichText, Vec2};

pub fn render_about_view(ui: &mut egui::Ui, state: &mut AppState) {
    let palette = state.design_system.palette.clone();

    ui.vertical_centered(|ui| {
        ui.add_space(30.0);

        // Logo Circle
        let (rect, _response) = ui.allocate_exact_size(Vec2::new(72.0, 72.0), egui::Sense::hover());
        ui.painter()
            .circle_filled(rect.center(), 36.0, palette.primary);
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            "🎵",
            egui::FontId::proportional(36.0),
            Color32::WHITE,
        );

        ui.add_space(15.0);

        ui.label(
            RichText::new("Jost Music")
                .size(26.0)
                .strong()
                .color(palette.text_primary),
        );

        ui.label(
            RichText::new("מוזיקה")
                .size(16.0)
                .strong()
                .color(palette.accent),
        );

        ui.add_space(8.0);

        ui.label(
            RichText::new("High-Performance Luxury Dark Crimson Desktop Audio Software")
                .size(13.0)
                .color(palette.text_secondary),
        );

        ui.add_space(20.0);

        ui.group(|ui| {
            ui.set_max_width(420.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Version / גרסה:").strong());
                    ui.label("v1.1.0 Premium");
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Developer / מפתח:").strong());
                    ui.label("Amlaach / Jost Systems");
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Architecture / ארכיטקטורה:").strong());
                    ui.label("Pure Rust Lock-Free Headless Audio Engine");
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("GUI / ממשק:").strong());
                    ui.label("Eframe/Egui Dark Crimson & Cyan Palette");
                });
            });
        });

        ui.add_space(20.0);

        ui.hyperlink_to("📂 צפה בפרויקט ב-GitHub / Open GitHub Repository", "https://github.com/Amlaach/just-music");
    });
}
