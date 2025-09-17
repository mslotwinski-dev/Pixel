use crate::utility::ui::icon_button;
use crate::window::dialogs;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, win: &mut Window) {
    let blur = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/shape/blur.png"),
        "Blur",
    );

    ui.add_space(20.0);

    let sharpen = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/shape/sharpen.png"),
        "Sharpen",
    );

    if blur.clicked() {
        win.dialogs.blur = true
    }

    if sharpen.clicked() {
        win.dialogs.sharpen = true;
    }

    if win.dialogs.blur {
        dialogs::shape::blur(ctx, win);
    }

    if win.dialogs.sharpen {
        dialogs::shape::sharpen(ctx, win);
    }
}
