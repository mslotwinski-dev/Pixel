use crate::image::color;
use crate::window::window::Window;

use eframe::egui::{self, Align, Id};

pub fn brightness(ctx: &egui::Context, win: &mut Window) {
    let mut percent_value = ctx.data_mut(|d| {
        d.get_persisted::<u32>(Id::new("brightness_percent"))
            .unwrap_or(100)
    });

    egui::Window::new("Brightness Adjustment")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Brightness");

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(" X ").clicked() {
                        win.dialogs.brightness = false;
                    }
                });
            });

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Brightness (%):");
                if ui
                    .add(egui::DragValue::new(&mut percent_value).range(0..=200))
                    .changed()
                {
                    ctx.data_mut(|d| {
                        d.insert_persisted(Id::new("brightness_percent"), percent_value)
                    });
                }

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Apply").clicked() {
                        if let Some(img) = &mut win.image {
                            color::adjust_brightness(img, percent_value);
                            win.update_texture(ctx);
                            win.dialogs.brightness = false;
                        }
                    }
                });
            });

            ui.add_space(5.0);
        });
}

pub fn contrast(ctx: &egui::Context, win: &mut Window) {
    let mut percent_value = ctx.data_mut(|d| {
        d.get_persisted::<f32>(Id::new("contrast_percent"))
            .unwrap_or(100.0)
    });

    egui::Window::new("Contrast Adjustment")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Contrast");

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(" X ").clicked() {
                        win.dialogs.contrast = false;
                    }
                });
            });

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Contrast (%):");
                if ui
                    .add(egui::DragValue::new(&mut percent_value).range(0..=200))
                    .changed()
                {
                    ctx.data_mut(|d| {
                        d.insert_persisted(Id::new("contrast_percent"), percent_value)
                    });
                }

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Apply").clicked() {
                        if let Some(img) = &mut win.image {
                            color::adjust_contrast(img, percent_value);
                            win.update_texture(ctx);
                            win.dialogs.contrast = false;
                        }
                    }
                });
            });

            ui.add_space(5.0);
        });
}

pub fn saturation(ctx: &egui::Context, win: &mut Window) {
    let mut percent_value = ctx.data_mut(|d| {
        d.get_persisted::<f32>(Id::new("saturation_percent"))
            .unwrap_or(100.0)
    });

    egui::Window::new("Saturation Adjustment")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Saturation");

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(" X ").clicked() {
                        win.dialogs.saturation = false;
                    }
                });
            });

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Saturation (%):");
                if ui
                    .add(egui::DragValue::new(&mut percent_value).range(0..=200))
                    .changed()
                {
                    ctx.data_mut(|d| {
                        d.insert_persisted(Id::new("saturation_percent"), percent_value)
                    });
                }

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Apply").clicked() {
                        if let Some(img) = &mut win.image {
                            color::adjust_saturation(img, percent_value);
                            win.update_texture(ctx);
                            win.dialogs.saturation = false;
                        }
                    }
                });
            });

            ui.add_space(5.0);
        });
}
