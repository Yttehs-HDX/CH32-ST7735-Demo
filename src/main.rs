#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal::{self as hal};
use constant::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_graphics::{
    image::ImageRaw,
    pixelcolor::{raw::LittleEndian, Rgb565},
    prelude::RgbColor,
};
use hal::{gpio::Pin, println, spi::Spi};
use my_display::MyDisplay;

mod constant;
mod lang_items;
mod my_display;

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    hal::debug::SDIPrint::enable();
    let hal_config = hal::Config::default();
    let p = hal::init(hal_config);

    let (dc, rst, cs) = (p.PD3, p.PB4, p.PD4);
    let (sck, sda) = (p.PB3, p.PB5);
    let spi = Spi::new_blocking_txonly(p.SPI3, sck, sda, Default::default());

    let mut display = MyDisplay::new(
        dc.degrade(),
        rst.degrade(),
        cs.degrade(),
        spi,
        false,
        false,
        ST7735_WIDTH as u32,
        ST7735_HEIGHT as u32,
    );

    let image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(RAW_IMAGE, IMAGE_WIDTH as u32);

    display.init();
    display.set_orientation(st7735_lcd::Orientation::Landscape);
    display.clear(Rgb565::BLACK);
    display.set_offset(
        (ST7735_WIDTH - IMAGE_WIDTH) / 2,
        (ST7735_HEIGHT - IMAGE_HEIGHT) / 2,
    );
    display.draw_image(&image);

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
