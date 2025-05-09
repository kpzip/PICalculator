use core::cmp::min;
use stm32f4xx_hal::gpio::Pin;
use stm32f4xx_hal::hal::digital::OutputPin;
use stm32f4xx_hal::spi::{Instance, SpiSlave};
use crate::gui::{CalculatorMenu, CalculatorState, clear_gdram, get_cursor_index};

// Math
pub const PI: f64 = core::f64::consts::PI;
pub const E: f64 = core::f64::consts::E;

// Physics
pub const G: f64 = 6.6743015e-11;
pub const C: f64 = 3e+8;
pub const VACUUM_PERMITTIVITY: f64 = 8.85e-12;
pub const VACUUM_PERMEABILITY: f64 = 4.0 * PI * 1e-7;
pub const PLANKS_CONSTANT: f64 = 6.63e-34;

// Chemistry
pub const AVOGADROS_NUMBER: f64 = 6.022e-23;
pub const FARADAYS_CONSTANT: f64 = 96485.3329;
pub const GAS_CONSTANT_SI: f64 = 8.314;
pub const GAS_CONSTANT_L_ATM: f64 = 0.08602;
pub const BOLTZMANN_CONSTANT: f64 = 1.38e-23;

const CONSTANTS_TEXT: &[&str] = &[
    "Pi",
    "e",
];

pub struct ConstantsState {
    display_vertical_offset: usize,
    cursor_vertical_position: usize,
    pub previous_screen: CalculatorMenu,
}

impl ConstantsState {

    pub fn new() -> Self {
        Self {
            display_vertical_offset: 0,
            cursor_vertical_position: 0,
            previous_screen: CalculatorMenu::Sci,
        }
    }

}

pub fn handle_button_press(key_id: u8, calc_state: &mut CalculatorState) {

    match key_id {
        18 => {
            calc_state.mode = calc_state.constants_state.previous_screen;
        }
        39 => {
            // up
            if calc_state.constants_state.cursor_vertical_position > 0 {
                calc_state.constants_state.cursor_vertical_position -= 1;
            }
        }
        41 => {
            // down
            if calc_state.constants_state.cursor_vertical_position < CONSTANTS_TEXT.len() - 1 {
                calc_state.constants_state.cursor_vertical_position += 1;
            }
        }
        _ => {

        }
    }

}

pub fn update_gui<SPI: Instance, MODE, const L: char, const N: u8>(
    state: &mut CalculatorState,
    spi: &mut SpiSlave<SPI>,
    ready: &mut Pin<L, N, MODE>,
) where
    Pin<L, N, MODE>: OutputPin
{

    let cursor_display_position: isize = state.constants_state.cursor_vertical_position as isize - state.constants_state.display_vertical_offset as isize;
    if cursor_display_position < 0 {
        state.constants_state.display_vertical_offset = state.constants_state.cursor_vertical_position;
    } else if cursor_display_position > 3 {
        state.constants_state.display_vertical_offset = state.constants_state.cursor_vertical_position - 3;
    }

    let cursor_display_y = state.constants_state.cursor_vertical_position - state.constants_state.display_vertical_offset;

    let constant_names = &CONSTANTS_TEXT[state.constants_state.display_vertical_offset..min(state.constants_state.display_vertical_offset + 4, CONSTANTS_TEXT.len())];

    ready.set_high();

    spi.write(&[0x81, 0x01]).unwrap();

    spi.write(&[0x81, 0x0C]).unwrap();
    spi.write(&[0x81, 0x34]).unwrap();
    spi.write(&[0x81, 0x36]).unwrap();

    clear_gdram(spi);

    for vert in 1..14 {
        let vertical_address = cursor_display_y * 16 + vert;
        let horizontal_address = 0;

        spi.write(&[0x81, 0x80 | (vertical_address % 32) as u8]).unwrap();
        spi.write(&[
            0x81,
            0x80 | if vertical_address >= 32 {
                horizontal_address + 8
            } else {
                horizontal_address
            },
        ])
            .unwrap();

        for _ in 0..16 {
            spi.write(&[0x82, 0xFF]).unwrap()
        }
    }

    spi.write(&[0x81, 0x30]).unwrap();

    for i in 0..constant_names.len() {
        spi.write(&[0x81, get_cursor_index(i as u8, 0)]).unwrap();
        let name = constant_names[i];
        for b in name.bytes() {
            spi.write(&[0x82, b]).unwrap();
        }
    }

    spi.write(&[0x00]);
    ready.set_low();

}