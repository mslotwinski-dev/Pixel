use std::{env, io};
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("public/icon.ico")
            .set("ProductName", "Pixel")
            .set("FileDescription", "Pixel")
            .set("FileVersion", "1.0.0")
            .set("ProductVersion", "1.0.0")
            .set("CompanyName", "Mateusz Słotwiński")
            .set("LegalCopyright", "Copyright (C) 2025. All rights reserved.")
            .compile()?;
    }
    Ok(())
}
