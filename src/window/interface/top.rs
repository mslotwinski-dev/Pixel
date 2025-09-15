use eframe::egui::{self, Color32, FontFamily, FontId, Frame, Layout, RichText, vec2};

use crate::tools::img::parse_img;

pub fn render(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            Frame::default().inner_margin(10.0).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(12.0);
                    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                        let img = parse_img(include_bytes!("../../assets/icon.png"), ctx)
                            .fit_to_exact_size(vec2(34.0, 34.0));
                        ui.add(img);

                        ui.add_space(12.0);

                        ui.heading(
                            RichText::new("PIXEL")
                                .color(Color32::from_rgb(0, 180, 130))
                                .size(32.0)
                                .font(FontId::new(40.0, FontFamily::Name("pixel_font".into())))
                                .strong(),
                        );
                    });
                })
            });
        });
    });
}
