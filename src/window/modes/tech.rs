use crate::utility::ui::icon_button;
use crate::window::dialogs;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, win: &mut Window) {
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
        win.dialogs.resize = true;
    }

    if win.dialogs.resize {
        dialogs::tech::resize(ctx, win);
    }

    if rotate.clicked() {
        win.dialogs.rotate = true;
    }
    if flip.clicked() {
        win.dialogs.flip = true;
    }
    if crop.clicked() {
        win.dialogs.crop = true;
    }
}
