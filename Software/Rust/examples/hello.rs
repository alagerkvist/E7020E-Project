//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
//#![feature(alloc_error_handler)]
//#![feature(alloc)]
extern crate panic_halt;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

extern crate alloc;
//#[global_allocator]
//#[alloc_error_handler]
//#![feature(alloc_error_handler)]
extern crate sample;

//#[global_allocator]
use sample::Sample;
use cortex_m::asm;

extern crate cortex_m;

//#use core::alloc::GlobalAlloc;
//#use core::ptr;
use core::alloc::{Layout, GlobalAlloc};//System;
use core::ptr::null_mut;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAllocator = MyAllocator;

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

#[alloc_error_handler]
fn on_oom(_layout: core::alloc::Layout) -> ! {
    asm::bkpt();

    loop {}
}

