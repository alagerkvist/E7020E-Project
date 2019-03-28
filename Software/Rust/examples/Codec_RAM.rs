#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
//extern crate stm32f4;
extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use cortex_m::{asm, iprintln};
use hal::stm32::ITM;
use hal::stm32::EXTI;
use hal::stm32::SPI2;
use hal::spi::{Spi, Mode, Phase, Polarity};
use hal::gpio::ExtiPin;
use rtfm::{app};

#[app(device = hal::stm32)]
const APP: () = {
    
    static mut ITM: ITM = ();
    static mut EXTI: EXTI = (); 
    static mut I2S: SPI2 = ();
    static mut BUF:[u32; 1000] = [0u32;1000]; 
    
    #[init]
    fn init() {
        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "hello codec");
        device.RCC.ahb1enr.modify(|_, w| w.gpiocen().set_bit());
        device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
        
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

        // buttons
        let gpiob = device.GPIOB.split();
        let mut button4 = gpiob.pb4.into_pull_up_input();
        let mut button5 = gpiob.pb5.into_pull_up_input();

        // button 4 interupt
        device.SYSCFG.exticr2.modify(|_, w| unsafe { w.exti5().bits(0b0001) });
        device.EXTI.imr.modify(|_, w| w.mr5().set_bit());
        // Falling edge trigge
        device.EXTI.ftsr.modify(|_, w| w.tr5().set_bit());    

        // button 5 interupt
        device.SYSCFG.exticr2.modify(|_, w| unsafe { w.exti4().bits(0b0001) });
        device.EXTI.imr.modify(|_, w| w.mr4().set_bit());
        // Falling edge trigge
        device.EXTI.ftsr.modify(|_, w| w.tr4().set_bit());    

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
    
        // disable auto mute
        cs.set_low();
        let mut something = [0x9E, 0x06, 0x00];
        let  data = spi.transfer(&mut something);
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        cs.set_high();

        cs.set_low();
        let mut something = [0x9F, 0x06];
        let  data = spi.transfer(&mut something);
        match data {
                Ok(v) => iprintln!(stim, "working with version: {:?}", v),
                Err(e) => iprintln!(stim, "error parsing header: {:?}", e),
        }
        cs.set_high();

        // set digital loopback
        cs.set_low();
        let mut something = [0x9E, 0x04, 0x00];
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
        
        asm::bkpt();

        ITM = core.ITM;
        EXTI = device.EXTI;
        I2S = device.SPI2;
        
        
    }
    #[idle]
    fn idle() -> ! {
        loop {
            asm::wfi();
        }
    }

    #[interrupt(resources = [ITM])]
    fn EXTI0(){
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "EXTI0");
    }
    #[interrupt(resources = [ITM])]
    fn EXTI1(){
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "EXTI1");
     }
    #[interrupt(resources = [ITM])]
    fn EXTI2(){
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "EXTI2");
    }
    #[interrupt(resources = [ITM])]
    fn EXTI3(){
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "EXTI3");
    }
    #[interrupt(resources = [ITM, EXTI, I2S, BUF])]
        //add a delay on BUF -> output send to codec.
    fn EXTI4(){
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "Reading Data");
        //read data from MISO            
        for index in 0..resources.BUF.len() {
            while !resources.I2S.sr.read().rxne().bit_is_set(){}
            let byte = resources.I2S.dr.read().bits();
            resources.BUF[index] = byte;
        }
        asm::bkpt();
        iprintln!(stim, "---------data-------");
        let index:u32;
        for index in 0..resources.BUF.len() {
            let mut a = resources.BUF[index];
            iprintln!(stim, "{:?}", a);
            for _ in 0..100{
                asm::nop();
            }  
        
        }
        resources.EXTI.pr.modify(|_, w| w.pr5().set_bit());      
    }
    

   #[interrupt(resources = [ITM, EXTI, I2S, BUF])]
    fn EXTI9_5(){
           let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "Sending Data"); 
        // delay
        let mut output: [u32; 4000] = [0; 4000];
            
        for index in 4000-(resources.BUF.len() as u32)..4000{
            let mut i = 0;
            output[index as usize] = resources.BUF[i];
            i += 1;
        }
        // data to send to codec
         asm::bkpt();
        loop {
            for _ in 0..10 {
                for i in 0..4000 {
                    while !resources.I2S.sr.read().txe().bit_is_set() {}
                    resources.I2S.dr.write(|w| unsafe{ w.bits(output[i])});
                }
            }
            asm::bkpt();
        }

        resources.EXTI.pr.modify(|_, w| w.pr5().set_bit()); 
    }
        
};


