#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{
    self, Align, Color32, FontData, FontDefinitions, FontFamily, FontId, Frame, Layout, RichText,
    Vec2, vec2,
};
use image;

pub struct Window {}

impl Window {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "pixel_font".to_owned(),
            FontData::from_static(include_bytes!("../assets/pixel_font.ttf")).into(),
        );

        fonts.families.insert(
            FontFamily::Name("pixel_font".into()),
            vec!["pixel_font".to_owned()],
        );

        cc.egui_ctx.set_fonts(fonts);

        Self {}
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                Frame::default().inner_margin(10.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                            let img = parse_img(include_bytes!("../assets/icon.png"), ctx)
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add_space(10.0);

                ui.label("Podtytuł");
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .max_height(44.0)
            .show(ctx, |ui| {
                Frame::default().inner_margin(10.0).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("by Mateusz Slotwinski")
                                .color(Color32::from_rgb(0, 180, 130))
                                .font(FontId::new(24.0, FontFamily::Name("pixel_font".into())))
                                .strong(),
                        );

                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            let img = parse_img(include_bytes!("../assets/github.png"), ctx)
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
}

fn parse_img<'a>(bytes: &'a [u8], ctx: &'a egui::Context) -> egui::Image<'a> {
    let image = image::load_from_memory(&bytes).unwrap().to_rgba8();
    let size = [image.width() as _, image.height() as _];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image);
    let texture = ctx.load_texture("icon", color_image, Default::default());

    egui::Image::new(&texture).fit_to_original_size(1.0)
}
