use image;

pub fn blur_image(img: &mut image::DynamicImage, sigma: f32) {
    if sigma > 0.0 {
        *img = img.blur(sigma);
    }
}

pub fn sharpen_image(img: &mut image::DynamicImage, amount: f32) {
    if amount > 0.0 {
        *img = img.unsharpen(amount, 1);
    }
}
