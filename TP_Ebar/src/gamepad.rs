#![no_std]

use embassy_stm32::gpio::Input;
use crate::bsp_ensea::Gamepad_Pins;
pub struct Gamepad {
    btns: [Input<'static>; 5],  // 8 LEDs pour le bargraph
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GamepadState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}
impl Gamepad {
    // Crée un nouveau bargraph à partir des pins du BSP
    pub fn new(gamepad_pin: Gamepad_Pins) -> Self {
        let btns = [
            gamepad_pin.btn_top,
            gamepad_pin.btn_bottom,
            gamepad_pin.btn_right,
            gamepad_pin.btn_left,
            gamepad_pin.btn_center,
        ];
        Self {
                btns,
        }
    }

    pub fn is_pressed(&self, button:Button) -> bool {
        !match button {
            Button::Top => self.btns[0].is_high(),
            Button::Bottom => self.btns[1].is_high(),
            Button::Right => self.btns[2].is_high(),
            Button::Left => self.btns[3].is_high(),
            Button::Center => self.btns[4].is_high(),
        }
    }

    pub fn state(&self) -> GamepadState {
        GamepadState {
            top: !self.btns[0].is_high(),
            bottom: !self.btns[1].is_high(),
            left: !self.btns[2].is_high(),
            right: !self.btns[3].is_high(),
            center: !self.btns[4].is_high(),
        }
    }
}