# CH32-ST7735-Demo

Demo for ST7735 under CH32V307VCT6 using ch32-hal

## Description

This project is for CH32V307VCT6 only:

| Board        | Display         | Display size |
|:------------:|:---------------:|:------------:|
| CH32V307VCT6 | ST7735 (8 pins) | 144 * 144    |

Driver the ST7735 to display rust logo (from [sajattack/st7735-lcd-rs](https://github.com/sajattack/st7735-lcd-rs)), with a brightness setting button (user button).

## Usage

1. Install [wlink](https://github.com/ch32-rs/wlink) tool.

2. Connect DuPont line.

On board:

PA14 <---> KEY

From ST7735 to board:

| ST7735 | Board |
|:------:|:-----:|
| GND    | GND   |
| VCC    | 3V3   |
| SCL    | PB3   |
| SDA    | PB5   |
| RES    | PB4   |
| DC     | PD3   |
| CS     | PD4   |
| BLK    | PA15  |

3. Customize config at `src/constant.rs` for your ST7735.

4. Run command:

```bash
cargo run --release
```

5. Press <kbd>USER</kbd> to switch brightness.
