use crate::tools::img::parse_img;

use eframe::egui::{self, RichText, vec2};

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

pub fn filter_button(ui: &mut egui::Ui, ctx: &egui::Context, name: &str) -> egui::Response {
    let response = ui
        .add(
            parse_img(include_bytes!("../assets/icons/landscape.png"), ctx)
                .fit_to_exact_size(vec2(40.0, 27.0))
                .sense(egui::Sense::click()),
        )
        .on_hover_text(name)
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    ui.label(RichText::new(name).color(egui::Color32::WHITE).size(11.0));
    ui.add_space(20.0);

    response
}
