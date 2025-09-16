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
        win.dialogs.resize = !win.dialogs.resize;
    }

    if rotate.clicked() {
        win.dialogs.rotate = !win.dialogs.rotate;
    }

    if flip.clicked() {
        win.dialogs.flip = !win.dialogs.flip;
    }

    if crop.clicked() {
        win.dialogs.crop = !win.dialogs.crop;
    }

    // Dialogs

    if win.dialogs.resize {
        dialogs::tech::resize(ctx, win);
    }

    if win.dialogs.rotate {
        dialogs::tech::rotate(ctx, win);
    }

    if win.dialogs.flip {
        dialogs::tech::flip(ctx, win);
    }
}
