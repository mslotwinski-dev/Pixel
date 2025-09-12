use crate::image::tech::size;
use crate::tools::open::open_image;
use crate::utility::log::Log;

pub fn process_image(input_path: &str, flags: &[String]) {
    Log::info(&format!("Processing image: {}", input_path));
    Log::info(&format!("With flags: {:?}", flags));

    let mut img = open_image(input_path);

    run_flags(flags, &mut img);

    let output_path = "output.png";
    match img.save(output_path) {
        Ok(_) => Log::info(&format!("Image saved to {}", output_path)),
        Err(e) => Log::error(&format!("Failed to save image: {}", e)),
    }
}

fn run_flags(flags: &[String], img: &mut image::DynamicImage) {
    let mut left_flags = flags;

    match left_flags {
        [flag, percentage] if flag == "--resize" => {
            Log::info(&format!("Resizing image to {}%", percentage));

            let percentage: u32 = percentage.parse().unwrap_or_else(|_| {
                Log::error(
                    "Invalid percentage provided for --resize flag. Image will remain unchanged.",
                );
                0
            });

            size::resize_image_percentage(img, percentage);

            left_flags = &left_flags[2..];
        }

        [flag, width, height] if flag == "--resize" => {
            Log::info(&format!("Resizing image to {}x{}", width, height));
            let width: u32 = width.parse().unwrap_or_else(|_| {
                Log::error(
                    "Invalid width provided for --resize flag. Image will remain unchanged.",
                );
                0
            });
            let height: u32 = height.parse().unwrap_or_else(|_| {
                Log::error(
                    "Invalid height provided for --resize flag. Image will remain unchanged.",
                );
                0
            });

            size::resize_image(img, width, height);

            left_flags = &left_flags[3..];
        }
        _ => {
            Log::error("Wrong flags provided. Image will remain unchanged.");
        }
    }

    if !left_flags.is_empty() {
        run_flags(left_flags, img);
    }
}
