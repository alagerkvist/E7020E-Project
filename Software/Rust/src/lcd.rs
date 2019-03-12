#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
//extern crate stm32f4;
extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use cortex_m::{asm, iprintln};
use crate::hal::spi::{Spi, Mode, Phase, Polarity};
//use hal::stm32::ITM;
// use crate::hal::stm32::Interrupt::EXTI0;
use rtfm::app;
// use hal::stm32::Interrupt::EXTI0;

#[app(device = hal::stm32)]
const APP: () = {
    #[init]
    fn init() {
        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "lcd");
        let rcc = device.RCC.constrain();

        // 16 MHz (default, all clocks)
        let clocks = rcc.cfgr.freeze();

        let gpioc = device.GPIOC.split();
        let sck = gpioc.pc10.into_alternate_af6();
        let miso = gpioc.pc11.into_alternate_af6();
        let mosi = gpioc.pc12.into_alternate_af6();
        let gpioa = device.GPIOA.split();
        let mut reset = gpioc.pc0.into_push_pull_output();
        reset.set_high();
        let mut cs = gpioa.pa15.into_push_pull_output();
        cs.set_high();
        let mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };
        let mut spi = Spi::spi3(
            device.SPI3,
            (sck, miso, mosi),
            mode,
            10000000.hz(),
            clocks
        );
        // cs.set_low();
        // spi.write(&[0x40]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0xA1]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0xC0]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0xA4]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0xA6]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0xA2]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0x2F]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0x27]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0x81]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0x10]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0xFA]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0x90]);
        // cs.set_high();
        // cs.set_low();
        // spi.write(&[0xAF]);
        // cs.set_high();

        cs.set_low();
        spi.send(0x40);
        cs.set_high();
        cs.set_low();
        spi.send(0xA1);
        cs.set_high();
        cs.set_low();
        spi.send(0xC0);
        cs.set_high();
        cs.set_low();
        spi.send(0xA4);
        cs.set_high();
        cs.set_low();
        spi.send(0xA6);
        cs.set_high();
        cs.set_low();
        spi.send(0xA2);
        cs.set_high();
        cs.set_low();
        spi.send(0x2F);
        cs.set_high();
        cs.set_low();
        spi.send(0x27);
        cs.set_high();
        cs.set_low();
        spi.send(0x81);
        cs.set_high();
        cs.set_low();
        spi.send(0x10);
        cs.set_high();
        cs.set_low();
        spi.send(0xFA);
        cs.set_high();
        cs.set_low();
        spi.send(0x90);
        cs.set_high();
        cs.set_low();
        spi.send(0xAF);
        cs.set_high();
        // cs.set_low();
        // // //spi.write(0b10101111);
        // // const WRITE: u8 = 0 << 7;
        // // const MULTI: u8 = 1 << 6;
        // // const SINGLE: u8 = 0 << 6;
        // let buffer = [0xAF];
        // if spi.write(&buffer).is_err() {
        //      iprintln!(stim, "Error!");
        // }
        // cs.set_high();
        loop{
            if spi.write("a".as_bytes()).is_err() {
                iprintln!(stim, "Error!");
            }
            asm::bkpt();
        }
    }

    #[idle]
    fn idle() -> ! {
        loop {
            asm::wfi();
        }
    }

};