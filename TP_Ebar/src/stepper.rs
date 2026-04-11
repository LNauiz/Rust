#![no_std]

// stepper.rs
use embassy_stm32::gpio::{Output};
use embassy_time::Timer;
use crate::bsp_ensea::Stepper_Pins;


// Énumération pour la direction de rotation
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

// Énumération pour le mode de microstepping
#[derive(Debug, Clone, Copy)]
pub enum MicrosteppingMode {
    FullStep,
    HalfStep,
    QuarterStep,
    EighthStep,
}

// Structure pour le moteur pas à pas
pub struct Stepper {
    step: Output<'static>,
    direction: Output<'static>,
    enable: Output<'static>,
    ms1: Output<'static>,
    ms2: Output<'static>,
}

impl Stepper{
    // Crée un nouveau moteur pas à pas
    pub fn new(stepper_pins: Stepper_Pins) -> Self {
        let step = stepper_pins.step;
        let ms1 = stepper_pins.ms1;
        let ms2 = stepper_pins.ms2;
        let enable = stepper_pins.enable;
        let direction = stepper_pins.direction;

        Self {
            step,
            direction,
            enable,
            ms1,
            ms2,
        }
    }

    // Définit la vitesse de rotation
    pub async fn set_speed(&mut self, speed: u32) {
        let delay = 1000 / speed as u64;
        loop{
        self.step.set_high();
        Timer::after_micros(delay).await;
        self.step.set_low();
        Timer::after_micros(delay).await;
        }
    }


    //Definit la direction
    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Forward => self.direction.set_high(),
            Direction::Backward => self.direction.set_low(),
        }
    }

    // Active le moteur
    pub fn enable(&mut self) {
        self.enable.set_low();
    }

    // Désactive le moteur
    pub fn disable(&mut self) {
        self.enable.set_high(); // Désactiver le moteur
    }

    // Définit le mode de microstepping
    pub fn set_microstepping(&mut self, mode: MicrosteppingMode) {
        match mode {
            MicrosteppingMode::FullStep => {
                self.ms1.set_low();
                self.ms2.set_low();
            }
            MicrosteppingMode::HalfStep => {
                self.ms1.set_high();
                self.ms2.set_low();
            }
            MicrosteppingMode::QuarterStep => {
                self.ms1.set_low();
                self.ms2.set_high();
            }
            MicrosteppingMode::EighthStep => {
                self.ms1.set_high();
                self.ms2.set_high();
            }
        }
    }
}