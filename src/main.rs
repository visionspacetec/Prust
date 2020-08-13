#![no_std] // #![no_std] if not testing
#![no_main] // #![no_main] if not testing
#![feature(alloc_error_handler)] // for defining alloc_error_handler

extern crate alloc; // linking alloc
// comment when debugging
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
/* Uncomment these and comment above for enabling printing to gdb terminal */
// use panic_semihosting as _;
// use cortex_m_semihosting::{ hprintln};

use cortex_m_rt::entry; // for declaring main an entry point

mod packet_parser; // includes packet_parser.rs
use packet_parser::*; // gets the helper functions

use alloc_cortex_m::CortexMHeap; // for declaring global allocator
use alloc::alloc::Layout; // for alloc error handler 

#[global_allocator] // set the global allocator
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry] // set entry point
fn main() -> ! {
    // initializing allocator before using it
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1024; // in bytes
    /* Starting the heap */
    unsafe { ALLOCATOR.init(start, size)};
    
    // calling the handler
    handle_packets();
}

#[alloc_error_handler]
/// Out Of Memory Handler
fn oom(_: Layout) -> ! {
    cortex_m::asm::bkpt(); // set breakpoint
    loop {}
}
