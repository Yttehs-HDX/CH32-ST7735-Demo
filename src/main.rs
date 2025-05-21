#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal::{self as hal};
use constant::*;
use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use embedded_graphics::{
    draw_target::DrawTarget,
    image::ImageDrawable,
    pixelcolor::{raw::LittleEndian, Rgb565},
    prelude::RgbColor,
};
use hal::{
    gpio::{Level, Output, Pin},
    println,
    spi::Spi,
};

mod constant;
mod lang_items;

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    hal::debug::SDIPrint::enable();
    let hal_config = hal::Config::default();
    let p = hal::init(hal_config);

    let dc = Output::new(p.PD3.degrade(), Level::Low, Default::default());
    let res = Output::new(p.PB4.degrade(), Level::Low, Default::default());
    let cs = Output::new(p.PD4.degrade(), Level::Low, Default::default());

    let (sck, sda) = (p.PB3, p.PB5);
    let spi = Spi::new_blocking_txonly(p.SPI3, sck, sda, Default::default());
    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let mut display = st7735_lcd::ST7735::new(
        spi_device,
        dc,
        res,
        false,
        false,
        ST7735_WIDTH as u32,
        ST7735_HEIGHT as u32,
    );

    display.init(&mut Delay).unwrap();
    display
        .set_orientation(&st7735_lcd::Orientation::Landscape)
        .unwrap();
    display.clear(Rgb565::BLACK).unwrap();
    let xpos = (ST7735_WIDTH - IMAGE_WIDTH) / 2;
    let ypos = (ST7735_HEIGHT - IMAGE_HEIGHT) / 2;
    let image: embedded_graphics::image::ImageRaw<Rgb565, LittleEndian> =
        embedded_graphics::image::ImageRaw::new(RAW_IMAGE, IMAGE_WIDTH as u32);

    display.set_offset(xpos, ypos);
    image.draw(&mut display).unwrap();

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
