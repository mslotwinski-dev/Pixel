use crate::image::shape;
use crate::window::window::Window;

use eframe::egui::{self, Align, Id};

pub fn blur(ctx: &egui::Context, win: &mut Window) {
    let mut amount = ctx.data_mut(|d| {
        d.get_persisted::<f32>(Id::new("blur_amount"))
            .unwrap_or(1.0)
    });

    egui::Window::new("Blur Adjustment")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Blur");

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(" X ").clicked() {
                        win.dialogs.blur = false;
                    }
                });
            });

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Amount (sigma):");
                if ui
                    .add(egui::DragValue::new(&mut amount).range(0.0..=10.0))
                    .changed()
                {
                    ctx.data_mut(|d| d.insert_persisted(Id::new("blur_amount"), amount));
                }

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Apply").clicked() {
                        if let Some(img) = &mut win.image {
                            shape::blur_image(img, amount);
                            win.update_texture(ctx);
                            win.dialogs.blur = false;
                        }
                    }
                });
            });

            ui.add_space(5.0);
        });
}

pub fn sharpen(ctx: &egui::Context, win: &mut Window) {
    let mut amount = ctx.data_mut(|d| {
        d.get_persisted::<f32>(Id::new("sharpen_amount"))
            .unwrap_or(1.0)
    });

    egui::Window::new("Sharpening Adjustment")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Sharpen");

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(" X ").clicked() {
                        win.dialogs.sharpen = false;
                    }
                });
            });

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Amount (sigma):");
                if ui
                    .add(egui::DragValue::new(&mut amount).range(0.0..=10.0))
                    .changed()
                {
                    ctx.data_mut(|d| d.insert_persisted(Id::new("sharpen_amount"), amount));
                }

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Apply").clicked() {
                        if let Some(img) = &mut win.image {
                            shape::sharpen_image(img, amount);
                            win.update_texture(ctx);
                            win.dialogs.sharpen = false;
                        }
                    }
                });
            });

            ui.add_space(5.0);
        });
}
