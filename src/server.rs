use super::*;
use stm32l4xx_hal as hal; // HAL alias
use hal::{gpio::{*,gpioa::*,gpiod::*,gpiof::*,gpioc::*,gpiog::*},prelude::*,stm32,serial};

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
    /* For testing*/
    experiments();

    /* FUNCTION MAP AREA START */ 
    let funcs:HashMap<FuncId,fn(&Vec::<u8>)->Result<(),Error>> = pus::map!(
        create_func_id("turn_led") => turn_led as fn(&Vec::<u8>)->Result<(),Error>,
        create_func_id("set_led") => set_led as fn(&Vec::<u8>)->Result<(),Error>,
        create_func_id("new_led") => new_led as fn(&Vec::<u8>)->Result<(),Error>
    );
    /* FUNCTION MAP AREA END */

    let mut hk_reports:HashMap<u8,Tc3_1> = HashMap::new();

    let mut usart2 = init(); // SharedPheriperal
    
    /* Allocate a 1KB Heapless buffer*/
    let mut buffer: heapless::Vec<u8, consts::U1024> = heapless::Vec::new();
    loop {
        buffer.clear();
        
        // Getting primary header
        for _i in 0..6 {
            while is_not_ok_to_read_usart2(){/*inf loop*/};
            let byte = nb::block!(usart2.read()).unwrap(); // if err wouldblock comes try again
            
            if buffer.push(byte).is_err() {
                // buffer full
                panic!("buffer_full");
            }
        };
        // If invalid packet ignore
        let ph = match  sp::PrimaryHeader::from_bytes(&buffer[0..6]){
            Ok(p) => {p}
            Err(_) => {continue;}
        };

        let data_len = ph.get_data_len() + 1;
        
        // getting the remaining of the pack
        for _i in 0..data_len {

            while is_not_ok_to_read_usart2(){/*inf loop*/};
            let byte = nb::block!(usart2.read()).unwrap(); // if err wouldblock comes try again

            if buffer.push(byte).is_err() {
                // buffer full
                panic!("buffer_full");
            }
        }
        let data_len = data_len + 6;

        let mut report_bytes:Vec<u8> = Vec::new() ;
        let mes_type =  mes_type_from_bytes(&buffer[0..data_len]);
        if mes_type == (8,1){
            /* TC[8,1] PERFORM A FUNCTION START */

            // checking if the packet given is in correct format or not
            let space_packet = match Tc8_1::from_bytes(&buffer[0..data_len]){
                Ok(sp) => {sp}
                Err(_) => {continue;}
            };
            // in case of error
            if let Err(e) = space_packet.exec_func(&funcs){
                let (err_code,err_data) = error::get_err_code_n_data(e);
                let err_report = SpacePacket::<_>::new_service_1_8(
                    &space_packet,0,0,err_code,err_data
                ).unwrap();
                report_bytes.extend(err_report.to_bytes().iter());
            } else {
                let exec_report = SpacePacket::<_>::new_service_1_7(
                    &space_packet,
                    42,
                    0,
                ).unwrap();
                report_bytes.extend(exec_report.to_bytes().iter());
            }

            /* TC[8,1] PERFORM A FUNCTION END */
        } else if mes_type == (3,1) {
            /* TC[3,1] CREATE A HOUSEKEEPING PARAMETER REPORT STRUCTURE START */
            
            let space_packet = match Tc3_1::from_bytes(&buffer[0..data_len]){
                Ok(sp) => {sp}
                Err(_) => {continue;}
            };
            let exec_report = SpacePacket::<_>::new_service_1_7(
                &space_packet,
                42,
                0,
            ).unwrap();
            // TODO: Give error in case of duplicate
            hk_reports.insert(space_packet.hk_id(), space_packet);

            report_bytes.extend(exec_report.to_bytes().iter());

            /* TC[3,1] CREATE A HOUSEKEEPING PARAMETER REPORT STRUCTURE END*/ 
        } else if mes_type == (3,27) {
            /* TC[3,27] GENERATE A ONE SHOT REPORT FOR HOUSEKEEPING PARAMETER REPORT STRUCTURES START*/

            let space_packet = match Tc3_27::from_bytes(&buffer[0..data_len]){
                Ok(sp) => {sp}
                Err(_) => {continue;}
            };
            
            generate_one_shot_report(&space_packet,&hk_reports,&mut report_bytes);

            let exec_report = SpacePacket::<_>::new_service_1_7(
                &space_packet,
                42,
                0,
            ).unwrap();
            report_bytes.extend(exec_report.to_bytes().iter());

            /* TC[3,27] GENERATE A ONE SHOT REPORT FOR HOUSEKEEPING PARAMETER REPORT STRUCTURES END*/
        }
        // write the report
        for &i in report_bytes.iter(){
            while is_not_ok_to_write_usart2() {};
            nb::block!(usart2.write(i)).ok();
        }
        
    }   
}