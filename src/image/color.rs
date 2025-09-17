use image;
use image::GenericImage;
use image::GenericImageView;

pub fn grayscale_image(img: &mut image::DynamicImage) {
    *img = img.grayscale();
}

pub fn invert_image(img: &mut image::DynamicImage) {
    img.invert();
}

pub fn sepia_image(img: &mut image::DynamicImage) {
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            let tr = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
            let tg = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
            let tb = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;

            img.put_pixel(x, y, image::Rgba([tr, tg, tb, pixel[3]]));
        }
    }
}

pub fn adjust_brightness(img: &mut image::DynamicImage, value: u32) {
    *img = img.brighten(value as i32 - 100);
}

pub fn adjust_contrast(img: &mut image::DynamicImage, value: f32) {
    *img = img.adjust_contrast(value - 100.0);
}

pub fn adjust_saturation(img: &mut image::DynamicImage, value: f32) {
    let adjustment = value / 100.0;

    let mut buffer = img.to_rgba32f();

    for pixel in buffer.pixels_mut() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let l = (max + min) / 2.0;

        if max == min {
            continue;
        }

        let mut s = if l > 0.5 {
            (max - min) / (2.0 - max - min)
        } else {
            (max - min) / (max + min)
        };

        let mut h = if max == r {
            (g - b) / (max - min)
        } else if max == g {
            2.0 + (b - r) / (max - min)
        } else {
            4.0 + (r - g) / (max - min)
        };
        h /= 6.0;
        if h < 0.0 {
            h += 1.0;
        }

        s *= adjustment;
        s = s.clamp(0.0, 1.0);

        let (new_r, new_g, new_b) = if s == 0.0 {
            (l, l, l)
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            (
                hue_to_rgb(p, q, h + 1.0 / 3.0),
                hue_to_rgb(p, q, h),
                hue_to_rgb(p, q, h - 1.0 / 3.0),
            )
        };

        *pixel = image::Rgba([new_r, new_g, new_b, pixel[3]]);
    }

    *img = image::DynamicImage::ImageRgba32F(buffer);
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}
