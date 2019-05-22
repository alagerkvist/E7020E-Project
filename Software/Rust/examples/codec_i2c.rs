#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
//extern crate stm32f4;
extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use cortex_m::{asm, iprintln};
use hal::i2c::{I2c};
//use hal::spi::{Spi, Mode, Phase, Polarity};
use rtfm::{app};

#[app(device = hal::stm32)]
const APP: () = {

    #[init]
    fn init() {
        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "hello codec");
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let gpioa = device.GPIOA.split();
        let _cs = gpioa.pa4.into_push_pull_output();
        let _mosi = gpioa.pa7.into_push_pull_output();
        // mosi.set_high();
        // cs.set_high();
        let gpiob = device.GPIOB.split();
        let scl = gpiob.pb8.into_alternate_af4();
        let sda = gpiob.pb9.into_alternate_af4();
        let mut i2c = I2c::i2c1(
            device.I2C1,
            (scl, sda),
            20.khz(),
            clocks,
        );

        let address = 0x91 as u8;
        let tx_buffer = [0x01];
        let mut rx_buffer: [u8; 32] = [0; 32];
        //i2c.write(address, tx_buffer);
        //let data = i2c.read();
        let data = i2c.write_read(address, &tx_buffer, &mut rx_buffer);
        match data {
            Ok(v) => {
                iprintln!(stim, "working with version: {:?}", v);
            },
            Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        iprintln!(stim, "{:?}", rx_buffer);
    }
    
    #[idle]
    fn idle() -> ! {
        loop {
            asm::wfi();
        }
    }

    //#[interrupt]

};