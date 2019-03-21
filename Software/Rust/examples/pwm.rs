//! examples/periodic.rs

#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;
use stm32f4::stm32f413::GPIOB;
use cortex_m_semihosting::hprintln;
use cortex_m::{asm, iprintln};
use rtfm::{app, Instant};

// calulate freq
// 

// freq = 16_000_000/PERIOD 
//
// 
// 
const PERIOD: u32 = 4_000_000;


// NOTE: does NOT work on QEMU!
#[app(device = stm32f4::stm32f413)]
const APP: () = {

    static mut GPIOB: GPIOB = ();



    static mut Duty_cycle: f32 = 0.5; // Duty range = [20 80]%
    static mut Pwm_on: u32 = 0;
    static mut Pwm_off: u32 = 0;

    #[init(schedule = [on], resources=[Pwm_on, Pwm_off, Duty_cycle])]
    fn init() {
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
    }

    #[task(schedule = [off], resources=[Pwm_on, GPIOB])]
    fn on() {
        resources.GPIOB.bsrr.write(|w| w.bs1().set_bit());
        // hprintln!("on").unwrap();
        let a = *resources.Pwm_on;
        schedule.off(scheduled + a.cycles()).unwrap();
    }

    #[task(schedule = [on], resources=[Pwm_off, GPIOB])]
    fn off() {
        resources.GPIOB.bsrr.write(|w| w.br1().set_bit());
        // hprintln!("off").unwrap();
        let a = *resources.Pwm_off;
        schedule.on(scheduled + a.cycles()).unwrap();
    }

    #[task(schedule = [foo])]
    fn foo() {
        let now = Instant::now();
        hprintln!("foo(scheduled = {:?}, now = {:?})", scheduled, now).unwrap();

        schedule.foo(scheduled + PERIOD.cycles()).unwrap();
    }

    // #[idle]
    // fn idle() -> ! {
    //     loop {
    //         asm::wfi();
    //     }
    // }


    extern "C" {
        fn UART5();
    }
};

