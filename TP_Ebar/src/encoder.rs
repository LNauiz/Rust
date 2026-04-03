#![no_std]

use embassy_stm32::gpio::Input;
use embedded_hal::Qei;
use crate::bsp_ensea::Encoder_Pins;
pub struct Encoder {
    btn: Input<'static>,
}

impl Encoder {
    // Crée un nouveau bargraph à partir des pins du BSP
    pub fn new(encoder_pin: Encoder_Pins) -> Self {
        let btn = encoder_pin.enc_btn;
        //let coder = Qei::new(encoder_pin);

        let quad1 = encoder_pin.enc_a;
        let quad2 = encoder_pin.enc_b;
        Self {
            btn,
            //coder,
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.btn.is_high()
    }



}