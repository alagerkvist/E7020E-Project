#![no_main]
#![no_std]

extern crate panic_halt;
extern crate embedded_hal as hal;

//use hal::prelude::_embedded_hal_blocking_spi_Write;//blocking::spi::Write;
//use hal::spi::{Mode, Phase, Polarity};
//use hal::gpio::GpioExt;
////use stm32f4::stm32f413::{SPI1, SPI2, SPI3, GPIOA, GPIOB, RCC};
//
//pub const MODE: Mode = Mode {
//    polarity: Polarity::IdleLow,
//    phase: Phase::CaptureOnSecondTransition,
//};
//
//pub struct EA_DOGS<SPI, CS, DISP> {
//    spi: SPI,
//    cs: CS,
//    disp: DISP,
//    buffer: [[u8; 16]; 128],
//}
//
//
//impl<SPI, CS, DISP, E> EA_DOGS<SPI, CS, DISP>
//where
//    SPI: _embedded_hal_blocking_spi_Write<u8, Error = E>,
//    CS: GpioExt,
//    DISP: GpioExt,
//{
//    /// Create a new Ls010b7dh01 object
//    ///
//    /// `disp` is the pin connected to the display_enable pin of
//    /// the memory LCD.
//    pub fn new(spi: SPI, mut cs: CS, mut disp: DISP) -> Self {
//        disp.set_low();
//        cs.set_low();
//
//        let buffer = [[0; 16]; 128];
//
//        Self {
//            spi,
//            cs,
//            disp,
//            buffer,
//        }
//    }
//}