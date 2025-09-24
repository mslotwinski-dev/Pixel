use crate::tools::open::open_image_gui;
use crate::utility::ui::icon_button_sized;
use crate::window::window::Window;

use eframe::egui;

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
        open_image_gui(ctx, win);
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
