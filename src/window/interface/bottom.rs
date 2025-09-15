use eframe::egui::{self, Align, Color32, FontFamily, FontId, Frame, Layout, RichText, Vec2};

use crate::tools::img::parse_img;

pub fn render(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom_panel")
        // .max_height(44.0)
        .show(ctx, |ui| {
            Frame::default().inner_margin(10.0).show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(
                        RichText::new("by Mateusz Słotwiński")
                            .color(Color32::from_rgb(0, 180, 130))
                            .font(FontId::new(24.0, FontFamily::Name("pixel_font".into())))
                            .strong(),
                    );

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        let img = parse_img(include_bytes!("../../assets/github.png"), ctx)
                            .fit_to_exact_size(Vec2::splat(22.0))
                            .sense(egui::Sense::click());

                        let response = ui
                            .add(img)
                            .on_hover_text("Open GitHub")
                            .on_hover_cursor(egui::CursorIcon::PointingHand);

                        if response.clicked() {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(
                                "https://github.com/mslotwinski-dev/Pixel",
                            ));
                        }
                    });
                });
            });
        });
}
