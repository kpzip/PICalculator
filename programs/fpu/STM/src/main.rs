#![feature(raw_ref_op)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod util;

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::{format, vec};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::{max, min};
use core::ops::{Deref, DerefMut};
use cortex_m::asm::delay;
use panic_halt as _; // panic handler

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use hal::gpio::alt::spi1::Nss;
use hal::spi::{Mode, Phase, Polarity};
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

use embedded_alloc::LlffHeap as Heap;
use lib::parser::{parse, ExpressionError};
use lib::parser::expression::Expression;
use stm32f4xx_hal::gpio::{Output, Pin};
use stm32f4xx_hal::pac::SPI1;
use stm32f4xx_hal::spi::SpiSlave;
use crate::util::truncate_trailing_zeros;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// 128 px wide by 64 px tall
const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: usize = 64;

// 16 chars horizontally by 4 lines
const DISPLAY_TEXT_WIDTH: usize = 16;
const DISPLAY_TEXT_HEIGHT: usize = 4;

// Keymap when not in alpha or second
const NORMAL_KEYMAP: &[(u8, &str)] = &[
    (2, "."),
    (3, "0"),
    (8, "7"),
    (9, "8"),
    (10, "9"),
    (14, "4"),
    (15, "5"),
    (16, "6"),
    (20, "1"),
    (21, "2"),
    (22, "3"),
    (29, "+"),
    (23, "-"),
    (17, "*"),
    (11, "/"),
    (25, "("),
];

const SECOND_KEYMAP: &[(u8, &str)] = &[
    (25, ")"),
];

fn get_key_text(key: u8, second: bool, alpha: bool) -> Option<&'static str> {
    if second {
        SECOND_KEYMAP
    } else {
        NORMAL_KEYMAP
    }.iter().find(|m| m.0 == key).map(|x| x.1)
}

enum CalculatorMode {
    Sci,
    SciError(ExpressionError),
    GraphEq,
    Graph,
}

struct Line {
    text: String,
    is_ans: bool,
}

