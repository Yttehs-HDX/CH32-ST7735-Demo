#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal::{self as hal};
use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::{
    gpio::{Level, Output, Pin},
    println,
    spi::Spi,
};

const RAW_IMAGE: &[u8] = include_bytes!("../assets/ferris.raw");

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

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
