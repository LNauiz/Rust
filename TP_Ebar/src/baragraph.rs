#![no_std]

use embedded_hal::digital::v2::OutputPin;
use embassy_stm32::gpio::Output;
use crate::bsp_ensea::Bargraph_Pins;
use core::marker::PhantomData;
pub struct Bargraph {
    leds: [Output<'static>; 8],  // 8 LEDs pour le bargraph
    min_value: u32,
    max_value: u32,
}

impl Bargraph {
    // Crée un nouveau bargraph à partir des pins du BSP
    pub fn new(bargraph_pins: Bargraph_Pins) -> Self {
        let leds = [
            bargraph_pins.led0,
            bargraph_pins.led1,
            bargraph_pins.led2,
            bargraph_pins.led3,
            bargraph_pins.led4,
            bargraph_pins.led5,
            bargraph_pins.led6,
            bargraph_pins.led7,
        ];
        Self {
            leds,
            min_value: 0,
            max_value: 100,
        }
    }

    // Définit la plage de valeurs (min, max)
    pub fn set_range(&mut self, min: u32, max: u32) {
        self.min_value = min;
        self.max_value = max;
    }

    // Définit la valeur actuelle et allume le nombre de LEDs proportionnel
    pub fn set_value(&mut self, value: u32) {
        let value = value.clamp(self.min_value, self.max_value);

        let num_leds = if self.max_value > self.min_value {
            ((value - self.min_value) as f32 / (self.max_value - self.min_value) as f32 * 8.0) as usize
        } else {
            0
        };

        for (i, led) in self.leds.iter_mut().enumerate() {
            if i < num_leds {
                led.set_high();
            } else {
                led.set_low();
            }
        }
    }
}