use image;
use photon_rs::PhotonImage;
use photon_rs::channels;
use photon_rs::effects;
use photon_rs::filters;
use photon_rs::monochrome;
use photon_rs::multiple;

use crate::utility::log::Log;

pub fn apply_filter(img: &mut image::DynamicImage, filter: &str) {
    match filter.to_lowercase().as_str() {
        "london" => london(img),
        "paris" => paris(img),
        "venice" => venice(img),
        "milan" => milan(img),
        "madrid" => madrid(img),
        "berlin" => berlin(img),
        "oslo" => oslo(img),
        "warsaw" => warsaw(img),
        "new-york" => new_york(img),
        "los-angeles" => los_angeles(img),
        "las-vegas" => las_vegas(img),
        "miami" => miami(img),
        "rio" => rio(img),
        "tokio" => tokio(img),
        // "shanghai" => shanghai(img),
        "dubai" => dubai(img),
        // "cairo" => cairo(img),
        // "lagos" => lagos(img),
        _ => {
            Log::warn(&format!(
                "Filter '{}' not recognized. No filter applied.",
                filter
            ));
        }
    }
}

fn to_photon(img: &mut image::DynamicImage) -> PhotonImage {
    let buf = img.to_rgba8();
    PhotonImage::new(buf.into_raw(), img.width(), img.height())
}

fn from_photon(pimg: PhotonImage) -> image::DynamicImage {
    let raw =
        image::RgbaImage::from_raw(pimg.get_width(), pimg.get_height(), pimg.get_raw_pixels())
            .expect("Błąd konwersji");
    image::DynamicImage::ImageRgba8(raw)
}

// OK
pub fn london(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "vintage");
    *img = from_photon(pimg);
}

// OK
pub fn paris(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "diamante");
    *img = from_photon(pimg);
}

pub fn venice(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    effects::inc_brightness(&mut pimg, 20);
    *img = from_photon(pimg);
}

// OK
pub fn milan(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "firenze");
    *img = from_photon(pimg);
}

// OK
pub fn madrid(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "golden");
    *img = from_photon(pimg);
}

pub fn berlin(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    monochrome::grayscale(&mut pimg);
    channels::alter_channel(&mut pimg, 2, 30);
    *img = from_photon(pimg);
}

pub fn oslo(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "oceanic");
    *img = from_photon(pimg);
}

// OK
pub fn warsaw(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    // filters::filter(&mut pimg, "oceanic");
    filters::filter(&mut pimg, "marine");

    *img = from_photon(pimg);
}

// OK
pub fn new_york(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    effects::offset_red(&mut pimg, 20);
    *img = from_photon(pimg);
}

pub fn los_angeles(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "islands");
    *img = from_photon(pimg);
}

pub fn las_vegas(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "neon");
    *img = from_photon(pimg);
}

// OK
pub fn miami(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "lofi");

    *img = from_photon(pimg);
}

pub fn rio(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "seagreen");
    *img = from_photon(pimg);
}

pub fn tokio(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "pastel_pink");
    *img = from_photon(pimg);
}

pub fn dubai(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "twenties");
    *img = from_photon(pimg);
}

#[allow(dead_code)]
pub fn shanghai(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    multiple::apply_gradient(&mut pimg);
    filters::filter(&mut pimg, "obsidian");
    *img = from_photon(pimg);
}

#[allow(dead_code)]
pub fn cairo(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "radui");
    *img = from_photon(pimg);
}

#[allow(dead_code)]
pub fn lagos(img: &mut image::DynamicImage) {
    let mut pimg = to_photon(img);
    filters::filter(&mut pimg, "bright");
    *img = from_photon(pimg);
}
