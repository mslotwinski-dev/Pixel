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

pub fn rotate_image(img: &mut image::DynamicImage, degrees: u32) {
    let times = (degrees / 90) % 4;
    *img = match times {
        1 => img.rotate90(),
        2 => img.rotate180(),
        3 => img.rotate270(),
        _ => img.clone(),
    };
}

pub fn flip_image(img: &mut image::DynamicImage, horizontal: bool, vertical: bool) {
    if horizontal {
        *img = img.fliph();
    }
    if vertical {
        *img = img.flipv();
    }
}

pub fn crop_image(img: &mut image::DynamicImage, x: u32, y: u32, width: u32, height: u32) {
    let (img_width, img_height) = img.dimensions();
    let crop_width = if x + width > img_width {
        img_width - x
    } else {
        width
    };
    let crop_height = if y + height > img_height {
        img_height - y
    } else {
        height
    };
    *img = img.crop_imm(x, y, crop_width, crop_height);
}
