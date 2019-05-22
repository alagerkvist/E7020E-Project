//Uses a local compiled sample (https://github.com/RustAudio/sample) with libm (https://github.com/rust-lang-nursery/libm)
//Not tested on a MCU yet, but does compile
#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate panic_halt;
extern crate sample;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use core::alloc::{Layout, GlobalAlloc};//System;
use core::ptr::null_mut;
use cortex_m::asm;


struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAllocator = MyAllocator;
use sample::{signal, Signal};

#[entry]
fn main() -> ! {
    let mut signal = signal::rate(4.0).const_hz(1.0).sine();

    //hprintln!("{:?}", unsafe { sinf64(0.0) });//signal.next());
    hprintln!("{:?}", signal.next());
    //hprintln!("{:?}", signal.next());
    //hprintln!("{:?}", signal.next());
    //hprintln!("{:?}", signal.next());
    //assert_eq!(signal.next(), [0.0]);
    //assert_eq!(signal.next(), [1.0]);
    //signal.next();
    //assert_eq!(signal.next(), [-1.0]);
    loop {}
}

#[alloc_error_handler]
fn on_oom(_layout: core::alloc::Layout) -> ! {
    asm::bkpt();

    loop {}
}

