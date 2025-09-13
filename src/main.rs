mod cli;
mod image;
mod tools;
mod utility;
mod window;

use crate::cli::start;
use crate::utility::log::Log;
use crate::window::app::run;

use std::env;

fn main() {
    Log::hello();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        Log::error("No arguments provided. Use --help for assistance.");

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
                "--resize <percent>",
                "Resize image to specified percentage of original size",
            );
            Log::flag(
                "--resize <width> <height>",
                "Resize image to specified width and height",
            );
            Log::flag(
                "--rotate <90|180|270>",
                "Rotate image clockwise by specified degrees",
            );
            Log::flag(
                "--flip <horizontal|both|vertical>",
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
            Log::flag("--filter <filter>", "Apply filter to the image");

            println!("Available filters (try them yourself!):");
            Log::flag("- london", "");
            Log::flag("- paris", "");
            Log::flag("- venice", "");
            Log::flag("- milan", "");
            Log::flag("- madrid", "");
            Log::flag("- berlin", "");
            Log::flag("- oslo", "");
            Log::flag("- warsaw", "");
            Log::flag("- new-york", "");
            Log::flag("- los-angeles", "");
            Log::flag("- las-vegas", "");
            Log::flag("- miami", "");
            Log::flag("- rio", "");
            Log::flag("- tokio", "");
            Log::flag("- shanghai", "");
            Log::flag("- dubai", "");
            Log::flag("- cairo", "");
            Log::flag("- lagos", "");

            println!("Additional CLI flags:");

            // Log::flag(
            //     "--batch",
            //     "Apply transformations to all images in folder (input must be a folder)",
            // );
            Log::flag("--output <file>", "Specify output file path");
        }
        "--version" | "-v" => {
            Log::info(&format!("Version: {}", env!("CARGO_PKG_VERSION")));
        }
        input => {
            if args.len() == 2 {
                let _input_path = input.to_string();

                run().unwrap();
            } else {
                let input_path = input.to_string();
                let flags = &args[2..];
                let flags: Vec<&str> = flags.iter().map(|s| s.as_str()).collect();

                start::process_image(&input_path, &flags);
            }
        }
    }
}
