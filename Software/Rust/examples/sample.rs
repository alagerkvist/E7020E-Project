//Uses a local compiled sample (https://github.com/RustAudio/sample) with libm (https://github.com/rust-lang-nursery/libm)
//Not tested on a MCU yet, but does compile
#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate panic_halt;
extern crate sample;
extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use core::f64::consts::PI;
use libm::sin;
use cortex_m::{asm, iprintln};
use rtfm::{app};
use hal::spi::{Spi, Mode, Phase, Polarity};
use core::alloc::{Layout, GlobalAlloc};
use core::ptr::null_mut;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAllocator = MyAllocator;
use sample::{signal, Signal, I24, Sample, conv};


#[app(device = hal::stm32)]
const APP: () = {

    #[init]
    fn init() {
        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "hello sample, i hope this work");
        device.I2S2EXT.dr.write(|w|  w.dr().bits(0x7FFF));
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
        device.RCC.cfgr.modify(|_, w| {
            w.i2ssrc().plli2s()
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

        device.I2S2EXT.i2spr.modify(|_, w|{
            unsafe{
                w.mckoe().enabled()
                .i2sdiv().bits(0b10)
            }
        });
        device.I2S2EXT.i2scfgr.modify(|_, w| {
            w.i2se().enabled()
        });
        
        device.SPI2.i2scfgr.modify(|_, w| {
            w.i2se().disabled()
        });

        device.SPI2.i2scfgr.modify(|_, w| {
            w.i2smod().i2smode()
            .i2scfg().master_tx()
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
        //0x29 loopback
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


        let mut signal = signal::rate(33100.0).const_hz(440.0).sine();
        /*
        for _i in 0..33100 {
            let sample = signal.next();
            let u16_sample = u32::from_sample(sample[0]);
            let i24_sample = I24::from_sample(sample[0]);
            let i32_sample = i24_sample.inner();//conv::i24::to_i32(i24_sample);
            //let low = (( (i32_sample as u32) & 0xFF00) >> 8) as u16;
            //let high = (( (i32_sample as u32) & (0xFFFF0000 as u32)) >> 16) as u16;
            let low = u16_sample as u16;
            let high = (u16_sample & 0xFFFF0000 << 16) as u16; 
            while !device.I2S2EXT.sr.read().txe().bit_is_set() {}
            device.I2S2EXT.dr.write(|w| w.dr().bits(0xFF));
            while !device.I2S2EXT.sr.read().txe().bit_is_set() {}
            device.I2S2EXT.dr.write(|w| w.dr().bits(0xF0));
            
            
            //iprintln!(stim, "Org: {:?}\ni32: {:b}\nHigh: {:?}\nLow: {:?}", i24_sample, i32_sample, high, low);
            /*
            for _ in 0..1000 {
                cortex_m::asm::nop(); // no operation (cannot be optimized out)
            }
            */
        }
        */
        for i in 0..10 {
        for i in 1..33101{//33101{
            let sin = 8388607.0 * sin(2.0 * PI * 440.0 * (i as f64) / 33100.0);
            let rounded = sin as i32;
            //iprintln!(stim, "{:?} {:x}", sin, rounded);
            let high = (((rounded as u32) >> 16) & 0xFFFF) as u16;
            let low = ((rounded as u32) & 0xFF00) as u16;
            while !device.SPI2.sr.read().txe().bit_is_set() {}
            device.SPI2.dr.write(|w| unsafe { w.bits(rounded as u32) });
            //while !device.SPI2.sr.read().txe().bit_is_set() {}
            //device.SPI2.dr.write(|w| w.dr().bits(low));
            //iprintln!(stim, "{:x} {:x}", high, low);
        }
        }
    }

    #[idle]
    fn idle() -> ! {
        loop {
            asm::wfi();
        }
    }
};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}

