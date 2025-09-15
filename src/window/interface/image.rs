use crate::tools::img::fit_size;
use crate::window::window::Window;

use eframe::egui::{self, Color32, Frame, Margin, vec2};

pub fn render(ui: &mut egui::Ui, win: &mut Window) {
    Frame::default()
        .stroke(egui::Stroke::new(2.0, Color32::from_rgb(0, 180, 130)))
        .inner_margin(Margin::symmetric(10, 10))
        .outer_margin(Margin::symmetric(10, 10))
        .corner_radius(5.0)
        .show(ui, |ui| {
            if let Some(texture) = &win.texture {
                let max_size = vec2(ui.available_width(), ui.available_height());
                let scaled = fit_size(texture.size_vec2(), max_size);
                ui.image((texture.id(), scaled));
            }
        });
}
