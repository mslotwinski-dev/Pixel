use crate::window::interface::{bottom, editor, main, top};
use crate::window::window::Window;
use eframe::egui;

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        top::render(ctx);

        if self.input_path.is_some() {
            editor::render(ctx, self);
        } else {
            main::render(ctx, self);
        }

        bottom::render(ctx);
    }
}
