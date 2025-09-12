use image;
use image::GenericImageView;

pub fn resize_image(img: &mut image::DynamicImage, width: u32, height: u32) {
    *img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
}

pub fn resize_image_percentage(img: &mut image::DynamicImage, percent: u32) {
    let (width, height) = img.dimensions();
    let new_width = (width as f32 * (percent as f32 / 100.0)).round() as u32;
    let new_height = (height as f32 * (percent as f32 / 100.0)).round() as u32;
    *img = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
}
