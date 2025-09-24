use eframe::egui::{Context, TextureOptions, Vec2};
use image;

use crate::tools::img::dynamic_image_to_color_image;
use crate::utility::log::Log;
use crate::window::window::Window;

pub fn open_image(path: &str) -> Option<image::DynamicImage> {
    match image::open(path) {
        Ok(img) => {
            Log::info(&format!("Successfully opened image."));
            Some(img)
        }
        Err(e) => {
            Log::error(&format!("Failed to open image. {}", e));
            None
        }
    }
}

pub fn open_image_gui(ctx: &Context, win: &mut Window) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "gif", "tiff"])
        .set_title("Open Image")
        .pick_file()
    {
        load_image(ctx, win, path.to_str().unwrap());
    }
}

pub fn load_image(ctx: &Context, win: &mut Window, path: &str) {
    if let Some(img) = open_image(path) {
        let color_image = dynamic_image_to_color_image(&img);
        let texture = ctx.load_texture("dyn-img", color_image, TextureOptions::default());

        win.input_path = Some(path.to_string());
        win.image = Some(img.clone());
        win.original_image = Some(img);
        win.texture = Some(texture);
    } else {
        win.input_path = None;
        win.image = None;
        win.original_image = None;
        win.texture = None;
    }
    win.zoom = 1.0;
    win.offset = Vec2::ZERO;
}
