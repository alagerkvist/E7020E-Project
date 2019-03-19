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
use hal::stm32::GPIOB;
use hal::stm32::SPI3;
use hal::spi::{Spi, Mode, Phase, Polarity, NoMiso};
use hal::gpio::{Alternate, AF6, PushPull, Output};
use hal::gpio::gpioc::{PC10, PC11, PC12};
use hal::gpio::gpioa::{PA15};
use rtfm::{app, Instant};
mod lcd;
use lcd::*;
const PERIOD: u32 = 400_000;

#[app(device = hal::stm32)]
const APP: () = {
    static mut GPIOB: GPIOB = ();
    static mut Duty_cycle: f32 = 0.5; // Duty range = [20 80]%
    static mut Pwm_on: u32 = 0;
    static mut Pwm_off: u32 = 0;
    static mut ITM: ITM = ();    
    static mut ADC: ADC1 = ();
    static mut LCD_SCREEN: EA_dogs102_6w<Spi<SPI3, (PC10<Alternate<AF6>>, NoMiso, PC12<Alternate<AF6>>)>, PA15<Output<PushPull>>, PC11<Output<PushPull>>> = ();
    //static mut adc_data: dr<hal::stm32::adc1> = ();
    // init runs in an interrupt free section
    #[init(schedule = [on], resources=[Pwm_on, Pwm_off, Duty_cycle], spawn=[update_pwm])]
    fn init() {
        //let mut c = hal::stm32::CorePeripherals::take().unwrap();
        let stim = &mut core.ITM.stim[0];
       
        device.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit()); //Enable GPIOA clock
        device.RCC.apb2enr.modify(|_, w| w.adc1en().set_bit());
        device.RCC.ahb1enr.modify(|_, w| w.gpioben().set_bit());

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
        //device.ADC1.sqr3.modify(|_, w| unsafe { w.sq2().bits(2 as u8) });
        
        device.ADC1.sqr1.modify(|_,w| w.l().bits(0));
        let gpioa = device.GPIOA.split();
        gpioa.pa0.into_analog();
        //gpioa.pa2.into_analog();
        
        let rcc = device.RCC.constrain();

        // 16 MHz (default, all clocks)
        let clocks = rcc.cfgr.freeze();
        let gpioc = device.GPIOC.split();
        let sck = gpioc.pc10.into_alternate_af6();
        let mosi = gpioc.pc12.into_alternate_af6();
        
        let mut cd = gpioc.pc11.into_push_pull_output();
        let mut cs = gpioa.pa15.into_push_pull_output();

        pub const MODE: Mode = Mode {
            polarity: Polarity::IdleHigh,
            phase: Phase::CaptureOnFirstTransition,
        };
        let mut spi = Spi::spi3(
            device.SPI3,
            (sck, NoMiso, mosi),
            MODE,
            10_000_000.hz(),
            clocks
        );

        let mut lcd1 = EA_dogs102_6w::init(spi, cs, cd).unwrap();
        lcd1.write_word("Duty cycle: ");
        
        //lcd1.init();
        
        iprintln!(stim, "main");
        //device.ADC1.cr2.modify(|_, w| w);
        device.ADC1.cr2.modify(|_, w| w.swstart().set_bit());
        //*resources.Pwm_on = (PERIOD as f32 * *resources.Duty_cycle) as u32;
        //*resources.Pwm_off = PERIOD - *resources.Pwm_on;
        spawn.update_pwm(*resources.Duty_cycle);
        //schedule.foo(Instant::now() + PERIOD.cycles()).unwrap();
        // configure PA5 as output, RM0368 8.4.1
        device.GPIOB.moder.modify(|_, w| w.moder1().bits(1));

        // pass on late resources

        schedule.on(Instant::now());
        GPIOB = device.GPIOB;
        ADC = device.ADC1;
        ITM = core.ITM;
        LCD_SCREEN = lcd1;

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

    #[task(priority = 3, resources = [Pwm_on, Pwm_off, Duty_cycle])]
    fn update_pwm(dc: f32){
        let mut pwm: u32 = 0;
        resources.Pwm_on.lock(|pwm_on|{
            *pwm_on = (PERIOD as f32 * dc) as u32;
            pwm = *pwm_on;
        });
        resources.Pwm_off.lock(|pwm_off|{
            *pwm_off = PERIOD - pwm;
        });
        *resources.Duty_cycle = dc;
    }

    #[task(priority = 3, resources = [ITM, LCD_SCREEN], spawn = [update_pwm])]
    fn change_freq(val: u32) {
        let duty_cycle_percent: f32 = ((val as f32) * (80.0-20.0) / (4050.0) + 20.0);
        spawn.update_pwm(duty_cycle_percent / 100.0);
        let stim = &mut resources.ITM.stim[0];
        // resources.LCD_SCREEN.lock(|lcd_screen|{
        //     lcd_screen.write_word("sd");
        // });
        resources.LCD_SCREEN.update_duty(duty_cycle_percent as u32);
        iprintln!(stim, "val: {}", duty_cycle_percent);
    }

    #[task(priority = 1, resources = [ADC])]
    fn restart_adc(){
        resources.ADC.lock(|adc|{
            adc.sr.modify(|_, w| w.eoc().clear_bit());
            adc.sr.modify(|_, w| w.ovr().clear_bit());
            adc.cr2.modify(|_, w| w.swstart().set_bit()); 
        });
    }

    #[interrupt(priority = 3, resources = [ADC], spawn = [change_freq,restart_adc], schedule=[restart_adc])]
    fn ADC() {
        let value: u32 = resources.ADC.dr.read().bits();
        spawn.change_freq(value);
        //resources.ADC.cr2.modify(|_, w| w.swstart().set_bit());         
        spawn.restart_adc();
        //schedule.restart_adc(Instant::now() + 4_000_000.cycles()).unwrap();

    }

    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }

};
