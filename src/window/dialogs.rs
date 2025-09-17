pub mod color;
pub mod shape;
pub mod tech;

pub struct Dialogs {
    pub reset: bool,

    pub resize: bool,
    pub rotate: bool,
    pub flip: bool,
    pub crop: bool,

    pub brightness: bool,
    pub contrast: bool,
    pub saturation: bool,

    pub blur: bool,
    pub sharpen: bool,
}

impl Dialogs {
    pub fn default() -> Self {
        Self {
            reset: false,

            resize: false,
            rotate: false,
            flip: false,
            crop: false,

            brightness: false,
            contrast: false,
            saturation: false,

            blur: false,
            sharpen: false,
        }
    }
}
