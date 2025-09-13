use crate::utility::log::Log;

#[derive(Debug, Clone)]
pub struct SaveService {
    pub input: String,
    pub output: Option<String>,
}

impl SaveService {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            output: None,
        }
    }

    pub fn save(&mut self, img: &image::DynamicImage) {
        if let Some(ref output) = self.output {
            self.save_image(img, output);
        } else if self.ask_overwrite() {
            self.output = Some(self.input.clone());
            self.save_image(img, &self.input);
        } else if let Some(path) = self.ask_for_output_path() {
            self.output = Some(path.clone());
            self.save_image(img, &path);
        } else {
            Log::warn("Image not saved.");
        }
    }

    fn save_image(&self, img: &image::DynamicImage, path: &str) {
        match img.save(path) {
            Ok(_) => Log::info(&format!("Image saved to {}", path)),
            Err(e) => Log::error(&format!("Failed to save image: {}", e)),
        }
    }

    fn ask_overwrite(&self) -> bool {
        Log::warn("No output path specified. Do you want to overwrite the original image? (y/N)");
        let mut response = String::new();
        std::io::stdin().read_line(&mut response).is_ok()
            && response.trim().eq_ignore_ascii_case("y")
    }

    fn ask_for_output_path(&self) -> Option<String> {
        Log::info("Provide output path:");
        let mut response = String::new();
        if std::io::stdin().read_line(&mut response).is_ok() {
            let trimmed = response.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
        None
    }
}
