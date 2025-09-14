use eframe::egui::{self, IconData};
use image::GenericImageView;

use crate::window::window::Window;

pub fn run() -> eframe::Result {
    env_logger::init();

    let icon_image = image::open("src/assets/icon.png").expect("Failed to open icon image");
    let icon_rgba = icon_image.to_rgba8();
    let (width, height) = icon_image.dimensions();

    let icon_data = IconData {
        rgba: icon_rgba.into_vec(),
        width: width as u32,
        height: height as u32,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1350.0, 800.0])
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "Pixel",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(Window::new(cc)))
        }),
    )
}
