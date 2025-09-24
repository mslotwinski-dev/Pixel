use crate::window::window::Window;
use eframe::egui::{self, Color32, Frame, Margin, Pos2, Rect, Sense};

pub fn render(ctx: &egui::Context, ui: &mut egui::Ui, win: &mut Window) {
    Frame::default()
        .stroke(egui::Stroke::new(2.0, Color32::from_rgb(0, 180, 130)))
        .inner_margin(Margin::ZERO)
        .outer_margin(Margin::symmetric(10, 10))
        .corner_radius(5.0)
        .show(ui, |ui| {
            if let Some(texture) = &win.texture {
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

                if response.dragged() {
                    win.offset += response.drag_delta();
                    ctx.request_repaint();
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
                } else if response.hovered() {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Default);
                }

                if response.hovered() {
                    let scroll = ui.input(|i| i.raw_scroll_delta.y);
                    if scroll != 0.0 {
                        let zoom_delta = scroll * win.zoom * 0.001;
                        win.zoom = (win.zoom + zoom_delta).max(0.25);
                    }
                    ctx.request_repaint();
                }

                let image_size = texture.size_vec2() * win.zoom;
                let image_center = response.rect.center() + win.offset;
                let image_top_left = image_center - image_size / 2.0;
                let image_rect = Rect::from_min_size(image_top_left, image_size);

                painter.with_clip_rect(response.rect).image(
                    texture.id(),
                    image_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Brak obrazu");
                });
            }
        });
}
