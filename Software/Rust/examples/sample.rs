//Uses a local compiled sample (https://github.com/RustAudio/sample) with libm (https://github.com/rust-lang-nursery/libm)
//Not tested on a MCU yet, but does compile
#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate panic_halt;
extern crate sample;
extern crate stm32f4xx_hal as hal;
use cortex_m::{asm, iprintln};
use rtfm::{app};

use core::alloc::{Layout, GlobalAlloc};
use core::ptr::null_mut;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAllocator = MyAllocator;
use sample::{signal, Signal, I24, Sample};


#[app(device = hal::stm32)]
const APP: () = {

    #[init]
    fn init() {
        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "hello codec");
        let mut signal = signal::rate(4.0).const_hz(1.0).sine();
        for _i in 0..4 {
            let s = signal.next();
            let t = I24::from_sample(s[0]);
            iprintln!(stim, "{:?}", t);
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

