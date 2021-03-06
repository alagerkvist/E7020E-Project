#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
//extern crate stm32f4;
extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use cortex_m::{asm, iprintln};
use crate::hal::prelude::_stm32f4xx_hal_rcc_RccExt;
use hal::spi::{Spi, Mode, Phase, Polarity};
use hal::stm32::i2s2ext;
    use crate::hal::gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;

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
                .plli2sm().bits(0b110)
                .plli2sn().bits(0b10000010)
            }
        });
        // let mut rcc_cfgr = device.RCC.cfgr.read().bits();
        // rcc_cfgr &= 0xFF7FFFFF;
        device.RCC.cfgr.modify(|_, w| {
            w.i2ssrc().plli2s()
            // unsafe {
            //     w.bits(rcc_cfgr)
            // }
        });
        device.RCC.apb1enr.modify(|_, w| {
            w.spi2en().enabled()
        });

        device.RCC.ahb1enr.modify(|_, w| {
            w.gpioben().enabled()
            .gpiocen().enabled()
        });
        
        device.RCC.cfgr.modify(|_, w|  { 
            w.mco2().sysclk().mco2pre().div4() 
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

        device.GPIOC.moder.modify(|_, w| w.moder9().alternate()); //bits(0b10));
        device.GPIOC.ospeedr.modify(|_, w| w.ospeedr9().very_high_speed()); // .bits(0b11));

        let gpioc = device.GPIOC.split();
        let _mclk = gpioc.pc6.into_alternate_af5();
        let gpiob = device.GPIOB.split();
        let _lrck = gpiob.pb12.into_alternate_af5();
        let _slck = gpiob.pb13.into_alternate_af5();
        let _sdin = gpiob.pb14.into_alternate_af6();
        let _sdout = gpiob.pb15.into_alternate_af5();

        device.I2S2EXT.i2spr.modify(|_, w| {
            unsafe{
                w.i2sdiv().bits(0b10)
            }
        });

        device.I2S2EXT.i2scfgr.modify(|_, w| {
            w.i2se().disabled()
        });

        device.I2S2EXT.i2scfgr.modify(|_, w| {
            w.i2smod().i2smode()
            .i2scfg().master_tx()
            .i2sstd().msb()
            .datlen().twenty_four_bit()
            .chlen().thirty_two_bit()
            .ckpol().idle_high()
        });

        device.I2S2EXT.i2scfgr.modify(|_, w| {
            w.i2se().enabled()
        });
        
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
            unsafe{
                w.mckoe().enabled()
                .i2sdiv().bits(0b10)
            }
        });
        device.SPI2.i2scfgr.modify(|_, w| {
            w.i2se().enabled()
        });
        
        
        cs.set_high();
        cs.set_low();
        cs.set_high();


        cs.set_low();
        let mut something = [0x9E, 0x04, 0x09];
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
        
        

        /*
        asm::bkpt();
        loop {
            let mut buf: [u32; 1000] = [0;1000];
            let mut zeros = 0;
            
            for i in 0..1000 {
                while !device.SPI2.sr.read().rxne().bit_is_set(){}
                let byte = device.SPI2.dr.read().bits();
                buf[i] = byte;
                /*
                let sr = device.SPI2.sr.read();
                if sr.ovr().bit_is_set() {
                    //iprintln!(stim, "Ovr error!");
                } else if sr.udr().bit_is_set() {
                    //iprintln!(stim, "udr error!");
                } else if sr.fre().bit_is_set() {
                    //iprintln!(stim, "fre error!");
                } else if sr.rxne().bit_is_set() {
                    let byte = device.SPI2.dr.read().bits();
                    buf[i] = byte;
                } else {
                    //iprintln!(stim, "Would block!");
                }
                */
            }
            iprintln!(stim, "---------new data-------");
            for i in 0..1000 {
                if buf[i] == 0{
                    zeros += 1;
                }
                iprintln!(stim, "{:?}", buf[i]);
                for _ in 0..100{
                    asm::nop();
                }  
            }
            iprintln!(stim, "COUNT ZEROS: {:?}", zeros);
            asm::bkpt();
                // for _ in 0..10_000{
                //     asm::nop();
                // }
        }
        */
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
        */

       
        // A full cycle, 16-bit, 2's complement Sine wave lookup table
        // let sine: [i32; 256] = [
        // 0x0000, 0x0324, 0x0647, 0x096a, 0x0c8b, 0x0fab, 0x12c8, 0x15e2, 
        // 0x18f8, 0x1c0b, 0x1f19, 0x2223, 0x2528, 0x2826, 0x2b1f, 0x2e11,
        // 0x30fb, 0x33de, 0x36ba, 0x398c, 0x3c56, 0x3f17, 0x41ce, 0x447a, 
        // 0x471c, 0x49b4, 0x4c3f, 0x4ebf, 0x5133, 0x539b, 0x55f5, 0x5842,
        // 0x5a82, 0x5cb4, 0x5ed7, 0x60ec, 0x62f2, 0x64e8, 0x66cf, 0x68a6, 
        // 0x6a6d, 0x6c24, 0x6dca, 0x6f5f, 0x70e2, 0x7255, 0x73b5, 0x7504,
        // 0x7641, 0x776c, 0x7884, 0x798a, 0x7a7d, 0x7b5d, 0x7c29, 0x7ce3, 
        // 0x7d8a, 0x7e1d, 0x7e9d, 0x7f09, 0x7f62, 0x7fa7, 0x7fd8, 0x7ff6,
        // 0x7fff, 0x7ff6, 0x7fd8, 0x7fa7, 0x7f62, 0x7f09, 0x7e9d, 0x7e1d, 
        // 0x7d8a, 0x7ce3, 0x7c29, 0x7b5d, 0x7a7d, 0x798a, 0x7884, 0x776c,
        // 0x7641, 0x7504, 0x73b5, 0x7255, 0x70e2, 0x6f5f, 0x6dca, 0x6c24, 
        // 0x6a6d, 0x68a6, 0x66cf, 0x64e8, 0x62f2, 0x60ec, 0x5ed7, 0x5cb4,
        // 0x5a82, 0x5842, 0x55f5, 0x539b, 0x5133, 0x4ebf, 0x4c3f, 0x49b4, 
        // 0x471c, 0x447a, 0x41ce, 0x3f17, 0x3c56, 0x398c, 0x36ba, 0x33de,
        // 0x30fb, 0x2e11, 0x2b1f, 0x2826, 0x2528, 0x2223, 0x1f19, 0x1c0b, 
        // 0x18f8, 0x15e2, 0x12c8, 0x0fab, 0x0c8b, 0x096a, 0x0647, 0x0324,
        // 0x0000, 0xfcdc, 0xf9b9, 0xf696, 0xf375, 0xf055, 0xed38, 0xea1e, 
        // 0xe708, 0xe3f5, 0xe0e7, 0xdddd, 0xdad8, 0xd7da, 0xd4e1, 0xd1ef,
        // 0xcf05, 0xcc22, 0xc946, 0xc674, 0xc3aa, 0xc0e9, 0xbe32, 0xbb86, 
        // 0xb8e4, 0xb64c, 0xb3c1, 0xb141, 0xaecd, 0xac65, 0xaa0b, 0xa7be,
        // 0xa57e, 0xa34c, 0xa129, 0x9f14, 0x9d0e, 0x9b18, 0x9931, 0x975a, 
        // 0x9593, 0x93dc, 0x9236, 0x90a1, 0x8f1e, 0x8dab, 0x8c4b, 0x8afc,
        // 0x89bf, 0x8894, 0x877c, 0x8676, 0x8583, 0x84a3, 0x83d7, 0x831d, 
        // 0x8276, 0x81e3, 0x8163, 0x80f7, 0x809e, 0x8059, 0x8028, 0x800a,
        // 0x8000, 0x800a, 0x8028, 0x8059, 0x809e, 0x80f7, 0x8163, 0x81e3, 
        // 0x8276, 0x831d, 0x83d7, 0x84a3, 0x8583, 0x8676, 0x877c, 0x8894,
        // 0x89bf, 0x8afc, 0x8c4b, 0x8dab, 0x8f1e, 0x90a1, 0x9236, 0x93dc, 
        // 0x9593, 0x975a, 0x9931, 0x9b18, 0x9d0e, 0x9f14, 0xa129, 0xa34c,
        // 0xa57e, 0xa7be, 0xaa0b, 0xac65, 0xaecd, 0xb141, 0xb3c1, 0xb64c, 
        // 0xb8e4, 0xbb86, 0xbe32, 0xc0e9, 0xc3aa, 0xc674, 0xc946, 0xcc22,
        // 0xcf05, 0xd1ef, 0xd4e1, 0xd7da, 0xdad8, 0xdddd, 0xe0e7, 0xe3f5, 
        // 0xe708, 0xea1e, 0xed38, 0xf055, 0xf375, 0xf696, 0xf9b9, 0xfcdc,
        // ];
        // 
        // const PI: f32 = 3.14159265358979323846;
        // const TAU: f32 = (2.0 * PI);
        // let sine: [i32; nSamples];
        // let mut i = 0;
        // while(i < nSamples) {
        //     let t: f32 = ((i as f32)/2.0)/(nSamples as f32);
        //     sine[i] = 3.14.sine();//8388601*sin(100.0 * t * TAU); // left
        //     sine[i+1] = sine[i]; // right
        //     i += 2;
        // }


        
        const nSamples: usize = 1000;
        //let sine: [i32; nSamples] = [0xAAABAB];
        let sine: [i32; nSamples] = [0x0,0x4b4d37,0x79c8ee,0x79a90e,0x4af9cb,0xff98f5,0xb45f8d,0x861781,0x867720,0xb559d3,0xce17,0x4bf37d,0x7a07c1,0x796863,0x4a5260,0xfecade,0xb3b9a9,0x85d94c,0x86b86a,0xb6019e,0x19c2d,0x4c98ff,0x7a4557,0x79267c,0x49aa35,0xfdfcca,0xb3148b,0x859c55,0x86faed,0xb6aa28,0x26a3e,0x4d3db9,0x7a81b0,0x78e35b,0x49014b,0xfd2ebc,0xb27034,0x85609a,0x873eab,0xb75371,0x33849,0x4de1ac,0x7abccc,0x789f01,0x4857a3,0xfc60b4,0xb1cca6,0x85261d,0x8783a1,0xb7fd77,0x4064c,0x4e84d5,0x7af6aa,0x78596e,0x47ad40,0xfb92b7,0xb129e3,0x84ecdf,0x87c9d0,0xb8a837,0x4d444,0x4f2732,0x7b2f48,0x7812a3,0x470224,0xfac4c4,0xb087ec,0x84b4e0,0x881137,0xb953b0,0x5a230,0x4fc8c2,0x7b66a8,0x77caa1,0x46564f,0xf9f6e0,0xafe6c4,0x847e21,0x8859d5,0xb9ffe0,0x6700d,0x506983,0x7b9cc7,0x778168,0x45a9c4,0xf9290b,0xaf466b,0x8448a1,0x88a3a8,0xbaacc6,0x73dd9,0x510973,0x7bd1a6,0x7736fa,0x44fc84,0xf85b47,0xaea6e3,0x841463,0x88eeb1,0xbb5a5f,0x80b93,0x51a892,0x7c0544,0x76eb56,0x444e91,0xf78d98,0xae082e,0x83e165,0x893aef,0xbc08aa,0x8d938,0x5246dd,0x7c37a1,0x769e7e,0x439fee,0xf6bffe,0xad6a4e,0x83afaa,0x898861,0xbcb7a6,0x9a6c6,0x52e452,0x7c68bb,0x765073,0x42f09b,0xf5f27c,0xaccd43,0x837f30,0x89d706,0xbd6750,0xa743b,0x5380f1,0x7c9893,0x760135,0x42409a,0xf52515,0xac3111,0x834ffa,0x8a26dd,0xbe17a7,0xb4195,0x541cb7,0x7cc728,0x75b0c5,0x418fee,0xf457c9,0xab95b7,0x832207,0x8a77e6,0xbec8a8,0xc0ed1,0x54b7a3,0x7cf47a,0x755f24,0x40de98,0xf38a9c,0xaafb39,0x82f557,0x8aca1f,0xbf7a53,0xcdbee,0x5551b4,0x7d2087,0x750c52,0x402c99,0xf2bd8f,0xaa6197,0x82c9ec,0x8b1d88,0xc02ca5,0xda8ea,0x55eae7,0x7d4b50,0x74b852,0x3f79f4,0xf1f0a5,0xa9c8d3,0x829fc5,0x8b7220,0xc0df9c,0xe75c3,0x56833c,0x7d74d5,0x746322,0x3ec6ab,0xf123df,0xa930ee,0x8276e3,0x8bc7e7,0xc19337,0xf4276,0x571ab0,0x7d9d14,0x740cc5,0x3e12bf,0xf0573f,0xa899ea,0x824f47,0x8c1eda,0xc24774,0x100f02,0x57b142,0x7dc40d,0x73b53b,0x3d5e32,0xef8ac8,0xa803c9,0x8228f0,0x8c76fa,0xc2fc50,0x10db64,0x5846f2,0x7de9c1,0x735c85,0x3ca906,0xeebe7c,0xa76e8d,0x8203e0,0x8cd046,0xc3b1cb,0x11a79a,0x58dbbc,0x7e0e2e,0x7302a4,0x3bf33c,0xedf25c,0xa6da35,0x81e016,0x8d2abc,0xc467e3,0x1273a2,0x596fa0,0x7e3154,0x72a799,0x3b3cd8,0xed266b,0xa646c5,0x81bd94,0x8d865c,0xc51e94,0x133f7b,0x5a029c,0x7e5333,0x724b64,0x3a85d9,0xec5aab,0xa5b43d,0x819c58,0x8de325,0xc5d5df,0x140b22,0x5a94af,0x7e73cb,0x71ee08,0x39ce43,0xeb8f1e,0xa522a0,0x817c65,0x8e4115,0xc68dc0,0x14d694,0x5b25d7,0x7e931a,0x718f84,0x391617,0xeac3c6,0xa491ee,0x815db9,0x8ea02d,0xc74636,0x15a1d1,0x5bb612,0x7eb122,0x712fd9,0x385d57,0xe9f8a5,0xa40229,0x814055,0x8f006a,0xc7ff3f,0x166cd6,0x5c4560,0x7ecde1,0x70cf09,0x37a405,0xe92dbe,0xa37353,0x81243b,0x8f61cc,0xc8b8da,0x1737a0,0x5cd3bf,0x7ee958,0x706d15,0x36ea23,0xe86311,0xa2e56c,0x810969,0x8fc453,0xc97304,0x18022f,0x5d612d,0x7f0385,0x7009fd,0x362fb2,0xe798a1,0xa25877,0x80efe0,0x9027fc,0xca2dbb,0x18cc7f,0x5deda9,0x7f1c69,0x6fa5c3,0x3574b5,0xe6ce71,0xa1cc75,0x80d7a0,0x908cc7,0xcae8fd,0x19968f,0x5e7931,0x7f3404,0x6f4067,0x34b92e,0xe60482,0xa14167,0x80c0ab,0x90f2b3,0xcba4ca,0x1a605c,0x5f03c5,0x7f4a55,0x6ed9eb,0x33fd1d,0xe53ad7,0xa0b74e,0x80aaff,0x9159bf,0xcc611e,0x1b29e5,0x5f8d62,0x7f5f5c,0x6e7250,0x334086,0xe47171,0xa02e2c,0x80969d,0x91c1ea,0xcd1df7,0x1bf328,0x601607,0x7f7318,0x6e0996,0x32836a,0xe3a852,0x9fa603,0x808385,0x922b32,0xcddb55,0x1cbc22,0x609db4,0x7f858b,0x6d9fbf,0x31c5cb,0xe2df7d,0x9f1ed4,0x8071b8,0x929597,0xce9935,0x1d84d2,0x612465,0x7f96b2,0x6d34cc,0x3107ab,0xe216f3,0x9e98a0,0x806136,0x930118,0xcf5795,0x1e4d35,0x61aa1c,0x7fa68f,0x6cc8bd,0x30490c,0xe14eb7,0x9e1368,0x8051ff,0x936db4,0xd01673,0x1f1549,0x622ed4,0x7fb521,0x6c5b95,0x2f89f0,0xe086ca,0x9d8f2e,0x804412,0x93db69,0xd0d5cd,0x1fdd0e,0x62b28f,0x7fc268,0x6bed53,0x2eca58,0xdfbf2f,0x9d0bf4,0x803771,0x944a36,0xd195a1,0x20a47f,0x633549,0x7fce64,0x6b7dfa,0x2e0a47,0xdef7e8,0x9c89ba,0x802c1b,0x94ba1b,0xd255ee,0x216b9c,0x63b703,0x7fd914,0x6b0d8b,0x2d49bf,0xde30f6,0x9c0882,0x802210,0x952b15,0xd316b1,0x223262,0x6437b9,0x7fe279,0x6a9c05,0x2c88c1,0xdd6a5c,0x9b884d,0x801951,0x959d25,0xd3d7e9,0x22f8cf,0x64b76c,0x7fea92,0x6a296c,0x2bc750,0xdca41c,0x9b091c,0x8011dd,0x961048,0xd49993,0x23bee2,0x65361a,0x7ff160,0x69b5bf,0x2b056e,0xdbde37,0x9a8af1,0x800bb5,0x96847f,0xd55bad,0x248499,0x65b3c2,0x7ff6e2,0x694100,0x2a431c,0xdb18b0,0x9a0dcd,0x8006d9,0x96f9c6,0xd61e36,0x2549f0,0x663062,0x7ffb19,0x68cb30,0x29805c,0xda5388,0x9991b2,0x800348,0x97701e,0xd6e12c,0x260ee7,0x66abf9,0x7ffe03,0x685451,0x28bd31,0xd98ec2,0x9916a0,0x800104,0x97e785,0xd7a48c,0x26d37b,0x672685,0x7fffa2,0x67dc63,0x27f99c,0xd8ca60,0x989c99,0x80000b,0x985ff9,0xd86855,0x2797ab,0x67a007,0x7ffff5,0x676367,0x2735a0,0xd80664,0x98239d,0x80005e,0x98d97b,0xd92c85,0x285b74,0x68187b,0x7ffefc,0x66e960,0x26713e,0xd742cf,0x97abaf,0x8001fd,0x995407,0xd9f119,0x291ed4,0x688fe2,0x7ffcb8,0x666e4e,0x25ac78,0xd67fa4,0x9734d0,0x8004e7,0x99cf9e,0xdab610,0x29e1ca,0x69063a,0x7ff927,0x65f233,0x24e750,0xd5bce4,0x96bf00,0x80091e,0x9a4c3e,0xdb7b67,0x2aa453,0x697b81,0x7ff44b,0x65750f,0x2421c9,0xd4fa92,0x964a41,0x800ea0,0x9ac9e6,0xdc411e,0x2b666d,0x69efb8,0x7fee23,0x64f6e4,0x235be4,0xd438b0,0x95d694,0x80156e,0x9b4894,0xdd0731,0x2c2817,0x6a62db,0x7fe6af,0x6477b3,0x2295a4,0xd3773f,0x9563fb,0x801d87,0x9bc847,0xddcd9e,0x2ce94f,0x6ad4eb,0x7fddf0,0x63f77e,0x21cf0a,0xd2b641,0x94f275,0x8026ec,0x9c48fd,0xde9464,0x2daa12,0x6b45e5,0x7fd3e5,0x637646,0x210818,0xd1f5b9,0x948206,0x80319c,0x9ccab7,0xdf5b81,0x2e6a5f,0x6bb5ca,0x7fc88f,0x62f40c,0x2040d1,0xd135a8,0x9412ad,0x803d98,0x9d4d71,0xe022f2,0x2f2a33,0x6c2497,0x7fbbee,0x6270d2,0x1f7936,0xd07610,0x93a46b,0x804adf,0x9dd12c,0xe0eab7,0x2fe98d,0x6c924c,0x7fae01,0x61ec98,0x1eb149,0xcfb6f4,0x933743,0x805971,0x9e55e4,0xe1b2cb,0x30a86b,0x6cfee8,0x7f9eca,0x616760,0x1de90d,0xcef855,0x92cb34,0x80694e,0x9edb9b,0xe27b2e,0x3166cb,0x6d6a69,0x7f8e48,0x60e12c,0x1d2083,0xce3a35,0x926041,0x807a75,0x9f624c,0xe343de,0x3224ab,0x6dd4ce,0x7f7c7b,0x6059fd,0x1c57ae,0xcd7c96,0x91f66a,0x808ce8,0x9fe9f9,0xe40cd8,0x32e209,0x6e3e16,0x7f6963,0x5fd1d4,0x1b8e8f,0xccbf7a,0x918db0,0x80a0a4,0xa0729e,0xe4d61b,0x339ee2,0x6ea641,0x7f5501,0x5f48b2,0x1ac529,0xcc02e3,0x912615,0x80b5ab,0xa0fc3b,0xe59fa4,0x345b36,0x6f0d4d,0x7f3f55,0x5ebe99,0x19fb7e,0xcb46d2,0x90bf99,0x80cbfc,0xa186cf,0xe66971,0x351703,0x6f7339,0x7f2860,0x5e338b,0x19318f,0xca8b4b,0x905a3d,0x80e397,0xa21257,0xe73381,0x35d245,0x6fd804,0x7f1020,0x5da789,0x18675f,0xc9d04e,0x8ff603,0x80fc7b,0xa29ed3,0xe7fdd1,0x368cfc,0x703bad,0x7ef697,0x5d1a94,0x179cef,0xc915dd,0x8f92eb,0x8116a8,0xa32c41,0xe8c860,0x374726,0x709e34,0x7edbc5,0x5c8cad,0x16d242,0xc85bfb,0x8f30f7,0x81321f,0xa3baa0,0xe9932a,0x3800c1,0x70ff96,0x7ebfab,0x5bfdd7,0x16075b,0xc7a2a9,0x8ed027,0x814ede,0xa449ee,0xea5e2f,0x38b9ca,0x715fd3,0x7ea247,0x5b6e12,0x153c3a,0xc6e9e9,0x8e707c,0x816ce6,0xa4da29,0xeb296c,0x397240,0x71beeb,0x7e839b,0x5add60,0x1470e2,0xc631bd,0x8e11f8,0x818c35,0xa56b51,0xebf4de,0x3a2a21,0x721cdb,0x7e63a8,0x5a4bc3,0x13a555,0xc57a27,0x8db49c,0x81accd,0xa5fd64,0xecc085,0x3ae16c,0x7279a4,0x7e426c,0x59b93b,0x12d995,0xc4c328,0x8d5867,0x81ceac,0xa69060,0xed8c5e,0x3b981d,0x72d544,0x7e1fea,0x5925cb,0x120da4,0xc40cc4,0x8cfd5c,0x81f1d2,0xa72444,0xee5866,0x3c4e35,0x732fba,0x7dfc20,0x589173,0x114184,0xc356fa,0x8ca37b,0x82163f,0xa7b90e,0xef249c,0x3d03b0,0x738906,0x7dd710,0x57fc37,0x107538,0xc2a1ce,0x8c4ac5,0x823bf3,0xa84ebe,0xeff0fe,0x3db88c,0x73e126,0x7db0b9,0x576616,0xfa8c1,0xc1ed41,0x8bf33b,0x8262ec,0xa8e550,0xf0bd8a,0x3e6cc9,0x743819,0x7d891d,0x56cf12,0xedc21,0xc13955,0x8b9cde,0x828b2b,0xa97cc4,0xf18a3d,0x3f2064,0x748de0,0x7d603b,0x56372d,0xe0f5b,0xc0860c,0x8b47ae,0x82b4b0,0xaa1519,0xf25716,0x3fd35b,0x74e278,0x7d3614,0x559e69,0xd4271,0xbfd367,0x8af3ae,0x82df79,0xaaae4c,0xf32412,0x4085ad,0x7535e1,0x7d0aa9,0x5504c7,0xc7564,0xbf2168,0x8aa0dc,0x830b86,0xab485d,0xf3f12f,0x413758,0x75881a,0x7cddf9,0x546a49,0xba837,0xbe7012,0x8a4f3b,0x8338d8,0xabe349,0xf4be6b,0x41e859,0x75d923,0x7cb006,0x53ceef,0xadaeb,0xbdbf66,0x89fecb,0x83676d,0xac7f0f,0xf58bc5,0x4298b0,0x7628fa,0x7c80d0,0x5332bd,0xa0d84,0xbd0f65,0x89af8d,0x839745,0xad1bae,0xf6593a,0x43485a,0x76779f,0x7c5056,0x5295b2,0x94002,0xbc6012,0x896182,0x83c85f,0xadb923,0xf726c8,0x43f756,0x76c511,0x7c1e9b,0x51f7d2,0x87268,0xbbb16f,0x8914aa,0x83fabc,0xae576e,0xf7f46d,0x44a5a1,0x77114f,0x7beb9d,0x51591d,0x7a4b9,0xbb037c,0x88c906,0x842e5a,0xaef68d,0xf8c227,0x45533a,0x775c58,0x7bb75f,0x50b995,0x6d6f5,0xba563c,0x887e98,0x846339,0xaf967d,0xf98ff3,0x460020,0x77a62b,0x7b81df,0x50193c,0x60920,0xb9a9b1,0x88355f,0x849958,0xb0373e,0xfa5dd0,0x46ac50,0x77eec9,0x7b4b20,0x4f7814,0x53b3c,0xb8fddc,0x87ed5d,0x84d0b8,0xb0d8ce,0xfb2bbc,0x4757c9,0x783630,0x7b1321,0x4ed61d,0x46d49,0xb852c0,0x87a692,0x850956,0xb17b2b,0xfbf9b4,0x480289,0x787c5f,0x7ad9e3,0x4e335a,0x39f4c,0xb7a85d,0x8760ff,0x854334,0xb21e54,0xfcc7b7,0x48ac8f,0x78c155,0x7a9f66,0x4d8fcc,0x2d144,0xb6feb5,0x871ca5,0x857e50,0xb2c247,0xfd95c2,0x4955d8,0x790513,0x7a63ab,0x4ceb75,0x20336,0xb655cb,0x86d984,0x85baa9,0xb36701,0xfe63d3,0x49fe62,0x794796,0x7a26b4,0x4c4657,0x13522,0xb5ada0,0x86979d,0x85f83f,0xb40c83,0xff31e9,0x4aa62d,0x7988e0,0x79e87f,0x4ba073,0x670b,0xb50635,0x8656f2,0x863712,0xb4b2c9,0x0];
        asm::bkpt();
        loop {
            for _ in 0..10 {
                for i in 0..8388608 {
                    let low = ((i & 0xFF) << 8) as u16;
                    let high = ((i & 0xFFFF00) >> 8) as u16;
                    while !device.SPI2.sr.read().txe().bit_is_set() {}
                    device.I2S2EXT.dr.write(|w| unsafe { w.dr().bits(high)});
                    while !device.SPI2.sr.read().txe().bit_is_set() {}
                    device.I2S2EXT.dr.write(|w| unsafe { w.dr().bits(low)});
                    
                }
                asm::bkpt();
            }
        }

        loop {
            for _ in 0..10 {
                for i in 0..nSamples {
                    //let low = ((sine[i] & 0xFF) << 8) as u16;
                    //let high = ((sine[i] & 0xFFFF00) >> 8) as u16;
                    /*
                    while !device.SPI2.sr.read().txe().bit_is_set() {}
                    //device.SPI2.dr.write(|w| unsafe { w.dr().bits(0xAAAA)});
                    device.SPI2.dr.write(|w| unsafe { w.dr().bits(high)});
                    while !device.SPI2.sr.read().txe().bit_is_set() {}
                    //device.SPI2.dr.write(|w| unsafe { w.dr().bits(0xBA00)});
                    
                    device.SPI2.dr.write(|w| unsafe { w.dr().bits(low)});
                    */
                    while !device.I2S2EXT.sr.read().txe().bit_is_set() {}
                    //device.I2S2EXT.dr.write(|w| unsafe { w.dr().bits(high)});
                    device.I2S2EXT.dr.write(|w| unsafe { w.dr().bits(0x7FFF)});
                    while !device.I2S2EXT.sr.read().txe().bit_is_set() {}
                    device.I2S2EXT.dr.write(|w| unsafe { w.dr().bits(0x01)});
                    //device.I2S2EXT.dr.write(|w| unsafe { w.dr().bits(low)});
                }
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

    //#[interrupt]

};