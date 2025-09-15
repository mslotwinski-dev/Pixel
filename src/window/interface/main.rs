use crate::tools::{img::dynamic_image_to_color_image, open::open_image};
use crate::window::window::Window;
use eframe::egui::{self, Align, Color32, FontFamily, FontId, Frame, RichText};

pub fn render(ctx: &egui::Context, win: &mut Window) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.set_max_height(400.0);
            ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
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
                    .inner_margin(egui::Margin::symmetric(20, 10))
                    .corner_radius(5.0)
                    .show(ui, |ui| {
                        ui.set_max_width(200.0);
                        ui.add_space(5.0);

                        if ui
                            .add(
                                egui::Button::new(
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
                                .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "gif", "tiff"])
                                .set_title("Open Image")
                                .pick_file()
                            {
                                win.input_path = Some(path.display().to_string());
                                let img = open_image(win.input_path.as_ref().unwrap());
                                let color_image = dynamic_image_to_color_image(&img);
                                win.texture = Some(ctx.load_texture(
                                    "dyn-img",
                                    color_image,
                                    egui::TextureOptions::default(),
                                ));
                            }
                        }
                    });
            });
        });
    });
}
