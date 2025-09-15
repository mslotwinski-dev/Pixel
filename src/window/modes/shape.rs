use crate::utility::ui::icon_button;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, _win: &mut Window) {
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
        // win.mode = 1;
    }
    if sharpen.clicked() {
        // win.mode = 2;
    }
}
