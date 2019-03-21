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
use hal::spi::{Spi, Mode, Phase, Polarity};
use hal::gpio::ExtiPin;
use rtfm::{app};

#[app(device = hal::stm32)]
const APP: () = {
    static mut ITM: ITM = ();
    static mut EXTI: EXTI = ();    

    #[init]
    fn init() {
        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "hello codec");
        device.RCC.ahb1enr.modify(|_, w| w.gpiocen().set_bit());
        device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        device.SYSCFG.exticr4.modify(|_, w| unsafe { w.exti13().bits(0b0010) });
        let gpioc = device.GPIOC.split();

        let button = gpioc.pc13.into_pull_up_input();

        device.EXTI.imr.modify(|_, w| w.mr13().set_bit());
        // Falling edge trigger
        device.EXTI.ftsr.modify(|_, w| w.tr13().set_bit());    

        ITM = core.ITM;
        EXTI = device.EXTI;
        //button.enable_interrupt();
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
    #[interrupt(resources = [ITM, EXTI])]
    fn EXTI15_10(){
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "EXTI4 {:?}", resources.EXTI.pr.read().pr12().bit());
        iprintln!(stim, "EXTI4 {:?}", resources.EXTI.pr.read().pr13().bit());
        resources.EXTI.pr.modify(|_, w| w.pr13().set_bit());
    }
};