#![no_std] // #![no_std] if not testing
#![no_main] // #![no_main] if not testing
#![feature(alloc_error_handler)] // for defining alloc_error_handler

extern crate alloc; // linking alloc
use nb; // for non blocking operations

// uncomment when debugging
//use panic_semihosting as _;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// useful when debugging
// use cortex_m_semihosting::{ hprintln};


use cortex_m_rt::entry; // for declaring main an entry point
use stm32l4xx_hal as hal; // HAL alias
use crate::hal::{prelude::*,stm32,serial};

// SPP packets
use spp::packets;
mod packet_parser; // includes packet_parser.rs
use packet_parser::*;

use arrayvec::ArrayString;
use heapless::consts;
use core::fmt::Write;

use alloc_cortex_m::CortexMHeap; // for declaring global allocator
use alloc::alloc::Layout; // for alloc error handler 

#[global_allocator] // set the global allocator
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();


//#[cfg(not(test))] // this is main if test is not set
#[entry] // set entry point
fn main() -> ! {
    // initializing allocator befor using it
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size)}

    let dp = stm32::Peripherals::take().unwrap(); // get the device peripheral

    let rcc = dp.RCC.constrain(); // get the Rcc's abstract struct
    let mut ahb2 = rcc.ahb2;
    let mut apb1r1 =rcc.apb1r1;
    let flash = dp.FLASH.constrain();
    let mut acr = flash.acr;

    let mut gpioa = dp.GPIOA.split(&mut ahb2);
    
    //let cfg = serial::Config::default().baudrate(115_200.bps());
    let cfg = serial::Config::default().baudrate(2_000_000.bps());

    let clocks = rcc.cfgr.sysclk(72.mhz());
    //clocks.pclk1(72.mhz());

    let clocks = clocks.freeze(&mut acr); 
    
    let mut usart2 = hal::serial::Serial::usart2(dp.USART2,
        (gpioa.pa2.into_af7(&mut gpioa.moder,&mut gpioa.afrl),
        gpioa.pa3.into_af7(&mut gpioa.moder,&mut gpioa.afrl)),
        cfg,clocks,
        &mut apb1r1);
    
    //hprintln!("pclk1{:?},os8:{},{:#b}",clocks.pclk1(),is_oversampling8(),get_baudrate()).unwrap();
    
    /* Allocate a 1KB Heapless buffer*/
    let mut buffer: heapless::Vec<u8, consts::U1024> = heapless::Vec::new();
    loop {
        buffer.clear();
        for _i in 0..6 {

            while is_not_ok_to_read_usart2(){/*inf loop*/};
            let byte = nb::block!(usart2.read()).unwrap(); // if err wouldblock comes try again

            if buffer.push(byte).is_err() {
                // buffer full
                for byte in b"error: buffer full\n\r" {
                    while is_not_ok_to_write_usart2() {};
                    nb::block!(usart2.write(*byte)).ok();
                }
                break;
            }
        };
        let ph = packets::PrimaryHeader::from_bytes(&buffer[0..6]).unwrap();
        let data_len = ph.get_data_len() + 1;

        for _i in 0..data_len {

            while is_not_ok_to_read_usart2(){/*inf loop*/};
            let byte = nb::block!(usart2.read()).unwrap(); // if err wouldblock comes try again

            if buffer.push(byte).is_err() {
                // buffer full
                for byte in b"error: buffer full\n\r" {
                    while is_not_ok_to_write_usart2() {};
                    nb::block!(usart2.write(*byte)).ok();
                }
                break;
            }

        }
        let data_len = data_len + 6;
        let space_packet = packets::SpacePacket::from_bytes(&buffer[0..data_len]).unwrap();
        // print packet
        let mut print_buf = ArrayString::<[u8; 512]>::new();
        writeln!(print_buf,"{}",space_packet).unwrap();

        for byte in print_buf.chars(){
            while is_not_ok_to_write_usart2() {};
            nb::block!(usart2.write(byte as u8)).ok();
        }
    }   
}


//#[cfg(not(test))] // only compile when the test flag is not set
#[alloc_error_handler]
/// Out Of Memory Handler
fn oom(_: Layout) -> ! {
    cortex_m::asm::bkpt(); // set breakpoint
    loop {}
}
