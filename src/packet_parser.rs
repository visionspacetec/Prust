use stm32l4xx_hal as hal; // HAL alias
use hal::{prelude::*,stm32,serial};
// SPP packets
use spp::packets;

// Data structure utilities
use arrayvec::ArrayString;
use heapless::consts;
use core::fmt::Write;

use nb; // for non blocking operations


// Helper functions to check the bits if its ok to read from usart
pub fn is_not_ok_to_read_usart2() -> bool {
    let isr = unsafe { &(*hal::stm32::USART2::ptr()).isr.read() };
    isr.rxne().bit_is_clear() && isr.ore().bit_is_clear()
}

pub fn is_not_ok_to_write_usart2() -> bool {
    let isr = unsafe { &(*hal::stm32::USART2::ptr()).isr.read() };
    isr.txe().bit_is_clear()
}

// Some debugging functions. Also to illustrate how registers are manipulated
pub fn _set_oversampling8() {
    unsafe { &(*hal::stm32::USART2::ptr()).cr1.modify(|_,w| w.over8().set_bit()) };
}

pub fn _is_oversampling8() -> bool {
    *unsafe { &(*hal::stm32::USART2::ptr()).cr1.read().over8().is_oversampling8()}
}

pub fn _get_baudrate() -> u32 {
    *unsafe { &(*hal::stm32::USART2::ptr()).brr.read().bits()}
}

// Function reads the packet and parses it and sends parsed packet.
pub fn handle_packets() -> ! {
    let dp = stm32::Peripherals::take().unwrap(); // get the device peripheral

    let rcc = dp.RCC.constrain(); // get the Rcc's abstract struct
    let mut ahb2 = rcc.ahb2;
    let mut apb1r1 =rcc.apb1r1;
    let flash = dp.FLASH.constrain();
    let mut acr = flash.acr;

    let mut gpioa = dp.GPIOA.split(&mut ahb2);
    
    // Could set to 115_200.bps for debugging
    let cfg = serial::Config::default().baudrate(2_000_000.bps());

    let clocks = rcc.cfgr.sysclk(72.mhz());

    let clocks = clocks.freeze(&mut acr); 
    
    let mut usart2 = hal::serial::Serial::usart2(dp.USART2,
        (gpioa.pa2.into_af7(&mut gpioa.moder,&mut gpioa.afrl),
        gpioa.pa3.into_af7(&mut gpioa.moder,&mut gpioa.afrl)),
        cfg,clocks,
        &mut apb1r1);
    
    
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