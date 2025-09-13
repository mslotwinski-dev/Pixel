use crate::image::tech;
use crate::tools::open::open_image;
use crate::tools::save::SaveService;
use crate::utility::log::Log;
use crate::utility::parse::{parse_directions, parse_u32};

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

            // <------------------------ Filter ------------------------>

            // <------------------------ Shape ------------------------>

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
