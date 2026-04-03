#![no_main]
#![no_std]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let mut led_green = Output::new(p.PA5, Level::Low, Speed::Low); // LD2

    info!("Hardware initialized, starting main loop...");

    loop {
        led_green.toggle();
        info!("Led toggled!");
        Timer::after_millis(1_000).await;
    }
}
