use crate::gui::{DISPLAY_TEXT_HEIGHT, DISPLAY_TEXT_WIDTH, clear_gdram, get_cursor_index};
use crate::keymaps::get_key_text;
use crate::util::truncate_trailing_zeros;
use crate::{CalculatorMenu, CalculatorState};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::cmp::{max, min};
use core::ops::{Deref, DerefMut};
use lib::parser::parse;
use stm32f4xx_hal::gpio::Pin;
use stm32f4xx_hal::hal::digital::OutputPin;
use stm32f4xx_hal::spi::{Instance, SpiSlave};

pub struct SciModeState {
    pub variable_table: BTreeMap<String, f64>,
    pub text: Vec<Line>,
    pub text_vertical_offset: isize, // Text y posision relative to the display, 0 -> top of calc_state.sci_state.text is at the top of display, 2 is 2 state.text above, etc.
    pub text_horizontal_offset: isize, // Text x posision relative to the display, 0 -> left side of text is on the left side of the display, 2 is 2 characters to the left of the left side of the display
    pub line_number: isize, // top line = 0, bottom line = calc_state.sci_state.text.len() - 1
    pub column_number: isize, // leftmost calc_state.sci_state.column_number = 0, rightmost calc_state.sci_state.column_number = calc_state.sci_state.text[calc_state.sci_state.line_number].len() - 1
}

impl SciModeState {
    pub fn new() -> Self {
        Self {
            variable_table: BTreeMap::new(),
            text: vec![Line::new(false, String::new())],
            text_vertical_offset: 0,
            text_horizontal_offset: 0,
            line_number: 0,
            column_number: 0,
        }
    }
}

pub struct Line {
    text: String,
    pub is_ans: bool,
}
impl Line {
    fn new(is_ans: bool, text: String) -> Self {
        Self { text, is_ans }
    }
}

impl Deref for Line {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.text
    }
}

