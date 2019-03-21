// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
//extern crate stm32f4;
extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use cortex_m::{asm, iprintln};
use hal::stm32::ITM;
use hal::stm32::ADC1;
// use crate::hal::stm32::Interrupt::EXTI0;
use rtfm::app;
// use hal::stm32::Interrupt::EXTI0;

#[app(device = hal::stm32)]
const APP: () = {
    static mut ITM: ITM = ();
        
    static mut ADC: ADC1 = ();

    //static mut adc_data: dr<hal::stm32::adc1> = ();
    // init runs in an interrupt free section
    #[init]
    fn init() {
        //let mut c = hal::stm32::CorePeripherals::take().unwrap();
        let stim = &mut core.ITM.stim[0];
        device.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit()); //Enable GPIOA clock
        device.RCC.apb2enr.modify(|_, w| w.adc1en().set_bit());
        device.ADC1.cr2.modify(|_, w| w.adon().clear_bit());
        device.ADC1.cr1.modify(|_, w| w.eocie().set_bit().scan().set_bit());
        device.ADC1.cr2.modify(|_, w|{
            w.align()
            .clear_bit()
            .cont()
            .clear_bit()
        //    .cont()
        //    .continuous()
            .eocs()
            .each_sequence()
        });
        device.ADC1.smpr2.modify(|_, w| w.smpx_x().cycles480());
        device.ADC1.cr2.modify(|_, w| w.adon().set_bit());
        //device.ADC1.sqr3.modify(|_, w| unsafe { w.sq1().bits(0 as u8) });
        device.ADC1.sqr3.modify(|_, w| unsafe { w.sq1().bits(2 as u8) });
        let l = device.ADC1.sqr1.read().l().bits();
        iprintln!(stim, "{:?}", l);
        device.ADC1.sqr1.modify(|_,w| w.l().bits(2));
        let gpioa = device.GPIOA.split();
        //gpioa.pa0.into_analog();
        //gpioa.pa1.into_analog();

        gpioa.pa2.into_analog();

        // device.GPIOA.afrl.modify(|_, w| w.afrl0().bits(0));
        // device.GPIOA.moder.modify(|_, w| w.moder0().bits(0b11)); //Analog mode
        // device.GPIOA.pupdr.modify(|_, w| unsafe { w.pupdr0().bits(0b00) });
        
        
        // device.GPIOA.moder.modify(|_, w| w.moder0().bits(0b11)); //Analog mode
        // device.GPIOA.pupdr.modify(|_, w| unsafe { w.pupdr0().bits(0b00) });
        iprintln!(stim, "pot");
        //device.ADC1.cr2.modify(|_, w| w);
        device.ADC1.cr2.modify(|_, w| w.swstart().set_bit());
        // loop {
        //     device.ADC1.cr2.modify(|_, w| w.swstart().set_bit());
        //     while !device.ADC1.sr.read().eoc().bit_is_set(){

        //     }
        //     let value = device.ADC1.dr.read().bits();
        //     device.ADC1.sr.modify(|_, w| w.eoc().clear_bit());
        //     iprintln!(stim, "val: {:?}", value);
        // }
        ADC = device.ADC1;
        ITM = core.ITM;
    }

    #[idle]
    fn idle() -> ! {
        loop {
            asm::wfi();
        }
    }

    // #[task(priority = 1, resources = [ITM])]
    // fn trace_data(byte: u32) {
    //     //asm::bkpt();
    //     let stim = &mut resources.ITM.stim[0];
    //     iprintln!(stim, "val: {}", byte);
    // }

    #[task(priority = 1, resources = [ITM])]
    fn change_freq(val: u32) {
        //let duty_cycle_percent: f32 = ((val as f32) * (80.0-20.0) / (4050.0) + 20.0);
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "val: {}", val);
    }

    #[interrupt(priority = 2, resources = [ADC], spawn = [change_freq])]
    fn ADC() {
        //let stim = &mut resources.ITM.stim[0];
        let value: u32 = resources.ADC.dr.read().bits();
        //iprintln!(stim, "val: {:?}", value);
        //if spawn.trace_data(value).is_;
        //spawn.trace_data(value);
        spawn.change_freq(value);
        resources.ADC.sr.modify(|_, w| w.eoc().clear_bit());
        resources.ADC.sr.modify(|_, w| w.ovr().clear_bit());
        resources.ADC.cr2.modify(|_, w| w.swstart().set_bit());         
    }

    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }

};
