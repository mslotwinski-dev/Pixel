use crate::utility::ui::icon_button;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, win: &mut Window) {
    let grayscale = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/tech.png"),
        "Techniques",
    );

    ui.add_space(20.0);

    let invert = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color.png"),
        "Color",
    );

    ui.add_space(20.0);

    let sepia = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/tech.png"),
        "Techniques",
    );

    ui.add_space(20.0);

    let brightness = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color.png"),
        "Color",
    );

    ui.add_space(20.0);

    let contrast = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/tech.png"),
        "Techniques",
    );

    ui.add_space(20.0);

    let saturation = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color.png"),
        "Color",
    );

    if grayscale.clicked() {
        win.dialogs.grayscale = true;
    }
    if invert.clicked() {
        win.dialogs.invert = true;
    }
    if sepia.clicked() {
        win.dialogs.sepia = true;
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
}
