//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[derive(Debug)]
struct A{
    a: i32,
    b: u32,
}

#[entry]
fn main() -> ! {
    let a = A{a: -2, b: 5};
    hprintln!("Hello, world! How is it{:?}", a).unwrap();
    loop {}
}
