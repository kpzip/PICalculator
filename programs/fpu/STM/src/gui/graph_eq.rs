use alloc::string::String;
use core::cmp::{max, min};
use stm32f4xx_hal::gpio::Pin;
use stm32f4xx_hal::hal::digital::OutputPin;
use stm32f4xx_hal::spi::{Instance, SpiSlave};
use crate::gui::{clear_gdram, CalculatorMenu, CalculatorState, DISPLAY_TEXT_WIDTH};
use crate::gui::sci_mode::Line;
use crate::keymaps::get_key_text;

const EQ_1_PREFIX: &str = "Y=";

pub struct GraphEqState {
    pub eq1: String,
    pub cursor_pos: usize,
    pub display_offset: usize,
}

impl GraphEqState {
    pub fn new() -> Self {
        Self {
            eq1: String::new(),
            cursor_pos: EQ_1_PREFIX.len(),
            display_offset: 0,
        }
    }
}

pub fn handle_button_press(key_id: u8, calc_state: &mut CalculatorState) {
    if let Some(text) = get_key_text(key_id, &calc_state) {
        if calc_state.graph_eq_state.cursor_pos >= EQ_1_PREFIX.len() {
            calc_state.graph_eq_state.eq1.insert_str(calc_state.graph_eq_state.cursor_pos - EQ_1_PREFIX.len(), text);
            calc_state.graph_eq_state.cursor_pos += text.len();
        }
    } else {
        match key_id {
            19 => {
                // Del
                if calc_state.graph_eq_state.cursor_pos >= EQ_1_PREFIX.len() {
                    if !calc_state.graph_eq_state.eq1.is_empty() {
                        calc_state.graph_eq_state.eq1.remove(max(calc_state.graph_eq_state.cursor_pos - 3, 0));
                    }
                    if calc_state.graph_eq_state.cursor_pos > EQ_1_PREFIX.len() {
                        calc_state.graph_eq_state.cursor_pos -= 1;
                    }
                }
            }
            13 => {
                // Clear
                calc_state.graph_eq_state.eq1.clear();
                calc_state.graph_eq_state.cursor_pos = EQ_1_PREFIX.len();
            }
            38 => {
                // left
                if calc_state.graph_eq_state.cursor_pos > 0 {
                    calc_state.graph_eq_state.cursor_pos -= 1;
                }
            }
            40 => {
                // right
                if calc_state.graph_eq_state.cursor_pos < calc_state.graph_eq_state.eq1.len() + EQ_1_PREFIX.len() {
                    calc_state.graph_eq_state.cursor_pos += 1;
                }
            }
            36 => {
                // Graph
                calc_state.mode = CalculatorMenu::Sci;
            }
            _ => {}
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
    let cursor_display_x = state.graph_eq_state.cursor_pos as isize - state.graph_eq_state.display_offset as isize;

    let bg_correct_x = if cursor_display_x < 0 {
        cursor_display_x
    } else if cursor_display_x >= DISPLAY_TEXT_WIDTH as isize {
        cursor_display_x - (DISPLAY_TEXT_WIDTH as isize - 1)
    } else {
        0
    };

    state.graph_eq_state.display_offset = (state.graph_eq_state.display_offset as isize + bg_correct_x) as usize;

    let padding = EQ_1_PREFIX.len();
    let eq_begin = min(max(state.graph_eq_state.display_offset as isize - padding as isize, 0) as usize, state.graph_eq_state.eq1.len());
    let eq_end = min(state.graph_eq_state.display_offset + DISPLAY_TEXT_WIDTH  - padding, state.graph_eq_state.eq1.len());
    let prefix_begin = min(state.graph_eq_state.display_offset, padding);
    let prefix_end = min(state.graph_eq_state.display_offset + DISPLAY_TEXT_WIDTH, padding);
    let prefix = &EQ_1_PREFIX[prefix_begin..prefix_end];
    let eq = &state.graph_eq_state.eq1[eq_begin..eq_end];

    let cursor_display_x = state.graph_eq_state.cursor_pos - state.graph_eq_state.display_offset;
    let horizontal_address = cursor_display_x as u8 / 2;
    let first_byte = cursor_display_x % 2 == 0;

    ready.set_high();

    spi.write(&[0x81, 0x01]).unwrap();

    for b in prefix.bytes().chain(eq.bytes()) {
        spi.write(&[0x82, b]).unwrap();
    }

    spi.write(&[0x81, 0x0C]).unwrap();
    spi.write(&[0x81, 0x34]).unwrap();
    spi.write(&[0x81, 0x36]).unwrap();

    clear_gdram(spi);

    spi.write(&[0x81, 0x80 | 15]).unwrap();
    spi.write(&[0x81, 0x80 | horizontal_address]).unwrap();

    if first_byte {
        spi.write(&[0x82, 0xFF]).unwrap();
        spi.write(&[0x82, 0x00]).unwrap();
    } else {
        spi.write(&[0x82, 0x00]).unwrap();
        spi.write(&[0x82, 0xFF]).unwrap();
    }

    spi.write(&[0x81, 0x30]).unwrap();

    spi.write(&[0x00]).unwrap();

    ready.set_low();
}
