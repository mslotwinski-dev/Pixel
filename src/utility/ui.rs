use crate::tools::img::parse_img;

use eframe::egui::{self, vec2};

pub fn icon_button_sized(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    path: &[u8],
    tooltip: &str,
    size: f32,
) -> egui::Response {
    ui.add(
        parse_img(path, ctx)
            .fit_to_exact_size(vec2(size, size))
            .sense(egui::Sense::click()),
    )
    .on_hover_text(tooltip)
    .on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn icon_button(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    path: &[u8],
    tooltip: &str,
) -> egui::Response {
    icon_button_sized(ui, ctx, path, tooltip, 40.0)
}
