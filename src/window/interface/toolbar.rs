use crate::tools::img::dynamic_image_to_color_image;
use crate::tools::open::open_image;
use crate::utility::ui::icon_button_sized;
use crate::window::window::Window;

use eframe::egui::{self, TextureOptions};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, win: &mut Window) {
    ui.add_space(10.0);

    if icon_button_sized(
        ui,
        ctx,
        include_bytes!("../../assets/icons/toolbar/new.png"),
        "Close File",
        25.0,
    )
    .clicked()
    {
        win.input_path = None;
        win.texture = None;
    }

    ui.add_space(10.0);

    if icon_button_sized(
        ui,
        ctx,
        include_bytes!("../../assets/icons/toolbar/open.png"),
        "Open File",
        25.0,
    )
    .clicked()
    {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "gif", "tiff"])
            .set_title("Open Image")
            .pick_file()
        {
            win.input_path = Some(path.display().to_string());
            let img = open_image(&win.input_path.as_ref().unwrap());
            let color_image = dynamic_image_to_color_image(&img);
            win.texture = Some(ctx.load_texture("dyn-img", color_image, TextureOptions::default()));
        }
    }

    ui.add_space(10.0);

    if icon_button_sized(
        ui,
        ctx,
        include_bytes!("../../assets/icons/toolbar/save.png"),
        "Save File",
        25.0,
    )
    .clicked()
    {
        let mut dialog = rfd::FileDialog::new()
            .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "gif", "tiff"])
            .set_title("Save Image");

        if let Some(path) = &win.input_path {
            if let Some(filename) = std::path::Path::new(path).file_name() {
                dialog = dialog.set_file_name(filename.to_string_lossy());
            }
        } else {
            dialog = dialog.set_file_name("output.png");
        }

        if let Some(path) = dialog.save_file() {
            if let Some(img) = &win.image {
                if let Err(err) = img.save(&path) {
                    eprintln!("Failed to save image: {}", err);
                }
            } else {
                eprintln!("No image loaded, nothing to save!");
            }
        }
    }
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    if let Some(path) = &win.input_path {
        ui.label(format!("Loaded image: {}", path));
    }
}
