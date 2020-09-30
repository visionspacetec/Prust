#![no_std]
#![no_main]
#![feature(alloc_error_handler)] // for defining alloc_error_handler

// pus sp
use pus::{*,error::*,sp::*,sp::{tm::*,tc::*,services::{service_8::*,service_3::service_3_1::*,service_3::service_3_27::*,service_3::service_3_25::*}}};

// Give aliases
type Tc3_1 =  SpacePacket::<TcPacket<Service3_1>>;
type Tc3_27 =  SpacePacket::<TcPacket<Service3_27>>;
type Tm3_25 =  SpacePacket::<TmPacket<Service3_25>>;
type Tc8_1 =  SpacePacket::<TcPacket<Service8_1>>;


extern crate alloc; // linking alloc
// comment when debugging
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
/* Uncomment these and comment above for enabling printing to gdb terminal */
use panic_semihosting as _;
// use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry; // for declaring main an entry point

mod server; // includes server.rs
use server::handle_packets; // gets the helper functions

use alloc_cortex_m::CortexMHeap; // for declaring global allocator
use alloc::alloc::Layout; // for alloc error handler 

#[global_allocator] // set the global allocator
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[macro_use]
extern crate lazy_static;

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
