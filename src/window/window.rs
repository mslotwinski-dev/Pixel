#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use image;

pub struct Window {}

impl Default for Window {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pixel");

            let img = parse_img(include_bytes!("../assets/icon.png"), ctx);

            ui.add(img);
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
