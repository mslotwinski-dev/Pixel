use crate::image::tech;
use crate::window::window::Window;

use eframe::egui::{self, Align, Id};

pub fn resize(ctx: &egui::Context, win: &mut Window) {
    let mut percent_value = ctx.data_mut(|d| {
        d.get_persisted::<u32>(Id::new("resize_percent"))
            .unwrap_or(100)
    });

    let mut width_value = ctx.data_mut(|d| {
        d.get_persisted::<u32>(Id::new("resize_width"))
            .unwrap_or_else(|| win.image.as_ref().map_or(0, |img| img.width()))
    });

    let mut height_value = ctx.data_mut(|d| {
        d.get_persisted::<u32>(Id::new("resize_height"))
            .unwrap_or_else(|| win.image.as_ref().map_or(0, |img| img.height()))
    });

    egui::Window::new("Resize Image")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.add_space(5.0);
            ui.label("Resize options:");

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // --- Scale (%)
            ui.horizontal(|ui| {
                ui.label("Scale (%):");
                if ui
                    .add(egui::DragValue::new(&mut percent_value).range(1..=500))
                    .changed()
                {
                    ctx.data_mut(|d| d.insert_persisted(Id::new("resize_percent"), percent_value));
                }

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Apply").clicked() {
                        if let Some(img) = &mut win.image {
                            tech::resize_image_percentage(img, percent_value);
                            win.update_texture(ctx);
                            win.dialogs.resize = false;
                        }
                    }
                });
            });

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // --- Width & Height
            ui.horizontal(|ui| {
                ui.label("Width:");
                if ui
                    .add(egui::DragValue::new(&mut width_value).range(1..=10000))
                    .changed()
                {
                    ctx.data_mut(|d| d.insert_persisted(Id::new("resize_width"), width_value));
                }

                ui.label("Height:");
                if ui
                    .add(egui::DragValue::new(&mut height_value).range(1..=10000))
                    .changed()
                {
                    ctx.data_mut(|d| d.insert_persisted(Id::new("resize_height"), height_value));
                }

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Apply").clicked() {
                        if let Some(img) = &mut win.image {
                            tech::resize_image(img, width_value, height_value);
                            win.update_texture(ctx);
                            win.dialogs.resize = false;
                        }
                    }
                });
            });

            ui.add_space(5.0);
        });
}