impl Line {
    fn new(is_ans: bool, text: String) -> Self {
        Self {
            text,
            is_ans
        }
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


#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 4;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
    let gpioa = dp.GPIOA.split();
    //let mut led = gpioa.pa5.into_push_pull_output();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    let mut ready = gpioa.pa2.into_push_pull_output();
    ready.set_low();

    let sck = gpioa.pa5;
    let mut miso = gpioa.pa6;
    let mosi = gpioa.pa7;
    let nss = gpioa.pa4;

    let mut spi = dp.SPI1.spi_slave(
        (sck, miso, mosi, Some(Nss::from(nss))),
        Mode {
            phase: Phase::CaptureOnSecondTransition,
            polarity: Polarity::IdleHigh,
        },
    );

    // Buffer for reading in SPI data
    let mut data: [u8; 2] = [0; 2];

    // Gui Mode
    let mut mode: CalculatorMode = CalculatorMode::Sci;

    let mut second: bool = false;
    let mut alpha: bool = false;
    let mut degrees: bool = false;

    // Sci Mode Data
    let mut variable_table: BTreeMap<String, f64> = BTreeMap::new();
    let mut sci_mode_text: Vec<Line> = vec![Line::new(false, String::new())];
    let mut text_vertical_offset: isize = 0; // Text y posision relative to the display, 0 -> top of sci_mode_text is at the top of display, 2 is 2 lines above, etc.
    let mut text_horizontal_offset: isize = 0; // Text x posision relative to the display, 0 -> left side of text is on the left side of the display, 2 is 2 characters to the left of the left side of the display
    let mut line_number: isize = 0; // top line = 0, bottom line = sci_mode_text.len() - 1
    let mut column_number: isize = 0; // leftmost column = 0, rightmost column = sci_mode_text[line_number].len() - 1


    let mut graph_eq_text: String = String::new();

    // 1 extra since we need to look 1 beyond the display
    let mut graph_data: [u8; DISPLAY_WIDTH + 1] = [0; DISPLAY_WIDTH + 1];
    let mut graph_scale_x: f64 = 1.0;
    let mut graph_scale_y: f64 = 1.0;



    loop {
        match spi.read(&mut data) {
            Ok(_) => {
                rprintln!("Received data: {:02X?}", data);
                // Key press code
                if data[0] == 1 {

                    match mode {
                        CalculatorMode::Sci => {
                            let key_id = data[1];
                            if let Some(text) = get_key_text(key_id, second, alpha) {
                                // Text Key
                                // Make sure sci_mode_text is setup properly and that the cursor is inbounds
                                if sci_mode_text.is_empty() {
                                    sci_mode_text.push(Line::new(false, String::new()));
                                }
                                if line_number >= sci_mode_text.len() as isize {
                                    line_number = (sci_mode_text.len() - 1) as isize;
                                }
                                // No writing to ans lines
                                if !sci_mode_text[line_number as usize].is_ans {
                                    // Write character to the line
                                    sci_mode_text[line_number as usize].insert_str(column_number as usize, text);
                                    column_number += text.len() as isize;

                                    // Update display
                                    // Have to do this every time due to cursor issue
                                    sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                } else {
                                    ready.set_high();
                                    spi.write(&[0x00]).unwrap();
                                    ready.set_low();
                                }
                            } else {
                                match key_id {
                                    24 => {
                                        // Alpha
                                        alpha = !alpha;
                                        ready.set_high();
                                        spi.write(&[0x00]).unwrap();
                                        ready.set_low();
                                    },
                                    30 => {
                                        // Second
                                        second = !second;
                                        ready.set_high();
                                        spi.write(&[0x00]).unwrap();
                                        ready.set_low();
                                    }
                                    19 => {
                                        // Del
                                        if !sci_mode_text[line_number as usize].is_empty() {
                                            sci_mode_text[line_number as usize].remove(max(column_number - 1, 0) as usize);
                                        }
                                        if column_number > 0 {
                                            column_number -= 1;
                                        }
                                        sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                    }
                                    13 => {
                                        // Clear
                                        if !sci_mode_text[line_number as usize].is_ans {
                                            sci_mode_text.remove(line_number as usize);
                                            if let Some(n2) = sci_mode_text.get(line_number as usize) {
                                                if n2.is_ans {
                                                    sci_mode_text.remove(line_number as usize);
                                                }
                                            }
                                            if sci_mode_text.is_empty() {
                                                sci_mode_text.push(Line::new(false, String::new()));
                                            }
                                            if line_number == sci_mode_text.len() as isize {
                                                line_number -= 1;
                                            }
                                            if column_number > sci_mode_text[line_number as usize].len() as isize && !sci_mode_text[line_number as usize].is_ans {
                                                column_number = sci_mode_text[line_number as usize].len() as isize;
                                            }
                                        }
                                        sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                    }
                                    38 => {
                                        // left
                                        if column_number > 0 && !sci_mode_text[line_number as usize].is_ans {
                                            column_number -= 1;
                                        }
                                        sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                    }
                                    39 => {
                                        // up
                                        if line_number > 0 {
                                            line_number -= 1;
                                            if column_number > sci_mode_text[line_number as usize].len() as isize {
                                                column_number = sci_mode_text[line_number as usize].len() as isize;
                                            }
                                        }
                                        sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                    }
                                    40 => {
                                        // right
                                        if column_number < sci_mode_text[line_number as usize].len() as isize && !sci_mode_text[line_number as usize].is_ans {
                                            column_number += 1;
                                        }
                                        sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                    }
                                    41 => {
                                        // down
                                        if line_number < sci_mode_text.len() as isize - 1 {
                                            line_number += 1;
                                            if column_number > sci_mode_text[line_number as usize].len() as isize {
                                                column_number = sci_mode_text[line_number as usize].len() as isize;
                                            }
                                        }
                                        sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                    }
                                    5 => {
                                        // Eval
                                        // dont eval if we are on an answer line
                                        if !sci_mode_text[line_number as usize].is_ans && !sci_mode_text[line_number as usize].is_empty() {
                                            let parsed = parse(sci_mode_text[line_number as usize].as_str());
                                            match parsed {
                                                Ok(e) => {
                                                    match e.evaluate(&mut variable_table, &[]) {
                                                        Ok(val) => {
                                                            variable_table.insert(String::from("ANS"), val);
                                                            let res = truncate_trailing_zeros(format!("{:.10}", val));
                                                            if line_number == sci_mode_text.len() as isize - 1 {
                                                                sci_mode_text.push(Line::new(true, res));
                                                                sci_mode_text.push(Line::new(false, String::new()));
                                                                line_number += 2;
                                                                column_number = 0;
                                                            } else if line_number == sci_mode_text.len() as isize - 2 {
                                                                sci_mode_text.push(Line::new(false, String::new()));
                                                                line_number += 2;
                                                                column_number = 0;
                                                            } else {
                                                                sci_mode_text[line_number as usize + 1] = Line::new(true, res);
                                                                line_number += 1;
                                                            }
                                                            sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                                        }
                                                        Err(e) => {
                                                            sci_error_display(&mut ready, &mut spi, &e);
                                                            mode = CalculatorMode::SciError(e);
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    sci_error_display(&mut ready, &mut spi, &e);
                                                    mode = CalculatorMode::SciError(e);
                                                }
                                            }
                                        } else {
                                            ready.set_high();
                                            spi.write(&[0x00]).unwrap();
                                            ready.set_low();
                                        }
                                    }
                                    _ => {
                                        // respond with no data for unimplemented button
                                        ready.set_high();
                                        spi.write(&[0x00]).unwrap();
                                        ready.set_low();
                                    }
                                }
                            }

                        }
                        CalculatorMode::SciError(ref e) => {
                            let key_id = data[1];
                            match key_id {
                                20 => {
                                    // 1
                                    mode = CalculatorMode::Sci;
                                    sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                }
                                21 => {
                                    // 2
                                    let position = match e {
                                        ExpressionError::InvalidSyntax(p) => { *p }
                                        ExpressionError::UnknownVariable(_) => { 0 }
                                        ExpressionError::DivisionByZero => { 0 }
                                    };
                                    column_number = position as isize;
                                    mode = CalculatorMode::Sci;
                                    sci_mode_fix_display(&sci_mode_text, &mut ready, &mut spi, &mut text_vertical_offset, &mut text_horizontal_offset, line_number, column_number);
                                }
                                _ => {
                                    ready.set_high();
                                    spi.write(&[0x00]).unwrap();
                                    ready.set_low();
                                }
                            }
                        }
                        _ => unimplemented!(),
                    }
                } else {
                    rprintln!("Error Unknown Request: {:02X?}", data);
                }
            }
            Err(e) => {
                rprintln!("Error: {:?}", e);
            }
        }
    }
}

fn sci_error_display(ready: &mut Pin<'A', 2, Output>, spi: &mut SpiSlave<SPI1>, error: &ExpressionError) {
    let message = match error {
        ExpressionError::InvalidSyntax(_) => {
            "Invalid Syntax"
        }
        ExpressionError::UnknownVariable(var) => {
            "Unknown Variable"
        }
        ExpressionError::DivisionByZero => {
            "Divide By Zero"
        }
    };
    ready.set_high();
    clear_sci_cursor(spi);
    spi.write(&[0x81, 0x01]).unwrap();
    // Have to do this since the PIC doesn't add the proper delay for the display reset
    spi.write(&[0x81, 0x02]).unwrap();
    spi.write(&[0x81, get_cursor_index(0, 0)]).unwrap();
    for b in message.bytes() {
        spi.write(&[0x82, b]).unwrap();
    }
    spi.write(&[0x81, get_cursor_index(1, 0)]).unwrap();
    for b in "1: Quit".bytes() {
        spi.write(&[0x82, b]).unwrap();
    }
    spi.write(&[0x81, get_cursor_index(2, 0)]).unwrap();
    for b in "2: Goto".bytes() {
        spi.write(&[0x82, b]).unwrap();
    }
    spi.write(&[0x00]).unwrap();
    ready.set_low();
}

/// Magic function that fixes everything
#[inline]
fn sci_mode_fix_display(lines: &[Line], ready: &mut Pin<'A', 2, Output>, spi: &mut SpiSlave<SPI1>, vertical: &mut isize, horizontal: &mut isize, cursor_line: isize, cursor_column: isize) {

    let fake_cursor_colum = if lines[cursor_line as usize].is_ans { 0 } else { cursor_column };

    // If the cursor has gone beyond the bounds of the display, we need to correct for it
    let old_cursor_display_x: isize = fake_cursor_colum - *horizontal;
    let old_cursor_display_y: isize = cursor_line - *vertical;
    let bg_correct_x: isize = if old_cursor_display_x < 0 { old_cursor_display_x } else if old_cursor_display_x >= DISPLAY_TEXT_WIDTH as isize { old_cursor_display_x - (DISPLAY_TEXT_WIDTH as isize - 1) } else { 0 };
    let bg_correct_y: isize = if old_cursor_display_y < 0 { old_cursor_display_y } else if old_cursor_display_y >= DISPLAY_TEXT_HEIGHT as isize { old_cursor_display_y - (DISPLAY_TEXT_HEIGHT as isize -1) } else { 0 };

    // Move the background so that way we can see the cursor again
    *horizontal += bg_correct_x;
    *vertical += bg_correct_y;

    // Reinit since we need to fix the cursor position on the display
    let cursor_display_x = (fake_cursor_colum - *horizontal) as u8;
    let cursor_display_y = (cursor_line - *vertical) as u8;

    // Reset and redraw display
    let vert_slice = &lines[*vertical as usize..min(*vertical as usize + DISPLAY_TEXT_HEIGHT, lines.len())];
    let horizontal_slice = vert_slice.iter().map(|l|{
        if l.is_ans {
            let padding = max(DISPLAY_TEXT_WIDTH as isize - l.len() as isize, 0);
            let start_index = min(max(*horizontal - padding, 0) as usize, l.len());
            let end_index = min(start_index + DISPLAY_WIDTH, l.len());
            (&l[start_index..end_index], l.is_ans)
        } else {
            (&l[min(*horizontal as usize, l.len())..min(*horizontal as usize + DISPLAY_TEXT_WIDTH, l.len())], l.is_ans)
        }
    }).collect::<Box<[_]>>();

    ready.set_high();
    spi.write(&[0x81, 0x01]).unwrap();
    // Have to do this since the PIC doesn't add the proper delay for the display reset
    spi.write(&[0x81, 0x02]).unwrap();
    for i in 0..horizontal_slice.len() {
        let y = horizontal_slice[i].0;
        // Set cursor position to the beginning of the row
        spi.write(&[0x81, get_cursor_index(i as u8, 0)]).unwrap();
        if horizontal_slice[i].1 && y.len() < DISPLAY_TEXT_WIDTH {
            for _ in 0..(DISPLAY_TEXT_WIDTH - y.len() - *horizontal as usize) {
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
    clear_sci_cursor(spi);

    let vertical_address = 15 + 16 * cursor_display_y;
    let horizontal_address = cursor_display_x / 2;
    let first_byte = cursor_display_x % 2 == 0;


    if !lines[cursor_line as usize].is_ans {
        // OR not AND!!!!!
        spi.write(&[0x81, 0x80 | (vertical_address % 32)]).unwrap();
        spi.write(&[0x81, 0x80 | if vertical_address >= 32 { horizontal_address + 8 } else { horizontal_address }]).unwrap();

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
        let start = DISPLAY_TEXT_WIDTH as isize - string.len() as isize - *horizontal;
        let end = start + string.len() as isize;
        let start = min(max(start, 0), DISPLAY_TEXT_WIDTH as isize) as u8;
        let end = min(max(end, 0), DISPLAY_TEXT_WIDTH as isize) as u8;
        for vert in 1..14 {
            let vertical_address = cursor_display_y * 16 + vert;
            let horizontal_address = start / 2;

            spi.write(&[0x81, 0x80 | (vertical_address % 32)]).unwrap();
            spi.write(&[0x81, 0x80 | if vertical_address >= 32 { horizontal_address + 8 } else { horizontal_address }]).unwrap();

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

#[inline]
fn get_cursor_index(mut row: u8, mut col: u8) -> u8 {
    col = (col & 0x0F)/2;
    row = row & 0x03;

    let mut data = 0x80 | ((row & 0x01) << 4);
    data |= (row & 0x02) << 2;
    data |= col;
    data
}

#[inline]
fn clear_sci_cursor(spi: &mut SpiSlave<SPI1>) {
    for i in 0..32u8 {
        spi.write(&[0x81, 0x80 | i]).unwrap();
        spi.write(&[0x81, 0x80]).unwrap();
        for i in 0..16u8 {
            spi.write(&[0x82, 0x00]).unwrap();
            spi.write(&[0x82, 0x00]).unwrap();
        }
    }


    // spi.write(&[0x81, 0x80 | 15]).unwrap();
    // spi.write(&[0x81, 0x80]).unwrap();
    // for _ in 0..16u8 {
    //     spi.write(&[0x82, 0x00]).unwrap();
    //     spi.write(&[0x82, 0x00]).unwrap();
    // }
    // spi.write(&[0x81, 0x80 | 31]).unwrap();
    // spi.write(&[0x81, 0x80]).unwrap();
    // for _ in 0..16u8 {
    //     spi.write(&[0x82, 0x00]).unwrap();
    //     spi.write(&[0x82, 0x00]).unwrap();
    // }
}
