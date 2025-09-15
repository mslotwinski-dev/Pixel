use crate::utility::ui::icon_button;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, win: &mut Window) {
    let tech = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/tech.png"),
        "Techniques",
    );

    ui.add_space(20.0);

    let color = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/color.png"),
        "Color",
    );

    ui.add_space(20.0);

    let shape = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/shape.png"),
        "Shape",
    );

    ui.add_space(20.0);

    let filter = icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/filter.png"),
        "Filter",
    );

    if tech.clicked() {
        win.mode = 1;
    }
    if color.clicked() {
        win.mode = 2;
    }
    if shape.clicked() {
        win.mode = 3;
    }
    if filter.clicked() {
        win.mode = 4;
    }

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(20.0);

    if icon_button(
        ui,
        ctx,
        include_bytes!("../../assets/icons/clean.png"),
        "Clean",
    )
    .clicked()
    {
        win.show_reset_dialog = true;
    }

    if win.show_reset_dialog {
        egui::Window::new("Reset Image")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label("Are you sure you want to reset the image to its original state?");
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    if ui.button("Yes").clicked() {
                        if let Some(orig) = &win.original_image {
                            win.image = Some(orig.clone());
                            win.update_texture(ctx);
                        }
                        win.show_reset_dialog = false;
                    }
                    ui.add_space(10.0);

                    if ui.button("No").clicked() {
                        win.show_reset_dialog = false;
                    }
                });
            });
    }
}
