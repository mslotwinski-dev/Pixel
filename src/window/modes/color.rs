use crate::utility::ui::icon_button;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, _win: &mut Window) {
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
        // __win.mode = 1;
    }
    if invert.clicked() {
        // __win.mode = 2;
    }
    if sepia.clicked() {
        // __win.mode = 1;
    }
    if brightness.clicked() {
        // __win.mode = 2;
    }
    if contrast.clicked() {
        // __win.mode = 1;
    }
    if saturation.clicked() {
        // __win.mode = 2;
    }
}