pub fn handle_button_press(key_id: u8, calc_state: &mut CalculatorState) {
    if let Some(text) = get_key_text(key_id, &calc_state) {
        // Text Key
        // Make sure calc_state.sci_state.text is setup properly and that the cursor is inbounds
        if calc_state.sci_state.text.is_empty() {
            calc_state
                .sci_state
                .text
                .push(Line::new(false, String::new()));
        }
        if calc_state.sci_state.line_number >= calc_state.sci_state.text.len() as isize {
            calc_state.sci_state.line_number = (calc_state.sci_state.text.len() - 1) as isize;
        }
        // No writing to ans state.text
        if !calc_state.sci_state.text[calc_state.sci_state.line_number as usize].is_ans {
            // Write character to the line
            calc_state.sci_state.text[calc_state.sci_state.line_number as usize]
                .insert_str(calc_state.sci_state.column_number as usize, text);
            calc_state.sci_state.column_number += text.len() as isize;

            // Update display
            // Have to do this every time due to cursor issue
        } else {
        }
    } else {
        match key_id {
            18 => {
                // Constants
                calc_state.constants_state.previous_screen = CalculatorMenu::Sci;
                calc_state.mode = CalculatorMenu::Constants
            }
            19 => {
                // Del
                if !calc_state.sci_state.text[calc_state.sci_state.line_number as usize].is_empty()
                {
                    calc_state.sci_state.text[calc_state.sci_state.line_number as usize]
                        .remove(max(calc_state.sci_state.column_number - 1, 0) as usize);
                }
                if calc_state.sci_state.column_number > 0 {
                    calc_state.sci_state.column_number -= 1;
                }
            }
            13 => {
                // Clear
                if !calc_state.sci_state.text[calc_state.sci_state.line_number as usize].is_ans {
                    if let Some(n2) = calc_state
                        .sci_state
                        .text
                        .get(calc_state.sci_state.line_number as usize + 1)
                    {
                        if n2.is_ans {
                            calc_state
                                .sci_state
                                .text
                                .remove(calc_state.sci_state.line_number as usize);
                            calc_state
                                .sci_state
                                .text
                                .remove(calc_state.sci_state.line_number as usize);
                        } else {
                            calc_state
                                .sci_state
                                .text
                                .remove(calc_state.sci_state.line_number as usize);
                        }
                    } else {
                        calc_state.sci_state.text[calc_state.sci_state.line_number as usize].clear();
                    }
                    if calc_state.sci_state.text.is_empty() {
                        calc_state
                            .sci_state
                            .text
                            .push(Line::new(false, String::new()));
                    }
                    if calc_state.sci_state.line_number == calc_state.sci_state.text.len() as isize
                    {
                        calc_state.sci_state.line_number -= 1;
                    }
                    if calc_state.sci_state.column_number
                        > calc_state.sci_state.text[calc_state.sci_state.line_number as usize].len()
                            as isize
                        && !calc_state.sci_state.text[calc_state.sci_state.line_number as usize]
                            .is_ans
                    {
                        calc_state.sci_state.column_number =
                            calc_state.sci_state.text[calc_state.sci_state.line_number as usize]
                                .len() as isize;
                    }
                }
            }
            38 => {
                // left
                if calc_state.sci_state.column_number > 0
                    && !calc_state.sci_state.text[calc_state.sci_state.line_number as usize].is_ans
                {
                    calc_state.sci_state.column_number -= 1;
                }
            }
            39 => {
                // up
                if calc_state.sci_state.line_number > 0 {
                    calc_state.sci_state.line_number -= 1;
                    if calc_state.sci_state.column_number
                        > calc_state.sci_state.text[calc_state.sci_state.line_number as usize].len()
                            as isize
                    {
                        calc_state.sci_state.column_number =
                            calc_state.sci_state.text[calc_state.sci_state.line_number as usize]
                                .len() as isize;
                    }
                }
            }
            40 => {
                // right
                if calc_state.sci_state.column_number
                    < calc_state.sci_state.text[calc_state.sci_state.line_number as usize].len()
                        as isize
                    && !calc_state.sci_state.text[calc_state.sci_state.line_number as usize].is_ans
                {
                    calc_state.sci_state.column_number += 1;
                }
            }
            41 => {
                // down
                if calc_state.sci_state.line_number < calc_state.sci_state.text.len() as isize - 1 {
                    calc_state.sci_state.line_number += 1;
                    if calc_state.sci_state.column_number
                        > calc_state.sci_state.text[calc_state.sci_state.line_number as usize].len()
                            as isize
                    {
                        calc_state.sci_state.column_number =
                            calc_state.sci_state.text[calc_state.sci_state.line_number as usize]
                                .len() as isize;
                    }
                }
            }
            36 => {
                // Graph
                calc_state.mode = CalculatorMenu::GraphEq;
            }
            5 => {
                // Eval
                // dont eval if we are on an answer line
                if !calc_state.sci_state.text[calc_state.sci_state.line_number as usize].is_ans
                    && !calc_state.sci_state.text[calc_state.sci_state.line_number as usize]
                        .is_empty()
                {
                    let parsed = parse(
                        calc_state.sci_state.text[calc_state.sci_state.line_number as usize]
                            .as_str(),
                    );
                    match parsed {
                        Ok(e) => match e.evaluate(&mut calc_state.sci_state.variable_table, &[], calc_state.degrees) {
                            Ok(val) => {
                                calc_state
                                    .sci_state
                                    .variable_table
                                    .insert(String::from("ANS"), val);
                                let res = truncate_trailing_zeros(format!("{:.10}", val));
                                if calc_state.sci_state.line_number
                                    == calc_state.sci_state.text.len() as isize - 1
                                {
                                    calc_state.sci_state.text.push(Line::new(true, res));
                                    calc_state
                                        .sci_state
                                        .text
                                        .push(Line::new(false, String::new()));
                                    calc_state.sci_state.line_number += 2;
                                    calc_state.sci_state.column_number = 0;
                                } else if calc_state.sci_state.line_number
                                    == calc_state.sci_state.text.len() as isize - 2
                                {
                                    calc_state
                                        .sci_state
                                        .text
                                        .push(Line::new(false, String::new()));
                                    calc_state.sci_state.line_number += 2;
                                    calc_state.sci_state.column_number = 0;
                                } else {
                                    calc_state.sci_state.text
                                        [calc_state.sci_state.line_number as usize + 1] =
                                        Line::new(true, res);
                                    calc_state.sci_state.line_number += 1;
                                }
                            }
                            Err(e) => {
                                calc_state.sci_err_state.0 = e;
                                calc_state.mode = CalculatorMenu::SciError;
                            }
                        },
                        Err(e) => {
                            calc_state.sci_err_state.0 = e;
                            calc_state.mode = CalculatorMenu::SciError;
                        }
                    }
                } else {
                }
            }
            _ => {
                // respond with no data for unimplemented button
            }
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
    let fake_cursor_colum = if state.sci_state.text[state.sci_state.line_number as usize].is_ans {
        0
    } else {
        state.sci_state.column_number
    };

    // If the cursor has gone beyond the bounds of the display, we need to correct for it
    let old_cursor_display_x: isize = fake_cursor_colum - state.sci_state.text_horizontal_offset;
    let old_cursor_display_y: isize =
        state.sci_state.line_number - state.sci_state.text_vertical_offset;
    let bg_correct_x: isize = if old_cursor_display_x < 0 {
        old_cursor_display_x
    } else if old_cursor_display_x >= DISPLAY_TEXT_WIDTH as isize {
        old_cursor_display_x - (DISPLAY_TEXT_WIDTH as isize - 1)
    } else {
        0
    };
    let bg_correct_y: isize = if old_cursor_display_y < 0 {
        old_cursor_display_y
    } else if old_cursor_display_y >= DISPLAY_TEXT_HEIGHT as isize {
        old_cursor_display_y - (DISPLAY_TEXT_HEIGHT as isize - 1)
    } else {
        0
    };

    // Move the background so that way we can see the cursor again
    state.sci_state.text_horizontal_offset += bg_correct_x;
    state.sci_state.text_vertical_offset += bg_correct_y;

    // Reinit since we need to fix the cursor position on the display
    let cursor_display_x = (fake_cursor_colum - state.sci_state.text_horizontal_offset) as u8;
    let cursor_display_y =
        (state.sci_state.line_number - state.sci_state.text_vertical_offset) as u8;

    // Reset and redraw display
    let vert_slice = &state.sci_state.text[state.sci_state.text_vertical_offset as usize
        ..min(
            state.sci_state.text_vertical_offset as usize + DISPLAY_TEXT_HEIGHT,
            state.sci_state.text.len(),
        )];
    let horizontal_slice = vert_slice
        .iter()
        .map(|l| {
            if l.is_ans {
                let padding = max(DISPLAY_TEXT_WIDTH as isize - l.len() as isize, 0);
                let start_index = min(
                    max(state.sci_state.text_horizontal_offset - padding, 0) as usize,
                    l.len(),
                );
                let end_index = min(start_index + DISPLAY_TEXT_WIDTH, l.len());
                (&l[start_index..end_index], l.is_ans)
            } else {
                (
                    &l[min(state.sci_state.text_horizontal_offset as usize, l.len())
                        ..min(
                            state.sci_state.text_horizontal_offset as usize + DISPLAY_TEXT_WIDTH,
                            l.len(),
                        )],
                    l.is_ans,
                )
            }
        })
        .collect::<Box<[_]>>();

    ready.set_high();
    spi.write(&[0x81, 0x01]).unwrap();
    for i in 0..horizontal_slice.len() {
        let y = horizontal_slice[i].0;
        // Set cursor position to the beginning of the row
        spi.write(&[0x81, get_cursor_index(i as u8, 0)]).unwrap();
        if horizontal_slice[i].1 && y.len() < DISPLAY_TEXT_WIDTH {
            for _ in
                0..(DISPLAY_TEXT_WIDTH - y.len() - state.sci_state.text_horizontal_offset as usize)
            {
                spi.write(&[0x82, 0x20]).unwrap()
            }
        }
        for x in y.bytes() {
            spi.write(&[0x82, x]).unwrap();
        }
    }

    // Switch to graphics mode to draw the cursor manually
    spi.write(&[0x81, 0x0C]).unwrap();
    spi.write(&[0x81, 0x34]).unwrap();
    spi.write(&[0x81, 0x36]).unwrap();

    // Zero out all cursor locations
    clear_gdram(spi);

    let vertical_address = 15 + 16 * cursor_display_y;
    let horizontal_address = cursor_display_x / 2;
    let first_byte = cursor_display_x % 2 == 0;

    if !state.sci_state.text[state.sci_state.line_number as usize].is_ans {
        // OR not AND!!!!!
        spi.write(&[0x81, 0x80 | (vertical_address % 32)]).unwrap();
        spi.write(&[
            0x81,
            0x80 | if vertical_address >= 32 {
                horizontal_address + 8
            } else {
                horizontal_address
            },
        ])
        .unwrap();

        if first_byte {
            spi.write(&[0x82, 0xFF]).unwrap();
            spi.write(&[0x82, 0x00]).unwrap();
        } else {
            spi.write(&[0x82, 0x00]).unwrap();
            spi.write(&[0x82, 0xFF]).unwrap();
        }
    } else {
        // Cursor should be a highlight if selecting an answer line
        let string = horizontal_slice[cursor_display_y as usize].0;
        let start = DISPLAY_TEXT_WIDTH as isize
            - string.len() as isize
            - state.sci_state.text_horizontal_offset;
        let end = start + string.len() as isize;
        let start = min(max(start, 0), DISPLAY_TEXT_WIDTH as isize) as u8;
        let end = min(max(end, 0), DISPLAY_TEXT_WIDTH as isize) as u8;
        for vert in 1..14 {
            let vertical_address = cursor_display_y * 16 + vert;
            let horizontal_address = start / 2;

            spi.write(&[0x81, 0x80 | (vertical_address % 32)]).unwrap();
            spi.write(&[
                0x81,
                0x80 | if vertical_address >= 32 {
                    horizontal_address + 8
                } else {
                    horizontal_address
                },
            ])
            .unwrap();

            if start % 2 == 1 {
                spi.write(&[0x82, 0x00]).unwrap();
            }
            for _ in 0..end - start {
                spi.write(&[0x82, 0xFF]).unwrap();
            }
            if start + end % 2 == 1 {
                spi.write(&[0x82, 0x00]).unwrap();
            }
        }
    }

    // Switch back and end transmission
    spi.write(&[0x81, 0x30]).unwrap();
    spi.write(&[0x00]).unwrap();
    ready.set_low();
}
