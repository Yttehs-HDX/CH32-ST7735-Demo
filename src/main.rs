#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use backlight_manager::BacklightManager;
use ch32_hal::{self as hal, gpio::Input};
use constant::*;
use display_manager::DisplayManager;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_graphics::{
    image::ImageRaw,
    pixelcolor::{raw::LittleEndian, Rgb565},
    prelude::RgbColor,
};
use hal::{
    gpio::{Pin, Pull},
    println,
    spi::Spi,
    time::Hertz,
    timer::{
        low_level::CountingMode,
        simple_pwm::{PwmPin, SimplePwm},
    },
};

mod backlight_manager;
mod constant;
mod display_manager;
mod lang_items;

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    hal::debug::SDIPrint::enable();
    let hal_config = hal::Config::default();
    let p = hal::init(hal_config);

    let (dc, rst, cs) = (p.PD3, p.PB4, p.PD4);
    let (sck, sda) = (p.PB3, p.PB5);
    let blk = p.PA15;
    let spi = Spi::new_blocking_txonly(p.SPI3, sck, sda, Default::default());

    let mut display = DisplayManager::new(
        dc.degrade(),
        rst.degrade(),
        cs.degrade(),
        spi,
        DISPLAY_RGB,
        DISPLAY_INVERTED,
        DISPLAY_WIDTH as u32,
        DISPLAY_HEIGHT as u32,
    );

    // display must be initialized before initializing the backlight
    display.init();

    let pwm = SimplePwm::new(
        p.TIM2,
        Some(PwmPin::new_ch1::<1>(blk)),
        None,
        None,
        None,
        Hertz::khz(1),
        CountingMode::default(),
    );

    let mut backlight = BacklightManager::new(pwm, ch32_hal::timer::Channel::Ch1);

    backlight.enable();

    let button = Input::new(p.PA0, Pull::Up);

    display.set_orientation(st7735_lcd::Orientation::Landscape);
    display.set_offset(0, 0);
    display.clear(Rgb565::BLACK);
    display.set_offset(
        (DISPLAY_WIDTH - IMAGE_WIDTH) / 2,
        (DISPLAY_HEIGHT - IMAGE_HEIGHT) / 2,
    );
    let image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(RAW_IMAGE, IMAGE_WIDTH as u32);
    display.draw_image(&image);

    loop {
        let (mut flag1, mut flag2) = (false, false);
        if button.is_low() {
            flag1 = true;
        }
        Timer::after_millis(10).await;
        if button.is_high() {
            flag2 = true;
        }

        if flag1 && flag2 {
            let current_bright = backlight.current_brightness();
            backlight.set_brightness(current_bright.next());
            println!("current level: {:?}\r", current_bright.next());
        }
    }
}
