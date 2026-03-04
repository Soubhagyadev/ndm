use eframe::egui;
use egui::{Color32, FontId, Pos2, Rect, Stroke, Vec2};

pub fn draw_ndm_canvas(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Allocating a Canvas
        let (response, painter) =
            ui.allocate_painter(Vec2::new(400.0, 200.0), egui::Sense::hover());

        let canvas_rect = response.rect;

        painter.rect_filled(canvas_rect, 8.0, Color32::from_rgb(30, 30, 46));

        painter.rect_stroke(
            canvas_rect,
            8.0,
            Stroke::new(2.0, Color32::from_rgb(100, 150, 255)),
        );

        let text_pos = canvas_rect.center();
        painter.text(
            text_pos,
            egui::Align2::CENTER_CENTER,
            "ndm",
            FontId::proportional(72.0),
            Color32::from_rgb(200, 220, 255),
        );
        let text_width = 120.0;
        let underline_y = text_pos.y + 46.0;
        painter.line_segment(
            [
                Pos2::new(text_pos.x - text_width / 2.0, underline_y),
                Pos2::new(text_pos.x + text_width / 2.0, underline_y),
            ],
            Stroke::new(2.5, Color32::from_rgb(100, 150, 255)),
        );
    });
}
