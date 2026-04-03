
#![no_std]

use embassy_stm32::gpio::{AnyPin,Input,Output,Level,Speed,Pull};
use embassy_stm32::Peripherals;
use embassy_stm32::timer::qei;
use embassy_stm32::pac::TIM2;

use embassy_stm32::Peri;

pub struct Board{
    pub bargraph_pins: Bargraph_Pins,
    //pub gps_pins : Gps_Pins,
    //pub gpio_pins : Gpio_Pins,
    pub gamepad_pins: Gamepad_Pins,
    //pub magnetometer_pins: Magnetometer_Pins,
    pub encoder_pins: Encoder_Pins,
    //pub stepper_pins: Stepper_Pins,
    //pub usart1_pins: Usart1_Pins,
    //pub usart2_pins: Usart2_Pins,
    //pub spi2_pins: Spi2_Pins,
    //pub i2c1_pins: I2c1_Pins,
    //pub free_pins: Free_Pins

}

impl Board {
    // Méthode `new` qui prend les périphériques Embassy en paramètre
    pub fn new(p: Peripherals) -> Self {
        let bargraph_pins = Bargraph_Pins {
            led0: Output::new(p.PC7, Level::Low, Speed::Low),
            led1: Output::new(p.PB2, Level::Low, Speed::Low),
            led2: Output::new(p.PA8, Level::Low, Speed::Low),
            led3: Output::new(p.PB1, Level::Low, Speed::Low),
            led4: Output::new(p.PB15, Level::Low, Speed::Low),
            led5: Output::new(p.PB4, Level::Low, Speed::Low),
            led6: Output::new(p.PB14, Level::Low, Speed::Low),
            led7: Output::new(p.PB5, Level::Low, Speed::Low),
        };

        let gamepad_pins = Gamepad_Pins{
            btn_top: Input::new(p.PC8, Pull::Up),
            btn_bottom: Input::new(p.PB11, Pull::Up),
            btn_right: Input::new(p.PC9, Pull::Up),
            btn_left: Input::new(p.PC6, Pull::Up),
            btn_center: Input::new(p.PC5, Pull::Up),
        };

        let encoder_pins = Encoder_Pins{
            enc_btn: Input::new(p.PA15, Pull::Up),
            enc_a: Input::new(p.PA0, Pull::Up),
            enc_b: Input::new(p.PA1, Pull::Up),
        };

        Self { bargraph_pins , gamepad_pins, encoder_pins}
    }
}

pub struct Bargraph_Pins {
    pub led7: Output<'static>, //Peri<'static, Output>, //PB5
    pub led6: Output<'static>, //PB14
    pub led5: Output<'static>, //PB4
    pub led4: Output<'static>, //PB15
    pub led3: Output<'static>, //PB1
    pub led2: Output<'static>, //PA8
    pub led1: Output<'static>, //PB2
    pub led0: Output<'static>, //PC7
}

struct Stepper_Pins {
    pub direction: Output<'static>, // PA7
    pub ms1: Output<'static>, // PA11
    pub ms2: Output<'static>, // PB12
    pub enn: Output<'static>, // PA12
    pub step: Output<'static>, // PA6
}

pub struct Gps_Pins {
    pub enn: Output<'static>, // PB13
}

pub struct Gpio_Pins {
    pub ld2: Output<'static>, // PA5 (LED verte)
    pub b1: Input<'static>, // PC13 (Bouton bleu)
}

pub struct Gamepad_Pins {
    pub btn_top: Input<'static>, // PC8
    pub btn_bottom: Input<'static>, // PB11
    pub btn_right: Input<'static>, // PC9
    pub btn_left: Input<'static>, // PC6
    pub btn_center: Input<'static>, // PC5
}

pub struct Magnetometer_Pins {
    pub status: Input<'static>, // PC1
    pub int: Input<'static>, // PB0
}

pub struct Encoder_Pins {
    pub enc_btn: Input<'static>, // PA15
    pub enc_a: Input<'static>, // PA0
    pub enc_b: Input<'static>, // PA1
}

pub struct Usart1_Pins {
    pub tx: Output<'static>, // PA9
    pub rx: Input<'static>, // PA10
}

pub struct Usart2_Pins {
    pub tx: Output<'static>, //PA2
    pub rx: Input<'static>, //PA3
}

pub struct Spi2_Pins {
    pub sck: Output<'static>, // PB10
    pub mosi: Output<'static>, // PC3
    pub miso: Input<'static>, // PC2
    pub cs: Output<'static>, // PC0
}

pub struct I2c1_Pins {
    pub scl: Output<'static>, // PB6
    pub sda: Peri<'static, AnyPin>, // PB7
}

pub struct Free_Pins {
    pub pc10: Peri<'static, AnyPin>,
    pub pc11: Peri<'static, AnyPin>,
    pub pc12: Peri<'static, AnyPin>,
    pub pb8: Peri<'static, AnyPin>,
    pub pb9: Peri<'static, AnyPin>,
    pub pd2: Peri<'static, AnyPin>,
}

