#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{
    self, Align, Button, Color32, ColorImage, FontData, FontDefinitions, FontFamily, FontId, Frame,
    Layout, Margin, RichText, TextureHandle, TextureOptions, Vec2, vec2,
};
use image::{self, DynamicImage};
use rfd;

use crate::tools::open::open_image;

pub struct Window {
    input_path: Option<String>,
    texture: Option<TextureHandle>,
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
                texture: Some(texture),
            };
        }

        Self {
            input_path: None,
            texture: None,
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            input_path: None,
            texture: None,
        }
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                Frame::default().inner_margin(10.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                            let img = parse_img(include_bytes!("../assets/icon.png"), ctx)
                                .fit_to_exact_size(vec2(34.0, 34.0));
                            ui.add(img);

                            ui.add_space(12.0);

                            ui.heading(
                                RichText::new("PIXEL")
                                    .color(Color32::from_rgb(0, 180, 130))
                                    .size(32.0)
                                    .font(FontId::new(40.0, FontFamily::Name("pixel_font".into())))
                                    .strong(),
                            );
                        });
                    })
                });
            });
        });

        if self.input_path.is_some() {
            let input_path = self.input_path.clone().unwrap();
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);

                    if ui
                        .add(
                            parse_img(include_bytes!("../assets/icons/new.png"), ctx)
                                .fit_to_exact_size(vec2(25.0, 25.0))
                                .sense(egui::Sense::click()),
                        )
                        .on_hover_text("Close File")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.input_path = None;
                        self.texture = None;
                    }

                    ui.add_space(10.0);

                    if ui
                        .add(
                            parse_img(include_bytes!("../assets/icons/open.png"), ctx)
                                .fit_to_exact_size(vec2(25.0, 25.0))
                                .sense(egui::Sense::click()),
                        )
                        .on_hover_text("Open File")
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "gif", "tiff"])
                            .set_title("Open Image")
                            .pick_file()
                        {
                            self.input_path = Some(path.display().to_string());

                            let img = open_image(&self.input_path.as_ref().unwrap());
                            let color_image = dynamic_image_to_color_image(&img);
                            self.texture = Some(ctx.load_texture(
                                "dyn-img",
                                color_image,
                                TextureOptions::default(),
                            ));
                        }
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    ui.label(format!("Loaded image: {}", input_path));
                });
                ui.separator();

                Frame::default().corner_radius(5.0).show(ui, |ui| {
                    ui.set_max_height(ui.available_height() - 50.0);
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        Frame::default()
                            .fill(Color32::from_rgb(0, 180, 130))
                            .inner_margin(Margin::symmetric(10, 30))
                            .outer_margin(Margin::symmetric(10, 10))
                            .corner_radius(5.0)
                            .show(ui, |ui| {
                                ui.set_width(60.0);
                                ui.set_min_height(ui.available_height());
                                // ui.set_height(60.0);
                                // ui.set_min_height(550.0);

                                ui.vertical_centered(|ui| {
                                    ui.add(
                                        parse_img(include_bytes!("../assets/icons/tech.png"), ctx)
                                            .fit_to_exact_size(vec2(40.0, 40.0)),
                                    );
                                    ui.add(
                                        parse_img(include_bytes!("../assets/icons/color.png"), ctx)
                                            .fit_to_exact_size(vec2(40.0, 40.0)),
                                    );
                                    ui.add(
                                        parse_img(include_bytes!("../assets/icons/shape.png"), ctx)
                                            .fit_to_exact_size(vec2(40.0, 40.0)),
                                    );
                                    ui.add(
                                        parse_img(
                                            include_bytes!("../assets/icons/filter.png"),
                                            ctx,
                                        )
                                        .fit_to_exact_size(vec2(40.0, 40.0)),
                                    );
                                });
                            });
                        ui.separator();
                        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            Frame::default()
                                // .fill(Color32::from_rgb(0, 180, 130))
                                .stroke(egui::Stroke::new(2.0, Color32::from_rgb(0, 180, 130)))
                                .inner_margin(Margin::symmetric(10, 10))
                                .outer_margin(Margin::symmetric(10, 10))
                                .corner_radius(5.0)
                                .show(ui, |ui| {
                                    ui.set_max_width(ui.available_width());
                                    ui.set_min_height(ui.available_height());
                                    if let Some(texture) = &self.texture {
                                        let max_size =
                                            vec2(ui.available_width(), ui.available_height());
                                        let scaled = fit_size(texture.size_vec2(), max_size);

                                        ui.image((texture.id(), scaled));
                                    }
                                });
                        });
                    });
                });
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.set_max_height(400.0);
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.label(
                            RichText::new("Welcome to Pixel!")
                                .color(Color32::from_rgb(0, 180, 130))
                                .font(FontId::new(42.0, FontFamily::Name("pixel_font".into())))
                                .strong(),
                        );

                        ui.label(RichText::new("Select file to edit!").size(20.0).strong());

                        ui.add_space(20.0);

                        Frame::new()
                            .fill(Color32::from_rgb(0, 180, 130))
                            .inner_margin(Margin::symmetric(20, 10))
                            .corner_radius(5.0)
                            .show(ui, |ui| {
                                ui.set_max_width(200.0);
                                ui.add_space(5.0);

                                if ui
                                    .add(
                                        Button::new(
                                            RichText::new("Open File")
                                                .color(Color32::WHITE)
                                                .font(FontId::new(
                                                    24.0,
                                                    FontFamily::Name("pixel_font".into()),
                                                ))
                                                .strong(),
                                        )
                                        .frame(false),
                                    )
                                    .clicked()
                                {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .add_filter(
                                            "Image",
                                            &["png", "jpg", "jpeg", "bmp", "gif", "tiff"],
                                        )
                                        .set_title("Open Image")
                                        .pick_file()
                                    {
                                        self.input_path = Some(path.display().to_string());

                                        let img = open_image(&self.input_path.as_ref().unwrap());
                                        let color_image = dynamic_image_to_color_image(&img);
                                        self.texture = Some(ctx.load_texture(
                                            "dyn-img",
                                            color_image,
                                            TextureOptions::default(),
                                        ));
                                    }
                                }
                            });
                    });
                });
            });
        }

        egui::TopBottomPanel::bottom("bottom_panel")
            // .max_height(44.0)
            .show(ctx, |ui| {
                Frame::default().inner_margin(10.0).show(ui, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(
                            RichText::new("by Mateusz Słotwiński")
                                .color(Color32::from_rgb(0, 180, 130))
                                .font(FontId::new(24.0, FontFamily::Name("pixel_font".into())))
                                .strong(),
                        );

                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            let img = parse_img(include_bytes!("../assets/github.png"), ctx)
                                .fit_to_exact_size(Vec2::splat(22.0))
                                .sense(egui::Sense::click());

                            let response = ui
                                .add(img)
                                .on_hover_text("Open GitHub")
                                .on_hover_cursor(egui::CursorIcon::PointingHand);

                            if response.clicked() {
                                ui.ctx().open_url(egui::OpenUrl::new_tab(
                                    "https://github.com/mslotwinski-dev/Pixel",
                                ));
                            }
                        });
                    });
                });
            });
    }
}

fn parse_img<'a>(bytes: &'a [u8], ctx: &'a egui::Context) -> egui::Image<'a> {
    let image = image::load_from_memory(&bytes).unwrap().to_rgba8();
    let size = [image.width() as _, image.height() as _];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image);
    let texture = ctx.load_texture("icon", color_image, egui::TextureOptions::LINEAR);

    egui::Image::new(&texture).fit_to_original_size(1.0)
}

fn dynamic_image_to_color_image(img: &DynamicImage) -> ColorImage {
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_vec();
    ColorImage::from_rgba_unmultiplied(size, &pixels)
}

// fn add_dynamic_image(ui: &mut Ui, ctx: &egui::Context, img: &DynamicImage) -> TextureHandle {
//     let color_image = dynamic_image_to_color_image(img);
//     let texture = ctx.load_texture("dyn-img", color_image, TextureOptions::default());
//     ui.image((texture.id(), texture.size_vec2()));
//     texture
// }

fn fit_size(original: egui::Vec2, max: egui::Vec2) -> egui::Vec2 {
    let scale = (max.x / original.x).min(max.y / original.y).min(1.0);
    original * scale
}
