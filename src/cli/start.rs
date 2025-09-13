use crate::image::{color, shape, tech};
use crate::tools::open::open_image;
use crate::tools::save::SaveService;
use crate::utility::log::Log;
use crate::utility::parse::{parse_directions, parse_f32, parse_i32, parse_u32};

pub fn process_image(input_path: &str, flags: &[&str]) {
    Log::info(&format!("Processing image: {}", input_path));
    Log::info(&format!("With flags: {:?}", flags));

    let mut img = open_image(input_path);
    let mut save_service = SaveService::new(input_path);

    run_flags(flags, &mut img, &mut save_service);

    save_service.save(&img);
}

fn run_flags(mut flags: &[&str], img: &mut image::DynamicImage, save_service: &mut SaveService) {
    while !flags.is_empty() {
        match flags {
            // <------------------------ Technical ------------------------>
            ["--resize", width, height, rest @ ..] if !height.starts_with('-') => {
                Log::info(&format!("Resizing image to {}x{}", width, height));

                if let (Some(w), Some(h)) =
                    (parse_u32(width, "--resize"), parse_u32(height, "--resize"))
                {
                    tech::resize_image(img, w, h);
                }

                flags = rest;
            }

            ["--resize", percentage, rest @ ..] => {
                Log::info(&format!("Resizing image to {}%", percentage));

                if let Some(p) = parse_u32(percentage, "--resize") {
                    tech::resize_image_percentage(img, p);
                }

                flags = rest;
            }

            ["--rotate", degrees, rest @ ..] => {
                Log::info(&format!("Rotating image by {} degrees", degrees));
                if let Some(p) = parse_u32(degrees, "--rotate") {
                    if p % 90 != 0 {
                        Log::warn(
                            "Rotation must be a multiple of 90. Image will remain unchanged.",
                        );
                    } else {
                        tech::rotate_image(img, p);
                    }
                }

                flags = rest;
            }

            ["--flip", direction, rest @ ..] => {
                Log::info(&format!("Flipping image in {} direction", direction));

                if let Some((horizontal, vertical)) = parse_directions(direction) {
                    tech::flip_image(img, horizontal, vertical);
                }

                flags = rest;
            }

            ["--crop", x, y, width, height, rest @ ..] if !height.starts_with('-') => {
                Log::info(&format!(
                    "Cropping image at ({}, {}) with size {}x{}",
                    x, y, width, height
                ));

                if let (Some(x), Some(y), Some(w), Some(h)) = (
                    parse_u32(x, "--crop"),
                    parse_u32(y, "--crop"),
                    parse_u32(width, "--crop"),
                    parse_u32(height, "--crop"),
                ) {
                    tech::crop_image(img, x, y, w, h);
                }

                flags = rest;
            }

            // <------------------------ Color ------------------------>
            ["--grayscale", rest @ ..] => {
                Log::info("Converting image to grayscale");
                color::grayscale_image(img);
                flags = rest;
            }

            ["--invert", rest @ ..] => {
                Log::info("Inverting image colors");
                color::invert_image(img);
                flags = rest;
            }

            ["--sepia", rest @ ..] => {
                Log::info("Applying sepia filter to image");
                color::sepia_image(img);
                flags = rest;
            }

            ["--brightness", value, rest @ ..] => {
                Log::info(&format!("Brightening image by {}", value));

                if let Some(v) = parse_i32(value, "--brightness") {
                    color::adjust_brightness(img, v);
                }

                flags = rest;
            }

            ["--contrast", value, rest @ ..] => {
                Log::info(&format!("Adjusting image contrast by {}", value));

                if let Some(v) = parse_f32(value, "--contrast") {
                    color::adjust_contrast(img, v);
                }

                flags = rest;
            }

            ["--saturation", value, rest @ ..] => {
                Log::info(&format!("Adjusting image saturation by {}", value));

                if let Some(v) = parse_f32(value, "--saturation") {
                    color::adjust_saturation(img, v);
                }

                flags = rest;
            }

            // <------------------------ Shape ------------------------>
            ["--blur", sigma, rest @ ..] => {
                Log::info(&format!("Applying Gaussian blur with sigma {}", sigma));

                if let Some(s) = parse_f32(sigma, "--blur") {
                    shape::blur_image(img, s);
                }

                flags = rest;
            }

            ["--sharpen", sigma, rest @ ..] => {
                Log::info(&format!("Applying Gaussian sharpen with sigma {}", sigma));

                if let Some(s) = parse_f32(sigma, "--sharpen") {
                    shape::sharpen_image(img, s);
                }

                flags = rest;
            }

            // <------------------------ Filter ------------------------>

            // <------------------------ Data ------------------------>
            ["--output", direction, rest @ ..] => {
                Log::info(&format!("Setting output path to {}", direction));
                save_service.output = Some(direction.to_string());
                flags = rest;
            }

            // <------------------------ Errors ------------------------>
            [arg, rest @ ..] => {
                Log::warn(&format!("Unknown flag provided: {}. Skipping.", arg));

                flags = rest;
            }

            // <------------------------ Break ------------------------>
            [] => break,
        }
    }
}
