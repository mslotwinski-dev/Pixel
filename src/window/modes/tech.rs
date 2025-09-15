use crate::utility::ui::icon_button;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, _win: &mut Window) {
    let resize = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/tech/resize.png"),
        "Resize",
    );

    ui.add_space(20.0);

    let rotate = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/tech/rotate.png"),
        "Rotate",
    );

    ui.add_space(20.0);

    let flip = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/tech/flip.png"),
        "Flip",
    );

    ui.add_space(20.0);

    let crop = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/tech/crop.png"),
        "Crop",
    );

    if resize.clicked() {
        // win.mode = 1;
    }
    if rotate.clicked() {
        // win.mode = 2;
    }
    if flip.clicked() {
        // win.mode = 3;
    }
    if crop.clicked() {
        // win.mode = 4;
    }
}
