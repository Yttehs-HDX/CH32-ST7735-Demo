#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::println;
use ch32_hal as hal;

const RAW_IMAGE: &[u8] = include_bytes!("../assets/ferris.raw");

mod lang_items;

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) -> ! {
    hal::debug::SDIPrint::enable();
    let hal_config = hal::Config::default();
    let p = hal::init(hal_config);

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
