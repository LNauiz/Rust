#![no_std]

use embassy_stm32::timer::qei::Qei;
use embassy_stm32::pac::TIM2;
use embassy_stm32::gpio::Input;
use embedded_hal::digital::v2::InputPin;
use crate::bsp_ensea::Encoder_Pins;

pub struct Encoder<'d> {
    button: Input<'d>,
    qei : Qei<'static, embassy_stm32::peripherals::TIM2>,
}

impl<'d> Encoder<'d> {
    pub fn new(encoder_pins: Encoder_Pins) -> Self {
        //let qei: Qei<'d, TimGp32>;
        let button = encoder_pins.enc_btn;
        let qei = encoder_pins.enc_hw;
        Self {
            button,
            qei
        }
    }

    pub fn get_position(&self) -> u16 {
        self.qei.count()
    }

    pub fn set_position(&mut self, position: i32) {
        unsafe {
            let tim2 = TIM2;
            tim2.cnt().write_value(position as u32);
        }
    }

    pub fn reset(&mut self) {
        self.set_position(0);
    }

    pub fn is_pressed(&self) -> bool {
        self.button.is_high()
    }
}