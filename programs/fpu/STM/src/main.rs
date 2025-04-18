//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};
use hal::gpio::alt::spi1::Nss;
use hal::spi::{Mode, Phase, Polarity};
use stm32f4xx_hal::spi::{BitFormat, Error, Flag, SpiSlave};
use rtt_target::{rprintln, rtt_init_print};
// use stm32f4xx_hal::adc::config::Sequence::Eight;
// use stm32f4xx_hal::gpio::Pin;
// use stm32f4xx_hal::pac::rcc::cfgr::HPRE::Div2;
// use stm32f4xx_hal::pac::SPI1;
// use stm32f4xx_hal::pac::spi1::cr1::BIDIMODE::Unidirectional;
// use stm32f4xx_hal::serial::CFlag;
// use stm32f4xx_hal::spi::Flag::TxEmpty;
//use stm32ral::{read_reg, write_reg, modify_reg, reset_reg};
//use stm32ral::{gpio, spi, rcc};
// use stm32ral::gpio::AFRL::AFRL5::RW::AF1;
// #[inline(never)]
// fn dostuff(spi: &mut SpiSlave<SPI1>) {
//     let mut data: [u8; 2] = [0; 2];
//     let mut buff: [u8; 1] = [0x02];
//
//     match spi.read(&mut data) {
//         Ok(_) => {
//             rprintln!("Received data: {:02X?}", data);
//             //ready.set_high();
//             let a = spi.is_tx_empty();
//             if let Err(e) = spi.write(&mut buff) {
//                 rprintln!("Write error: {:?}", e);
//             }
//             //ready.set_low();
//             rprintln!("{} {} {}", spi.is_rx_not_empty(), spi.is_tx_empty(), a);
//         },
//         Err(e) => {
//             rprintln!("Error: {:?}", e);
//         },
//     }
// }

