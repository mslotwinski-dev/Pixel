use crate::window::interface::{image, side_menu, toolbar};
use crate::window::window::Window;

use eframe::egui::{self, Color32, Frame, Layout, Margin};

pub fn render(ctx: &egui::Context, win: &mut Window) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| toolbar::render(ui, ctx, win));
        ui.separator();

        Frame::default().corner_radius(5.0).show(ui, |ui| {
            ui.set_max_height(ui.available_height() - 50.0);
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                Frame::default()
                    .fill(Color32::from_rgb(0, 180, 130))
                    .inner_margin(Margin::symmetric(10, 30))
                    .outer_margin(Margin::symmetric(10, 10))
                    .corner_radius(5.0)
                    .show(ui, |ui| {
                        ui.set_width(60.0);
                        ui.set_min_height(ui.available_height());
                        ui.set_style(egui::Style {
                            visuals: egui::Visuals::light(),
                            ..egui::Style::default()
                        });
                        side_menu::render(ui, ctx, win);
                    });

                ui.separator();
                ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                    image::render(ui, win)
                });
            });
        });
    });
}
