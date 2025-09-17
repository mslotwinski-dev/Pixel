use crate::image::filter;
use crate::utility::ui::filter_button;
use crate::window::window::Window;

use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, ctx: &egui::Context, win: &mut Window) {
    let london = filter_button(ui, ctx, "London");
    let paris = filter_button(ui, ctx, "Paris");
    let venice = filter_button(ui, ctx, "Venice");
    let milan = filter_button(ui, ctx, "Milan");
    let madrid = filter_button(ui, ctx, "Madrid");
    let berlin = filter_button(ui, ctx, "Berlin");
    let oslo = filter_button(ui, ctx, "Oslo");
    let warsaw = filter_button(ui, ctx, "Warsaw");
    let new_york = filter_button(ui, ctx, "New York");
    let los_angeles = filter_button(ui, ctx, "Los Angeles");
    let las_vegas = filter_button(ui, ctx, "Las Vegas");
    let miami = filter_button(ui, ctx, "Miami");
    let rio = filter_button(ui, ctx, "Rio");
    let tokio = filter_button(ui, ctx, "Tokyo");
    let dubai = filter_button(ui, ctx, "Dubai");

    if london.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "london");
            win.update_texture(ctx);
        }
    }

    if paris.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "paris");
            win.update_texture(ctx);
        }
    }

    if venice.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "venice");
            win.update_texture(ctx);
        }
    }

    if milan.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "milan");
            win.update_texture(ctx);
        }
    }

    if madrid.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "madrid");
            win.update_texture(ctx);
        }
    }

    if berlin.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "berlin");
            win.update_texture(ctx);
        }
    }

    if oslo.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "oslo");
            win.update_texture(ctx);
        }
    }

    if warsaw.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "warsaw");
            win.update_texture(ctx);
        }
    }

    if new_york.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "new-york");
            win.update_texture(ctx);
        }
    }

    if los_angeles.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "los-angeles");
            win.update_texture(ctx);
        }
    }

    if las_vegas.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "las-vegas");
            win.update_texture(ctx);
        }
    }

    if miami.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "miami");
            win.update_texture(ctx);
        }
    }

    if rio.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "rio");
            win.update_texture(ctx);
        }
    }

    if tokio.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "tokio");
            win.update_texture(ctx);
        }
    }

    if dubai.clicked() {
        if let Some(img) = &mut win.image {
            filter::apply_filter(img, "dubai");
            win.update_texture(ctx);
        }
    }
}
