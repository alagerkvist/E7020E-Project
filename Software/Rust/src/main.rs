// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
//extern crate stm32f4;
extern crate stm32f4xx_hal as hal;
use cortex_m::{asm, iprintln};
use hal::stm32::ITM;
use hal::stm32::ADC1;
use stm32f4::stm32f413::GPIOB;
use rtfm::{app, Instant};

#[app(device = hal::stm32)]
const PERIOD: u32 = 4_000_000;


const APP: () = {
    static mut GPIOB: GPIOB = ();
    static mut Duty_cycle: f32 = 0.5; // Duty range = [20 80]%
    static mut Pwm_on: u32 = 0;
    static mut Pwm_off: u32 = 0;
    static mut ITM: ITM = ();    
    static mut ADC: ADC1 = ();

    //static mut adc_data: dr<hal::stm32::adc1> = ();
    // init runs in an interrupt free section
    #[init(schedule = [on], resources=[Pwm_on, Pwm_off, Duty_cycle])]
    fn init() {
        //let mut c = hal::stm32::CorePeripherals::take().unwrap();
        let stim = &mut core.ITM.stim[0];
        device.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit()); //Enable GPIOA clock
        device.RCC.apb2enr.modify(|_, w| w.adc1en().set_bit());
        device.ADC1.cr2.modify(|_, w| w.adon().clear_bit());
        device.ADC1.cr1.modify(|_, w| w.eocie().set_bit());
        device.ADC1.cr2.modify(|_, w|{
            w.align()
            .clear_bit()
            .cont()
            .clear_bit()
        //    .cont()
        //    .continuous()
        });
        device.ADC1.smpr2.modify(|_, w| w.smpx_x().cycles480());
        device.ADC1.cr2.modify(|_, w| w.adon().set_bit());
        device.ADC1.sqr3.modify(|_, w| unsafe { w.sq1().bits(0 as u8) });
        device.GPIOA.afrl.modify(|_, w| w.afrl0().bits(0));
        device.GPIOA.moder.modify(|_, w| w.moder0().bits(0b11)); //Analog mode
        device.GPIOA.pupdr.modify(|_, w| unsafe { w.pupdr0().bits(0b00) });
        iprintln!(stim, "main");
        //device.ADC1.cr2.modify(|_, w| w);
        device.ADC1.cr2.modify(|_, w| w.swstart().set_bit());
        *resources.Pwm_on = (PERIOD as f32 * *resources.Duty_cycle) as u32;
        *resources.Pwm_off = PERIOD - *resources.Pwm_on;
        //schedule.foo(Instant::now() + PERIOD.cycles()).unwrap();
    
        // power on GPIOA, RM0368 6.3.11
        device.RCC.ahb1enr.modify(|_, w| w.gpioben().set_bit());
        // configure PA5 as output, RM0368 8.4.1
        device.GPIOB.moder.modify(|_, w| w.moder1().bits(1));

        // pass on late resources

        schedule.on(Instant::now());
        GPIOB = device.GPIOB;
        ADC = device.ADC1;
        ITM = core.ITM;
    }

    #[idle]
    fn idle() -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(priority = 3, schedule = [off], resources=[Pwm_on, GPIOB])]
    fn on() {
        resources.GPIOB.bsrr.write(|w| w.bs1().set_bit());
        let a = *resources.Pwm_on;
        schedule.off(scheduled + a.cycles()).unwrap();
    }

    #[task(priority = 3, schedule = [on], resources=[Pwm_off, GPIOB])]
    fn off() {
        resources.GPIOB.bsrr.write(|w| w.br1().set_bit());
        let a = *resources.Pwm_off;
        schedule.on(scheduled + a.cycles()).unwrap();
    }

    #[task(priority = 1, resources = [ITM])]
    fn change_freq(val: u32) {
        let duty_cycle_percent: f32 = ((val as f32) * (80.0-20.0) / (4050.0) + 20.0);
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "val: {}", duty_cycle_percent);
    }

    #[interrupt(priority = 2, resources = [ADC], spawn = [change_freq])]
    fn ADC() {
        let value: u32 = resources.ADC.dr.read().bits();
        spawn.change_freq(value);
        resources.ADC.sr.modify(|_, w| w.eoc().clear_bit());
        resources.ADC.cr2.modify(|_, w| w.swstart().set_bit());         
    }

    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }

};
