use eframe::egui::{FontData, FontDefinitions, FontFamily, TextureHandle, TextureOptions};
use image::DynamicImage;

use crate::tools::img::dynamic_image_to_color_image;
use crate::tools::open::open_image;
use crate::window::dialogs::Dialogs;

pub struct Window {
    pub input_path: Option<String>,
    pub mode: u8,
    pub image: Option<DynamicImage>,
    pub original_image: Option<DynamicImage>,
    pub texture: Option<TextureHandle>,

    pub dialogs: Dialogs,
}

impl Window {
    pub fn new(cc: &eframe::CreationContext<'_>, input_path: Option<String>) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "pixel_font".to_owned(),
            FontData::from_static(include_bytes!("../assets/pixel_font.ttf")).into(),
        );

        fonts.families.insert(
            FontFamily::Name("pixel_font".into()),
            vec!["pixel_font".to_owned()],
        );

        cc.egui_ctx.set_fonts(fonts);

        if let Some(input_path) = &input_path {
            let img = open_image(input_path);

            let color_image = dynamic_image_to_color_image(&img);
            let texture =
                cc.egui_ctx
                    .load_texture("dyn-img", color_image, TextureOptions::default());

            return Self {
                input_path: Some(input_path.clone()),
                image: Some(img.clone()),
                original_image: Some(img),
                texture: Some(texture),
                mode: 0,
                dialogs: Dialogs::default(),
            };
        }

        Self {
            input_path: None,
            image: None,
            original_image: None,
            texture: None,
            mode: 0,
            dialogs: Dialogs::default(),
        }
    }

    pub fn update_texture(&mut self, ctx: &eframe::egui::Context) {
        if let Some(img) = &self.image {
            let color_image = dynamic_image_to_color_image(img);
            self.texture =
                Some(ctx.load_texture("dyn-img", color_image, TextureOptions::default()));
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            input_path: None,
            image: None,
            original_image: None,
            texture: None,
            mode: 0,
            dialogs: Dialogs::default(),
        }
    }
}
