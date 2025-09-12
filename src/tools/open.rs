use image;

use crate::utility::log::Log;

pub fn open_image(path: &str) -> image::DynamicImage {
    match image::open(path) {
        Ok(img) => {
            Log::info(&format!("Successfully opened image."));
            img
        }
        Err(e) => {
            Log::error(&format!("Failed to open image. {}", e));
            unreachable!();
        }
    }
}
