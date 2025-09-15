use crate::utility::ui::icon_button;
use crate::window::modes::{color, filter, main, shape, tech};
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, win: &mut Window) {
    ui.vertical_centered(|ui| match win.mode {
        1..=4 => {
            if icon_button(
                ui,
                ctx,
                include_bytes!("../../assets/icons/menu.png"),
                "Techniques",
            )
            .clicked()
            {
                win.mode = 0;
            }
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            match win.mode {
                1 => tech::render(ui, ctx, win),
                2 => color::render(ui, ctx, win),
                3 => shape::render(ui, ctx, win),
                4 => filter::render(ui, ctx, win),
                _ => {
                    win.mode = 0;
                }
            }
        }
        _ => {
            main::render(ui, ctx, win);
        }
    });
}
