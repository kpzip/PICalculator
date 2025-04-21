use crate::gui::{clear_gdram, get_cursor_index};
use crate::{CalculatorMenu, CalculatorState};
use lib::parser::ExpressionError;
use stm32f4xx_hal::gpio::Pin;
use stm32f4xx_hal::hal::digital::OutputPin;
use stm32f4xx_hal::spi::{Instance, SpiSlave};
pub struct SciErrorState(pub(crate) ExpressionError);

impl SciErrorState {
    pub fn new() -> Self {
        Self(ExpressionError::DivisionByZero)
    }
}

pub fn handle_button_press(key_id: u8, calc_state: &mut CalculatorState) {
    match key_id {
        20 => {
            // 1
            calc_state.mode = CalculatorMenu::Sci;
        }
        21 => {
            // 2
            let position = match calc_state.sci_err_state.0 {
                ExpressionError::InvalidSyntax(p) => p,
                ExpressionError::UnknownVariable(_) => 0,
                ExpressionError::DivisionByZero => 0,
            };
            calc_state.sci_state.column_number = position as isize;
            calc_state.mode = CalculatorMenu::Sci;
        }
        _ => {
            // Do Nothing
        }
    }
}

pub fn update_gui<SPI: Instance, MODE, const L: char, const N: u8>(
    state: &mut CalculatorState,
    mut spi: &mut SpiSlave<SPI>,
    ready: &mut Pin<L, N, MODE>,
) where
    Pin<L, N, MODE>: OutputPin
{
    let message = match state.sci_err_state.0 {
        ExpressionError::InvalidSyntax(_) => "Invalid Syntax",
        ExpressionError::UnknownVariable(ref var) => "Unknown Variable",
        ExpressionError::DivisionByZero => "Divide By Zero",
    };
    ready.set_high();
    clear_gdram(spi);
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
