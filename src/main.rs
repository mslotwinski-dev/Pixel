mod utility;
mod window;

use crate::utility::log::Log;
use crate::window::app::run;

use std::env;

fn main() {
    Log::hello();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        Log::info(&format!("Version: {}", env!("CARGO_PKG_VERSION")));
        Log::warn("No arguments provided. Use --help for assistance.");

        run().unwrap();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            println!("Available options:");
            Log::command("--help, -h", "Displays this help message");
            Log::command("--version, -v", "Displays the version information");
            Log::command("<input>", "Opens image at <input> with pixel GUI");
            Log::command("<input> [flags]", "Opens image at <input> with pixel CLI");
            println!();
            println!("Available CLI flags:");
            Log::flag(
                "--resize <width> <height>",
                "Resize image to specified width and height",
            );
            Log::flag(
                "--rotate <90|180|270>",
                "Rotate image clockwise by specified degrees",
            );
            Log::flag(
                "--flip <horizontal|vertical>",
                "Flip image horizontally or vertically",
            );
            Log::flag(
                "--crop <x> <y> <width> <height>",
                "Crop image to specified rectangle",
            );

            Log::flag("--grayscale", "Convert image to grayscale");
            Log::flag("--invert", "Invert image colors");
            Log::flag("--sepia", "Apply sepia tone to image");

            Log::flag("--brightness <amount -100..100>", "Adjust image brightness");
            Log::flag("--contrast <amount -100..100>", "Adjust image contrast");
            Log::flag("--saturation <amount -100..100>", "Adjust image saturation");

            Log::flag("--blur <sigma>", "Apply Gaussian blur to the image");
            Log::flag("--sharpen <amount>", "Sharpen the image (light effect)");

            //
            Log::flag(
                "--format <png|jpg|bmp|gif>",
                "Convert image to specified format",
            );
            Log::flag("--quality <1..100>", "Set output image quality (JPEG only)");
            Log::flag(
                "--batch",
                "Apply transformations to all images in folder (input must be a folder)",
            );
            Log::flag("--output <file>", "Specify output file path");
        }
        _ => Log::error("Unknown argument. Use --help for assistance."),
    }
}
