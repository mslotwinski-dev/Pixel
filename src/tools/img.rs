use eframe::egui::{self, ColorImage};
use image::DynamicImage;

pub fn parse_img<'a>(bytes: &'a [u8], ctx: &'a egui::Context) -> egui::Image<'a> {
    let image = image::load_from_memory(&bytes).unwrap().to_rgba8();
    let size = [image.width() as _, image.height() as _];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image);
    let texture = ctx.load_texture("icon", color_image, egui::TextureOptions::LINEAR);

    egui::Image::new(&texture).fit_to_original_size(1.0)
}

pub fn dynamic_image_to_color_image(img: &DynamicImage) -> ColorImage {
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_vec();
    ColorImage::from_rgba_unmultiplied(size, &pixels)
}

// fn add_dynamic_image(ui: &mut Ui, ctx: &egui::Context, img: &DynamicImage) -> TextureHandle {
//     let color_image = dynamic_image_to_color_image(img);
//     let texture = ctx.load_texture("dyn-img", color_image, TextureOptions::default());
//     ui.image((texture.id(), texture.size_vec2()));
//     texture
// }

pub fn fit_size(original: egui::Vec2, max: egui::Vec2) -> egui::Vec2 {
    let scale = (max.x / original.x).min(max.y / original.y).min(1.0);
    original * scale
}
