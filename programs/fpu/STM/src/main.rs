#![feature(raw_ref_op)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod gui;
mod keymaps;
mod util;

extern crate alloc;
extern crate panic_halt;

use alloc::string::ToString;
use core::ops::{Deref, DerefMut};

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use hal::gpio::alt::spi1::Nss;
use hal::spi::{Mode, Phase, Polarity};
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;
use gui::{CalculatorMenu, CalculatorState, sci_error, sci_mode};
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

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

    let gpioa = dp.GPIOA.split();

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

    // Calc State
    let mut calc_state = CalculatorState::new();


    loop {
        match spi.read(&mut data) {
            Ok(()) => {
                rprintln!("Received data: {:02X?}", data);
                // Key press code
                if data[0] == 1 {
                    let key_id = data[1];
                    gui::handle_button_press(key_id, &mut calc_state);
                    gui::update_gui(&mut calc_state, &mut spi, &mut ready);
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
