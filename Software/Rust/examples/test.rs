#![no_main]
#![no_std]

extern crate cortex_m;
extern crate panic_halt;
use cortex_m::{iprintln, peripheral::DWT, Peripherals};
use cortex_m_rt::entry;
extern crate app;
use app::ea_dogs;
#[entry]
fn main() -> ! {
    
    loop {

    }
}