use crate::image::color;
use crate::utility::ui::icon_button;
use crate::window::dialogs;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, win: &mut Window) {
    let grayscale = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color/grayscale.png"),
        "Grayscale",
    );

    ui.add_space(20.0);

    let invert = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color/invert.png"),
        "Invert",
    );

    ui.add_space(20.0);

    let sepia = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color/sepia.png"),
        "Sepia",
    );

    ui.add_space(20.0);

    let brightness = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color/brightness.png"),
        "Brightness",
    );

    ui.add_space(20.0);

    let contrast = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color/contrast.png"),
        "Contrast",
    );

    ui.add_space(20.0);

    let saturation = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color/saturation.png"),
        "Saturation",
    );

    if grayscale.clicked() {
        if let Some(img) = &mut win.image {
            color::grayscale_image(img);
            win.update_texture(ctx);
        }
    }

    if invert.clicked() {
        if let Some(img) = &mut win.image {
            color::invert_image(img);
            win.update_texture(ctx);
        }
    }

    if sepia.clicked() {
        if let Some(img) = &mut win.image {
            color::sepia_image(img);
            win.update_texture(ctx);
        }
    }

    if brightness.clicked() {
        win.dialogs.brightness = true;
    }

    if contrast.clicked() {
        win.dialogs.contrast = true;
    }

    if saturation.clicked() {
        win.dialogs.saturation = true;
    }

    if win.dialogs.brightness {
        dialogs::color::brightness(ctx, win);
    }

    if win.dialogs.contrast {
        dialogs::color::contrast(ctx, win);
    }

    if win.dialogs.saturation {
        dialogs::color::saturation(ctx, win);
    }
}
