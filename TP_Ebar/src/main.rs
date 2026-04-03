#![no_main]
#![no_std]

pub mod bsp_ensea;
pub mod baragraph;
pub mod gamepad;
pub mod  encoder;

use defmt::export::u32;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _};

use bsp_ensea::Board;
use baragraph::Bargraph;
use gamepad::{Gamepad,Button};
use encoder::Encoder;
use crate::bsp_ensea::Gamepad_Pins;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let mut bargraph = Bargraph::new(board.bargraph_pins);
    let mut gamepad = Gamepad::new(board.gamepad_pins);
    let mut encoder = Encoder::new(board.encoder_pins);
    bargraph.set_range(0, 100);
    bargraph.set_value(0);
    let mut value:u32 = 0;
    let mut flag:bool = false;
    loop {
        embassy_time::Timer::after_millis(10).await;
        if encoder.is_pressed() && !flag{
            value = value + 10;
            flag = true;
        }

        if !encoder.is_pressed() {
            flag = false;
        }
        if value>100{
            value=0;
        }
        bargraph.set_value(value);
    }
}