#[entry]
fn main() -> ! {

    rtt_init_print!();

    //let mut gpioa = gpio::GPIOA::take().unwrap();
    //let rcc = rcc::RCC::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // modify_reg!(rcc, rcc, CR, HSION: On, PLLON: On);
    // modify_reg!(rcc, rcc, PLLCFGR, PLLM: 8, PLLN: 84, PLLP: 2, PLLSRC: HSI);
    // modify_reg!(rcc, rcc, CFGR, SW: PLL, HPRE: Div2, PPRE1: Div2, PPRE2: Div1);
    // modify_reg!(rcc, rcc, AHB1ENR, GPIOAEN: Enabled);
    // modify_reg!(rcc, rcc, APB2ENR, SPI1EN: Enabled);

    unsafe {
        // dp.RCC.cr().write(|w| { w.bits(50360195) });
        // dp.RCC.pllcfgr().write(|w| { w.bits(67114248) });
        // dp.RCC.cfgr().write(|w| { w.bits(4234) });

        dp.RCC.ahb1enr().write(|w| { w.bits(3) });
        dp.RCC.ahb1lpenr().write(|w| { w.bits(6394015) });
        dp.RCC.ahb1rstr().write(|w| { w.bits(0) });
        dp.RCC.ahb2enr().write(|w| { w.bits(0) });
        dp.RCC.ahb2lpenr().write(|w| { w.bits(128) });
        dp.RCC.ahb1rstr().write(|w| { w.bits(0) });

        dp.RCC.apb1enr().write(|w| { w.bits(268435456) });
        dp.RCC.apb1lpenr().write(|w| { w.bits(283297807) });
        dp.RCC.apb1rstr().write(|w| { w.bits(0) });
        dp.RCC.apb2enr().write(|w| { w.bits(20480) });
        dp.RCC.apb2lpenr().write(|w| { w.bits(489777) });
        dp.RCC.apb1rstr().write(|w| { w.bits(0) });

        dp.GPIOA.afrl().write(|w| { w.bits(1431633920) });
        dp.GPIOA.moder().write(|w| { w.bits(2818615824) });
        dp.GPIOA.ospeedr().write(|w| { w.bits(201391872) });
        dp.GPIOA.otyper().modify(|_, w| w.ot2().clear_bit());

        dp.SPI1.cr1().write(|w| { w.bits(67) });
        dp.SPI1.cr2().write(|w| { w.bits(0) });
    }

    let mut data: [u8; 2] = [0; 2];

    rprintln!("TX Empty: {}", dp.SPI1.sr().read().txe().bit());

    loop {
        while dp.SPI1.sr().read().rxne().bit_is_clear() {}
        data[0] = dp.SPI1.dr8().read().bits();
        while dp.SPI1.sr().read().rxne().bit_is_clear() {}
        data[1] = dp.SPI1.dr8().read().bits();
        rprintln!("Got data: {:02X?}", data);
        rprintln!("TX Empty: {}", dp.SPI1.sr().read().txe().bit());

        dp.SPI1.dr8().write(|w| unsafe { w.bits(0x02) });
        rprintln!("TX Empty: {}", dp.SPI1.sr().read().txe().bit());

        dp.GPIOA.odr().write(|w| { w.odr2().set_bit() });

        while dp.SPI1.sr().read().txe().bit_is_clear() {}
        dp.SPI1.dr8().read();

        dp.SPI1.dr8().write(|w| unsafe { w.bits('A' as u8) });
        while dp.SPI1.sr().read().txe().bit_is_clear() {}
        dp.SPI1.dr8().read();
        rprintln!("Wrote data");
        dp.GPIOA.odr().write(|w| { w.odr2().clear_bit() });
    }
/*
    // write_reg!(gpio, gpioa, AFRL, 0b0101_0101_0101_0101_0000_0000_0000_0000);
    // modify_reg!(gpio, gpioa, MODER, MODER4: Alternate, MODER5: Alternate, MODER6: Alternate, MODER7: Alternate);
    // write_reg!(gpio, gpioa, OSPEEDR, 0xffffffff);

    let mut data: [u8; 2] = [0; 2];

    //let spi1 = spi::SPI1::take().unwrap();
    // modify_reg!(spi, spi1, CR1, CPHA: SecondEdge, CPOL: IdleHigh,
    //     MSTR: Slave, BR: Div2, SPE: Enabled, LSBFIRST: MSBFirst,
    //     SSM: Disabled, RXONLY: FullDuplex, DFF: EightBit, BIDIMODE: Unidirectional);
    write_reg!(spi, spi1, CR1, 3);
    write_reg!(spi, spi1, CR2, 0);

    while read_reg!(spi, spi1, SR, RXNE == 0) {
        //rprintln!("{}", read_reg!(spi, spi1, SR));
    }
    rprintln!("here");
    data[0] = read_reg!(spi, spi1, DR) as u8;
    while read_reg!(spi, spi1, SR, RXNE == 0) {}
    data[1] = read_reg!(spi, spi1, DR) as u8;

    rprintln!("Read data: {:02X?}", data);
    //ready.set_high();
    write_reg!(spi, spi1, DR, 2);
    while read_reg!(spi, spi1, SR, TXE == 0) {}
 */
    //ready.set_low();
    //
    // if let (Some(dp), Some(cp)) = (
    //     pac::Peripherals::take(),
    //     cortex_m::peripheral::Peripherals::take(),
    // ) {
    //     // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
    //     let gpioa = dp.GPIOA.split();
    //     //let mut led = gpioa.pa5.into_push_pull_output();
    //
    //     // Set up the system clock. We want to run at 48MHz for this one.
    //     let rcc = dp.RCC.constrain();
    //     let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
    //
    //     // Create a delay abstraction based on SysTick
    //     let mut delay = cp.SYST.delay(&clocks);
    //
    //     let mut ready = gpioa.pa2.into_push_pull_output();
    //     ready.set_low();
    //
    //     let sck = gpioa.pa5.into_alternate::<5>();
    //     let mut miso = gpioa.pa6.into_alternate::<5>();
    //     let mosi = gpioa.pa7.into_alternate::<5>();
    //     let nss = gpioa.pa4.into_alternate::<5>();
    //
    //     // let mut spi = dp.SPI1.spi_slave(
    //     //     (sck, miso, mosi, Some(Nss::from(nss))),
    //     //     Mode {
    //     //         phase: Phase::CaptureOnSecondTransition,
    //     //         polarity: Polarity::IdleHigh,
    //     //     }
    //     // );
    //
    //     rprintln!("Initialized SPI");
    //
    //     // dostuff(&mut spi);
    // }

    loop {}


}
