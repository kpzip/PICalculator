use sci_error::SciErrorState;
use sci_mode::SciModeState;
use stm32f4xx_hal::spi::{Instance, SpiSlave};

pub mod sci_error;
pub mod sci_mode;

// 128 px wide by 64 px tall
pub const DISPLAY_WIDTH: usize = 128;
pub const DISPLAY_HEIGHT: usize = 64;

// 16 chars horizontally by 4 lines
pub const DISPLAY_TEXT_WIDTH: usize = 16;
pub const DISPLAY_TEXT_HEIGHT: usize = 4;

pub enum CalculatorMenu {
    Sci,
    SciError,
    GraphEq,
    Graph,
}

pub struct CalculatorState {
    pub second: bool,
    pub alpha: bool,
    pub degrees: bool,
    pub mode: CalculatorMenu,
    pub sci_state: SciModeState,
    pub sci_err_state: SciErrorState,
}

impl CalculatorState {
    pub fn new() -> Self {
        Self {
            second: false,
            alpha: false,
            degrees: false,
            mode: CalculatorMenu::Sci,
            sci_state: SciModeState::new(),
            sci_err_state: SciErrorState::new(),
        }
    }
}

#[inline]
pub fn get_cursor_index(mut row: u8, mut col: u8) -> u8 {
    col = (col & 0x0F) / 2;
    row = row & 0x03;

    let mut data = 0x80 | ((row & 0x01) << 4);
    data |= (row & 0x02) << 2;
    data |= col;
    data
}

#[inline]
pub fn clear_gdram<SPI: Instance>(spi: &mut SpiSlave<SPI>) {
    for i in 0..32u8 {
        spi.write(&[0x81, 0x80 | i]).unwrap();
        spi.write(&[0x81, 0x80]).unwrap();
        for i in 0..16u8 {
            spi.write(&[0x82, 0x00]).unwrap();
            spi.write(&[0x82, 0x00]).unwrap();
        }
    }
}
