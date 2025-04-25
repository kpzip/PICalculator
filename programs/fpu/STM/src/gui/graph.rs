use lib::parser::expression::Expression;
use lib::parser::ExpressionError;
use rtt_target::rprintln;
use stm32f4xx_hal::gpio::Pin;
use stm32f4xx_hal::hal::digital::OutputPin;
use stm32f4xx_hal::spi::{Instance, SpiSlave};
use crate::gui::{CalculatorMenu, CalculatorState, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::util::abs;

pub struct GraphState {
    // Numbers / Pixel
    pub graph_x_scale: f64,
    pub graph_y_scale: f64,

    // Numbers NOT pixels
    pub graph_x_offset: f64,
    pub graph_y_offset: f64,

    pub expr: Expression,
}

impl GraphState {
    pub fn new() -> Self {
        Self {
            graph_x_scale: 1.0 / 8.0,
            graph_y_scale: 1.0 / 8.0,
            graph_x_offset: 0.0,
            graph_y_offset: 0.0,
            expr: Expression::Immediate(0.0),
        }
    }
}

pub fn handle_button_press(key_id: u8, calc_state: &mut CalculatorState) {
    match key_id {
        36 => {
            // Switch back to graph eq mode
            calc_state.mode = CalculatorMenu::GraphEq;
        }
        37 => {
            calc_state.mode = CalculatorMenu::Sci;
        }
        _ => {
            // Do Nothing
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
    const ORIGIN_X_PX_IDX: usize = (DISPLAY_WIDTH - 1) / 2;
    const ORIGIN_Y_PX_IDX: usize = DISPLAY_HEIGHT / 2;
    // Render the graph
    // Need one more to the right for infill
    // Up is positive
    // Right is positive
    let mut values: [i8; DISPLAY_WIDTH + 1] = [0; DISPLAY_WIDTH + 1];
    for i in 0..(DISPLAY_WIDTH + 1usize) {
        let display_index = i as isize - ORIGIN_X_PX_IDX as isize;
        let numerical_position = display_index as f64 * state.graph_state.graph_x_scale + state.graph_state.graph_x_offset;
        let evaluated = state.graph_state.expr.evaluate(&mut state.sci_state.variable_table, &[("X", numerical_position)], state.degrees);
        let val = match evaluated {
            Ok(value) => {
                value
            }
            Err(e) => {
                match e {
                    ExpressionError::InvalidSyntax(_) => {
                        // Should fail
                        rprintln!("bruh");
                        todo!()
                    }
                    ExpressionError::UnknownVariable(_) => {
                        // Should fail
                        rprintln!("bruh");
                        todo!()
                    }
                    ExpressionError::DivisionByZero => {
                        f64::INFINITY
                    }
                }
            }
        };
        let pixel_pos = ((val - state.graph_state.graph_y_offset)/state.graph_state.graph_y_scale + (ORIGIN_Y_PX_IDX as f64 - 1.0)) as isize;
        let pixel_pos_truncated = if pixel_pos >= DISPLAY_HEIGHT as isize {
            127
        } else if pixel_pos < 0{
            -1
        } else {
            pixel_pos as i8
        };
        values[i] = pixel_pos_truncated;
    }

    ready.set_high();
    spi.write(&[0x81, 0x01]).unwrap();

    spi.write(&[0x81, 0x0C]).unwrap();
    spi.write(&[0x81, 0x34]).unwrap();
    spi.write(&[0x81, 0x36]).unwrap();

    for vertical_pos in 0..DISPLAY_HEIGHT {
        spi.write(&[0x81, 0x80 | if vertical_pos >= DISPLAY_HEIGHT/2 { vertical_pos as u8 - 32 } else { vertical_pos as u8 }]).unwrap();
        spi.write(&[0x81, 0x80 | if vertical_pos >= DISPLAY_HEIGHT/2 { 8u8 } else { 0u8 }]).unwrap();
        for horizontal_pos in 0..8usize {
            let mut msb: u8 = 0;
            let mut lsb: u8 = 0;
            if vertical_pos != ORIGIN_Y_PX_IDX {
                for i in 0..8usize {
                    let vert = ((DISPLAY_HEIGHT - 1) - vertical_pos) as i8;
                    let index = horizontal_pos * 16 + i;
                    // Fix artifacts
                    let same_as_prev = if i > 0 { values[index-1] == values[index] } else { false };
                    if (values[index] > vert && vert > values[index + 1]) || (values[index] < vert && vert < values[index + 1]) || (values[index] == vert && !(same_as_prev && abs(values[index] - values[index+1]) > 1)) || (index == ORIGIN_X_PX_IDX) {
                        msb |= 0x80 >> i
                    }
                }
                for i in 0..8usize {
                    let vert = ((DISPLAY_HEIGHT - 1) - vertical_pos) as i8;
                    let index = horizontal_pos * 16 + 8 + i;
                    // Fix artifacts
                    let same_as_prev = if i > 0 { values[index-1] == values[index] } else { false };
                    if (values[index] > vert && vert > values[index + 1]) || (values[index] < vert && vert < values[index + 1]) || (values[index] == vert && !(same_as_prev && abs(values[index] - values[index+1]) > 1)) || (index == ORIGIN_X_PX_IDX) {
                        lsb |= 0x80 >> i
                    }
                }
            } else {
                msb = 0xFF;
                lsb = 0xFF;
            }
            spi.write(&[0x82, msb]).unwrap();
            spi.write(&[0x82, lsb]).unwrap();
        }
    }
    spi.write(&[0x81, 0x30]).unwrap();
    spi.write(&[0x00]).unwrap();
    ready.set_low();
}