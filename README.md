# 🎨 Pixel

**Pixel** is a fast and lightweight **image editor** written in **Rust**, available with both a **GUI** and **CLI**.  
It provides powerful yet simple tools for image processing – from basic operations (resize, crop, rotate) to built-in Instagram-style filters.

---

## ✨ Features

- 📂 **GUI** for quick and easy image editing
- 💻 **CLI** with rich set of flags
- 🔄 Image operations:
  - Resize (by percentage or custom dimensions)
  - Rotate (90°, 180°, 270°)
  - Flip (horizontal, vertical, both)
  - Crop (x, y, width, height)
- 🎨 Filters:
  - london, paris, venice, milan, madrid, berlin, oslo, warsaw, new-york, los-angeles, las-vegas, miami, rio, tokio, dubai
- 🎭 Effects:
  - Grayscale, invert, sepia
  - Brightness, contrast, saturation
  - Blur, sharpen, custom filter
- 💾 Output to custom file path (`--output <file>`)

---

## 🚀 Installation

The easiest way:  
Download the installer from [Releases](https://github.com/mslotwinski-dev/Pixel/releases) and install Pixel like a regular program.

Alternatively, build from source:

```bash
git clone https://github.com/mslotwinski-dev/Pixel.git
cd Pixel
cargo build --release
```

---

## ⚡️ Usage

### GUI

```bash
pixel <file>
```

Opens the image in the graphical interface.

### CLI

```bash
pixel <file> [flags] --output result.png
```

Edits the image in CLI with flags and saves to result.png.

---

## ✨ Available options

```bash
pixel --help
```

- `--help, -h` – Show help
- `--version, -v` – Show version information
- `<input>` – Open image in GUI
- `<input> [flags]` – Open image in CLI with flags

#### CLI Flags

- `--resize <percent>` – Resize to percentage
- `--resize <width> <height>` – Resize to given dimensions
- `--rotate <90|180|270>` – Rotate clockwise
- `--flip <horizontal|both|vertical>` – Flip image
- `--crop <x> <y> <width> <height>` – Crop rectangle
- `--grayscale`, `--invert`, `--sepia` – Color effects
- `--brightness <0..200>`, `--contrast <0..200>`, `--saturation <0..200>` – Adjustments
- `--blur <sigma>`, `--sharpen <amount>` – Blur & sharpen
- `--filter <filter>` – Apply filter (e.g. `--filter paris`)
- `--output <file>` – Save output file

## Examples

```bash
# Convert to grayscale and save
pixel image.jpg --grayscale --output grayscale.png

# Rotate and apply a filter
pixel image.png --rotate 90 --filter venice --output rotated.png

# Crop and increase contrast
pixel photo.jpg --crop 10 10 200 200 --contrast 150 --output cropped.jpg
```

## 👨‍💻 Author

Created by mslotwinski-dev
