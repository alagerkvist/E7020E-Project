#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
//extern crate stm32f4;
extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use cortex_m::{asm, iprintln};
use hal::spi::{Spi, Mode, Phase, Polarity};
use rtfm::{app};

#[app(device = hal::stm32)]
const APP: () = {

    #[init]
    fn init() {
        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "hello codec");
        device.RCC.cr.modify(|_,w| {
            w.plli2son().on()
            .hseon().on()
        });

        device.RCC.plli2scfgr.modify(|_,w| {
            unsafe {
                w.plli2sr().bits(0b100)
                .plli2sq().bits(0b110)
                .plli2sn().bits(0b10010110)
            }
        });
        let mut rcc_cfgr = device.RCC.cfgr.read().bits();
        rcc_cfgr &= 0xFF7FFFFF;
        device.RCC.cfgr.modify(|_, w| {
            unsafe {
                w.bits(rcc_cfgr)
            }
        });
        device.RCC.apb1enr.modify(|_, w| {
            w.spi3en().enabled()
        });

        device.RCC.ahb1enr.modify(|_, w| {
            w.gpioben().enabled()
            .gpiocen().enabled()
        });

        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.freeze();
        
        let gpioa = device.GPIOA.split();
        let sck = gpioa.pa5.into_alternate_af5();
        let miso = gpioa.pa6.into_alternate_af5();
        let mosi = gpioa.pa7.into_alternate_af5();
        let mut cs = gpioa.pa4.into_push_pull_output();

        pub const MODE: Mode = Mode {
            polarity: Polarity::IdleHigh,
            phase: Phase::CaptureOnSecondTransition,
        };
        let mut spi = Spi::spi1(
            device.SPI1,
            (sck, miso, mosi),
            MODE,
            1_000_000.hz(),
            clocks
        );
        device.GPIOC.ospeedr.modify(|_, w| w.ospeedr6().very_high_speed());

        let gpioc = device.GPIOC.split();
        let _mclk = gpioc.pc6.into_alternate_af5();
        let gpiob = device.GPIOB.split();
        let _lrck = gpiob.pb12.into_alternate_af5();
        let _slck = gpiob.pb13.into_alternate_af5();
        let _sdin = gpiob.pb14.into_alternate_af6();
        let _sdout = gpiob.pb15.into_alternate_af5();
        device.SPI2.i2scfgr.modify(|_, w| {
            w.i2se().disabled()
        });

        device.SPI2.i2scfgr.modify(|_, w| {
            w.i2smod().i2smode()
            .i2scfg().master_rx()
            .i2sstd().msb()
            .datlen().twenty_four_bit()
            .chlen().thirty_two_bit()
            .ckpol().idle_high()
        });

        device.SPI2.i2spr.modify(|_, w|{
            w.mckoe().enabled()
        });
        device.SPI2.i2scfgr.modify(|_, w| {
            w.i2se().enabled()
        });

        cs.set_high();
        cs.set_low();
        cs.set_high();


        cs.set_low();
        let mut something = [0x9E, 0x04, 0x20];
        let  data = spi.transfer(&mut something);
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        cs.set_high();

        cs.set_low();
        let mut something = [0x9E, 0x06, 0x00];
        let  data = spi.transfer(&mut something);
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        cs.set_high();

        cs.set_low();
        let mut something = [0x9F, 0x04];
        let  data = spi.transfer(&mut something);
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        cs.set_high();






















        // let mut something = [0x9E, 0x02, 0x40];
        // //let data = spi.write(&[0x9E, 0x84, 0x20]);
        // let  data = spi.transfer(&mut something);
        // match data {
        //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        // cs.set_high();

        // cs.set_low();
        // let mut something = [0x9E, 0x83, 0x41];
        // //let data = spi.write(&[0x9E, 0x84, 0x20]);
        // let  data = spi.transfer(&mut something);
        // match data {
        //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        // cs.set_high();
        /*
        cs.set_low();
        let mut something = [0x9E, 0x02, 0x60];
        let  data = spi.transfer(&mut something);
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        cs.set_high();

        cs.set_low();
        let mut something = [0x9E, 0x01];
        let  data = spi.transfer(&mut something);
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        cs.set_high();

        cs.set_low();
        let mut something = [0x9F, 0x01];
        let  data = spi.transfer(&mut something);
        //let data2 = spi.read();
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        //println!(stim, "{:?}", data2);
        cs.set_high();

        cs.set_low();
        let mut something = [0x9E, 0x02];
        let  data = spi.transfer(&mut something);
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        cs.set_high();

        cs.set_low();
        let mut something = [0x9F, 0x02];
        let  data = spi.transfer(&mut something);
        //let data2 = spi.read();
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        //println!(stim, "{:?}", data2);
        cs.set_high();
        */
        // asm::bkpt();
        // cs.set_low();
        // for _ in 1..10000 {
        //     asm::nop();
        // }
        // let mut something = [0x9F, 0x01];
        // //let mut something = [0x9F, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x08];
        
        // //let mut something = [0x9F, 0x02];
        // //let data = spi.write(&[0x9E, 0x84, 0x20]);
        // let  data = spi.transfer(&mut something);
        // let mut number: &[u8] = &[0x1];
        // match data {
        //         Ok(v) => {
        //             iprintln!(stim, "working with version: {:?}", v);
        //             number = v;
        //         },
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        // cs.set_high();
        
        // asm::bkpt();
        // //cs.set_low();
        // // iprintln!(stim, "{:?}", number);
        // // iprintln!(stim, "{:?}", number[1] & 0x5C);
        // cs.set_low();
        // for _ in 1..10000 {
        //     asm::nop();
        // }
        // spi.write(&[0x9E, 0x02, 0x41]);
        // // let mut something = [0x9E];
        // // //let mut something = [0x9F, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x08];
        
        // // //let mut something = [0x9F, 0x02];
        // // //let data = spi.write(&[0x9E, 0x84, 0x20]);
        // // let  data = spi.transfer(&mut something);
        // // match data {
        // //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        // //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // // }
        // // let mut something = [0x02];
        // // //let mut something = [0x9F, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x08];
        
        // // //let mut something = [0x9F, 0x02];
        // // //let data = spi.write(&[0x9E, 0x84, 0x20]);
        // // let  data = spi.transfer(&mut something);
        // // match data {
        // //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        // //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // // }
        // // let mut something = [0x40];
        // // //let mut something = [0x9F, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x08];
        
        // // //let mut something = [0x9F, 0x02];
        // // //let data = spi.write(&[0x9E, 0x84, 0x20]);
        // // let  data = spi.transfer(&mut something);
        // // match data {
        // //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        // //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // // }
        // cs.set_high();

        // asm::bkpt();
        // cs.set_low();
        // for _ in 1..10000 {
        //     asm::nop();
        // }
        // //let mut something = [0x9F, 0x05];
        // let mut something = [0x9F, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x08];
        
        // //let mut something = [0x9F, 0x02];
        // //let data = spi.write(&[0x9E, 0x84, 0x20]);
        // let  data = spi.transfer(&mut something);
        // //let mut number: &[u8] = &[0x1];
        // match data {
        //         Ok(v) => {
        //             iprintln!(stim, "working with version: {:?}", v);
        //             //number = v;
        //         },
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        // cs.set_high();
                //     let sr = device.SPI3.sr.read();
        /*
        let mut done = false;
        while !done {
            let sr = device.SPI3.sr.read();
            if sr.rxne().bit_is_set() {
                done = true;
            }
        }
        let data = spi.read();
        */
        
        // let data = spi.write(&[0x04]);
        // match data {
        //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        // let data = spi.write(&[0x20]);
        // match data {
        //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        
        // cs.set_low();
        
        // let data = spi.write(&[0x9E, 0x06, 0x00]);
        // match data {
        //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        // cs.set_high();

        // cs.set_low();
        
        // let data = spi.write(&[0x9E, 0x08, 0x00]);
        // match data {
        //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        // cs.set_high();

        // cs.set_low();
        
        // let data = spi.write(&[0x9E, 0x08, 0x00]);
        // match data {
        //         Ok(v) => iprintln!(stim, "working with version: {:?}", v),
        //         Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        // }
        // cs.set_high();
        /*
        let gpiob = device.GPIOB.split();
        let lrck = gpiob.pb12.into_alternate_af5();
        let slck = gpiob.pb13.into_alternate_af5();
        let sdin = gpiob.pb14.into_alternate_af6();
        let sdout = gpiob.pb15.into_alternate_af5();

        let mut button4 = gpiob.pb4.into_pull_up_input();
        let mut button5 = gpiob.pb5.into_pull_up_input();
        

       
        // A full cycle, 16-bit, 2's complement Sine wave lookup table
        let sine: [u16; 256] = [
        0x0000, 0x0324, 0x0647, 0x096a, 0x0c8b, 0x0fab, 0x12c8, 0x15e2, 
        0x18f8, 0x1c0b, 0x1f19, 0x2223, 0x2528, 0x2826, 0x2b1f, 0x2e11,
        0x30fb, 0x33de, 0x36ba, 0x398c, 0x3c56, 0x3f17, 0x41ce, 0x447a, 
        0x471c, 0x49b4, 0x4c3f, 0x4ebf, 0x5133, 0x539b, 0x55f5, 0x5842,
        0x5a82, 0x5cb4, 0x5ed7, 0x60ec, 0x62f2, 0x64e8, 0x66cf, 0x68a6, 
        0x6a6d, 0x6c24, 0x6dca, 0x6f5f, 0x70e2, 0x7255, 0x73b5, 0x7504,
        0x7641, 0x776c, 0x7884, 0x798a, 0x7a7d, 0x7b5d, 0x7c29, 0x7ce3, 
        0x7d8a, 0x7e1d, 0x7e9d, 0x7f09, 0x7f62, 0x7fa7, 0x7fd8, 0x7ff6,
        0x7fff, 0x7ff6, 0x7fd8, 0x7fa7, 0x7f62, 0x7f09, 0x7e9d, 0x7e1d, 
        0x7d8a, 0x7ce3, 0x7c29, 0x7b5d, 0x7a7d, 0x798a, 0x7884, 0x776c,
        0x7641, 0x7504, 0x73b5, 0x7255, 0x70e2, 0x6f5f, 0x6dca, 0x6c24, 
        0x6a6d, 0x68a6, 0x66cf, 0x64e8, 0x62f2, 0x60ec, 0x5ed7, 0x5cb4,
        0x5a82, 0x5842, 0x55f5, 0x539b, 0x5133, 0x4ebf, 0x4c3f, 0x49b4, 
        0x471c, 0x447a, 0x41ce, 0x3f17, 0x3c56, 0x398c, 0x36ba, 0x33de,
        0x30fb, 0x2e11, 0x2b1f, 0x2826, 0x2528, 0x2223, 0x1f19, 0x1c0b, 
        0x18f8, 0x15e2, 0x12c8, 0x0fab, 0x0c8b, 0x096a, 0x0647, 0x0324,
        0x0000, 0xfcdc, 0xf9b9, 0xf696, 0xf375, 0xf055, 0xed38, 0xea1e, 
        0xe708, 0xe3f5, 0xe0e7, 0xdddd, 0xdad8, 0xd7da, 0xd4e1, 0xd1ef,
        0xcf05, 0xcc22, 0xc946, 0xc674, 0xc3aa, 0xc0e9, 0xbe32, 0xbb86, 
        0xb8e4, 0xb64c, 0xb3c1, 0xb141, 0xaecd, 0xac65, 0xaa0b, 0xa7be,
        0xa57e, 0xa34c, 0xa129, 0x9f14, 0x9d0e, 0x9b18, 0x9931, 0x975a, 
        0x9593, 0x93dc, 0x9236, 0x90a1, 0x8f1e, 0x8dab, 0x8c4b, 0x8afc,
        0x89bf, 0x8894, 0x877c, 0x8676, 0x8583, 0x84a3, 0x83d7, 0x831d, 
        0x8276, 0x81e3, 0x8163, 0x80f7, 0x809e, 0x8059, 0x8028, 0x800a,
        0x8000, 0x800a, 0x8028, 0x8059, 0x809e, 0x80f7, 0x8163, 0x81e3, 
        0x8276, 0x831d, 0x83d7, 0x84a3, 0x8583, 0x8676, 0x877c, 0x8894,
        0x89bf, 0x8afc, 0x8c4b, 0x8dab, 0x8f1e, 0x90a1, 0x9236, 0x93dc, 
        0x9593, 0x975a, 0x9931, 0x9b18, 0x9d0e, 0x9f14, 0xa129, 0xa34c,
        0xa57e, 0xa7be, 0xaa0b, 0xac65, 0xaecd, 0xb141, 0xb3c1, 0xb64c, 
        0xb8e4, 0xbb86, 0xbe32, 0xc0e9, 0xc3aa, 0xc674, 0xc946, 0xcc22,
        0xcf05, 0xd1ef, 0xd4e1, 0xd7da, 0xdad8, 0xdddd, 0xe0e7, 0xe3f5, 
        0xe708, 0xea1e, 0xed38, 0xf055, 0xf375, 0xf696, 0xf9b9, 0xfcdc,
        ];
        */
        // loop {
        //     let sr = device.SPI3.sr.read();
        //     let byte: u32 = 255;
        //     if sr.ovr().bit_is_set() {
        //         iprintln!(stim, "Ovr error!");
        //     } else if sr.txe().bit_is_set() {
        //         device.SPI3.dr.write(|w| unsafe{ w.bits(0x)});
        //         device.SPI3.dr.write(|w| unsafe{ w.bits(byte)});
        //     } else {
        //         iprintln!(stim, "Would block!");
        //     }

        // }
    }
    
    #[idle]
    fn idle() -> ! {
        loop {
            asm::wfi();
        }
    }

    //#[interrupt]

};