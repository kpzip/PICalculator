//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod util;

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
// Halt on panic
use panic_halt as _; // panic handler

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use hal::gpio::alt::spi1::Nss;
use hal::spi::{Mode, Phase, Polarity};
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::spi::{BitFormat, Error, Flag, SpiSlave};

use embedded_alloc::LlffHeap as Heap;
use lib::parser::parse;
use crate::util::truncate_trailing_zeros;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
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

    let mut data: [u8; 2] = [0; 2];
    let mut sci_mode_text: String = String::new();

    loop {
        match spi.read(&mut data) {
            Ok(_) => {
                rprintln!("Received data: {:02X?}", data);
                if data[0] == 1 {
                    // Key press received
                    let key = data[1];
                    let c = match key {
                        13 => {
                            sci_mode_text.clear();
                            ready.set_high();
                            spi.write(&[0x01, 0x01]).unwrap();
                            ready.set_low();
                            ""
                        },
                        2 => ".",
                        3 => "0",
                        8 => "7",
                        9 => "8",
                        10 => "9",
                        14 => "4",
                        15 => "5",
                        16 => "6",
                        20 => "1",
                        21 => "2",
                        22 => "3",
                        29 => "+",
                        23 => "-",
                        17 => "*",
                        11 => "/",
                        5 => {
                            // Eval
                            let parsed = parse(sci_mode_text.as_str()).map(|v| truncate_trailing_zeros(format!("{:.8}", v.evaluate()))).unwrap_or(String::from("err"));
                            sci_mode_text = parsed;
                            ready.set_high();
                            // Reset Display
                            spi.write(&[0x81, 0x01]).unwrap();
                            for b in sci_mode_text.as_bytes().iter().copied() {
                                spi.write(&[0x82, b]).unwrap();
                            }
                            spi.write(&[0x00]).unwrap();
                            ready.set_low();
                            ""
                        }
                        _ => {
                            ready.set_high();
                            spi.write(&[0x00]).unwrap();
                            ready.set_low();
                            ""
                        },
                    };
                    sci_mode_text.push_str(c);
                    if c.len() > 0 {
                        ready.set_high();
                        if let Err(e) = spi.write(&[0x02, c.as_bytes()[0]]) {
                            rprintln!("Write error: {:?}", e);
                        }
                        ready.set_low();
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
