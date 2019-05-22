#![no_main]
#![no_std]


extern crate cortex_m;
extern crate panic_halt;
use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;
use core::f64::consts::PI;
extern crate libm;
//use libm;
//use libm::sin;

#[entry]
fn main() -> ! {
    let x: f32 = 1.0;
    hprintln!("{:?}", x.F32Ext.sin());
    //hprintln!("{:?}", sin(3.0));
    hprintln!("{:?}", PI);
    loop {

    }
}