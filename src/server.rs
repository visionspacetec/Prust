use stm32l4xx_hal as hal; // HAL alias
use hal::{gpio::{*,gpioa::*},prelude::*,stm32,serial};

// pus sp
use pus::{*,error::*,sp::*,sp::{tc::*,services::{service_8::*}}};

// Data structure utilities
use heapless::consts;
use alloc::{vec::Vec,string::String};
use hashbrown::HashMap; // for storing function names
extern crate alloc; // link the allocator
use nb; // for non blocking operations
//use cortex_m_semihosting::hprintln;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex; // for sharing LED

static SHARED_PER: Mutex<RefCell<Option<SharedPeripherals>>> = Mutex::new(RefCell::new(None));

/// Utility module for the temporary problem
pub mod utils;
pub mod func_man;

use utils::*;
use func_man::*;

// Function reads the packet and parses it and sends parsed packet.
pub fn handle_packets() -> ! {
    /* FUNCTION MAP AREA START */ 
    let funcs:HashMap<FuncId,fn(&Vec::<u8>)->Result<(),Error>> = pus::map!(
        create_func_id("turn_led") => turn_led as fn(&Vec::<u8>)->Result<(),Error>,
        create_func_id("set_led") => set_led as fn(&Vec::<u8>)->Result<(),Error>
    );
    /* FUNCTION MAP AREA END */

    let mut usart2 = init(); // SharedPheriperal
    
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
        let ph = sp::PrimaryHeader::from_bytes(&buffer[0..6]).unwrap();
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
        let space_packet = SpacePacket::<TcPacket<Service8_1>>::from_bytes(&buffer[0..data_len]).unwrap();
        // in case of error
        if let Err(e) = space_packet.exec_func(&funcs){
            let (err_code,err_data) = services::service_1::service_fail::get_err_code_n_data(e);
            let err_report = SpacePacket::<_>::new_service_1_8(
                &space_packet,0,0,err_code,err_data
            ).unwrap();
         
            // write the report
            for &i in err_report.to_bytes().iter(){
                while is_not_ok_to_write_usart2() {};
                nb::block!(usart2.write(i)).ok();
            } 
        } else {
            let exec_report = SpacePacket::<_>::new_service_1_7(
                &space_packet,
                0,
                0,
            ).unwrap();
            // write the report
            for &i in exec_report.to_bytes().iter(){
                while is_not_ok_to_write_usart2() {};
                nb::block!(usart2.write(i)).ok();
            }
            
        }
    }   
}
