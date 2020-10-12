use super::*;
use hal::{
    gpio::{gpioc::*, gpiof::*, gpiog::*, *},
    prelude::*,
    serial, stm32,
};
use stm32l4xx_hal as hal; // HAL alias

// Data structure utilities
use alloc::{string::String, vec::Vec};
use hashbrown::HashMap;
use heapless::consts; // for storing function names
extern crate alloc; // link the allocator
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex}; // for sharing PINS and resources

// use cortex_m_semihosting::hprintln;
use hal::interrupt;
use hal::time::Hertz;
use hal::timer::{Event, Timer};
use nb; // for non blocking operations

// Type aliases
type UART5TXType = serial::Tx<stm32::UART5>;
type UART5RXType = serial::Rx<stm32::UART5>;
type Timer2Type = Timer<hal::device::TIM2>;

static SHARED_PER: Mutex<RefCell<Option<SharedPeripherals>>> = Mutex::new(RefCell::new(None));
static UART5TX: Mutex<RefCell<Option<UART5TXType>>> = Mutex::new(RefCell::new(None));
static TIMER2: Mutex<RefCell<Option<Timer2Type>>> = Mutex::new(RefCell::new(None));
// REPORT_ID -> (PACKET,PERIODIC_REPORT_ENABLED)
lazy_static! {
    pub static ref HK_REPORTS: Mutex<RefCell<HashMap<u8, (Tc3_1, bool)>>> =
        Mutex::new(RefCell::new(HashMap::new()));
}
// NOTE: Right now we have one min sample collection interval
const MIN_SAMPL_DIV: u32 = 20;
const SYS_FREQ: Hertz = Hertz { 0: 72_000_000 };

pub mod func_man;
/// Utility module for the temporary problem
pub mod utils;
use func_man::{functions::*, *};
use utils::*;

// Function reads the packet and parses it and sends parsed packet.
pub fn handle_packets() -> ! {
    /* FUNCTION MAP AREA START */
    let funcs: HashMap<FuncId, fn(&Vec<u8>) -> Result<(), Error>> = pus::map!(
        create_func_id("turn_led") => pre_turn_led as fn(&Vec::<u8>)->Result<(),Error>,
        create_func_id("set_led") => pre_set_led as fn(&Vec::<u8>)->Result<(),Error>
    );
    /* FUNCTION MAP AREA END */

    // Initializing peripheral and internal resources
    let mut rx = init();

    /* Allocate a 1KB Heapless buffer*/
    let mut buffer: heapless::Vec<u8, consts::U1024> = heapless::Vec::new();
    let mut data_len;
    'main: loop {
        buffer.clear();

        for _i in 0..6 {
            while is_not_ok_to_read_uart5() { /*inf loop*/ }
            let byte = match nb::block!(rx.read()) {
                Ok(b) => b,
                Err(_) => {
                    continue 'main; // TODO: Give warning
                }
            }; // if err wouldblock comes try again

            if buffer.push(byte).is_err() {
                continue 'main; // TODO: Give warning
            }
        }
        // If invalid packet ignore
        let ph = match sp::PrimaryHeader::from_bytes(&buffer[0..6]) {
            Ok(p) => p,
            Err(_) => {
                continue; // TODO: Give warning
            }
        };

        data_len = ph.get_data_len() + 1;

        // getting the remaining of the pack
        for _i in 0..data_len {
            let byte = match nb::block!(rx.read()) {
                Ok(b) => b,
                Err(_) => {
                    continue 'main; // TODO: Give warning
                }
            }; // if err wouldblock comes try again

            if buffer.push(byte).is_err() {
                continue 'main; // TODO: Give warning
            }
        }

        let data_len = data_len + 6;

        let mut report_bytes: Vec<u8> = Vec::new();
        let mes_type = match pus::sp::get_service_type(&buffer[0..data_len]) {
            Ok(m) => m,
            _ => continue, // TODO: Give warning
        };

        if mes_type == (8, 1) {
            /* TC[8,1] PERFORM A FUNCTION START */

            // checking if the packet given is in correct format or not
            let space_packet = match Tc8_1::from_bytes(&buffer[0..data_len]) {
                Ok(sp) => sp,
                Err(_) => {
                    continue;
                }
            };
            // in case of error after executing the func
            if let Err(e) = space_packet.exec_func(&funcs) {
                let (err_code, err_data) = error::get_err_code_n_data(e);
                let err_report =
                    match SpacePacket::new_service_1_8(&space_packet, 0, 0, err_code, err_data) {
                        Ok(s) => s,
                        Err(_) => continue, // TODO: Give a warning
                    };
                report_bytes.extend(err_report.to_bytes().iter());
            } else {
                let exec_report = match SpacePacket::new_service_1_7(&space_packet, 42, 0) {
                    Ok(s) => s,
                    Err(_) => continue, // TODO: Give warning
                };
                report_bytes.extend(exec_report.to_bytes().iter());
            }

        /* TC[8,1] PERFORM A FUNCTION END */
        } else if mes_type == (3, 1) {
            /* TC[3,1] CREATE A HOUSEKEEPING PARAMETER REPORT STRUCTURE START */

            let space_packet = match Tc3_1::from_bytes(&buffer[0..data_len]) {
                Ok(sp) => sp,
                Err(_) => continue, // TODO: Give warning
            };
            let exec_report = SpacePacket::<_>::new_service_1_7(&space_packet, 42, 0).unwrap();
            // TODO: Give error in case of duplicate
            free(|cs| {
                if let Ok(mut hk) = HK_REPORTS.borrow(cs).try_borrow_mut() {
                    hk.insert(space_packet.hk_id(), (space_packet, false));
                }
            });

            report_bytes.extend(exec_report.to_bytes().iter());

        /* TC[3,1] CREATE A HOUSEKEEPING PARAMETER REPORT STRUCTURE END*/
        } else if mes_type == (3, 27) {
            /* TC[3,27] GENERATE A ONE SHOT REPORT FOR HOUSEKEEPING PARAMETER REPORT STRUCTURES START*/

            let space_packet = match Tc3_27::from_bytes(&buffer[0..data_len]) {
                Ok(sp) => sp,
                Err(_) => continue, // TODO: Give warning
            };

            generate_one_shot_report(&space_packet, &mut report_bytes);

            let exec_report = SpacePacket::new_service_1_7(&space_packet, 42, 0).unwrap();
            report_bytes.extend(exec_report.to_bytes().iter());

        /* TC[3,27] GENERATE A ONE SHOT REPORT FOR HOUSEKEEPING PARAMETER REPORT STRUCTURES END*/
        } else if mes_type == (3, 5) || mes_type == (3, 6) {
            /* TC[3,5/6] ENABLE OR DISABLE PERIODIC GENERATION OF THE HOUSEKEEPING PARAMETER REPORT*/
            let space_packet = match SpacePacket::from_bytes_service_3_5x6(&buffer[0..data_len]) {
                Ok(sp) => sp,
                Err(_) => continue, // TODO: Give warning
            };
            let hk_params = space_packet.get_report_parameter_ids();
            let mut toggled_timer = false;
            free(|cs| {
                let mut hk_reports = HK_REPORTS.borrow(cs).try_borrow_mut().unwrap();
                for i in hk_params.iter() {
                    if let Some(ent) = hk_reports.get_mut(i) {
                        if ent.1 != (mes_type.1 == 5) {
                            // toggle if you have to else do nothing
                            ent.1 = mes_type.1 == 5;
                            toggled_timer = true;
                        }
                    }
                }
            });
            // if enabled listen to timer
            if toggled_timer {
                // Listening or Unlistening to timer in critical section
                cortex_m::interrupt::free(|cs| {
                    if let Some(timer2) = TIMER2.borrow(cs).try_borrow_mut().unwrap().as_mut() {
                        if mes_type.1 == 5 {
                            // enable timer
                            timer2.listen(Event::TimeOut);
                        } else {
                            // disable timer
                            timer2.unlisten(Event::TimeOut);
                        }
                    }
                });
            }
        } else {
            continue;
        }

        // Writing to uart in Critical Section.
        cortex_m::interrupt::free(|cs| {
            if let Some(uart5) = UART5TX.borrow(cs).try_borrow_mut().unwrap().as_mut() {
                // write the report
                for &i in report_bytes.iter() {
                    while is_not_ok_to_write_uart5() {}
                    if let Err(_) = nb::block!(uart5.write(i)) {
                        continue;
                    }
                }
            }
        });
    }
}
#[interrupt]
fn TIM2() {
    static mut PERIODIC_BUF: Vec<u8> = Vec::new();

    PERIODIC_BUF.clear();
    generate_periodic_report(PERIODIC_BUF);
    // get in critical section
    free(|cs| {
        if let Some(tim2) = TIMER2.borrow(cs).try_borrow_mut().unwrap().as_mut() {
            tim2.clear_update_interrupt_flag();
            if let Some(uart5) = UART5TX.borrow(cs).try_borrow_mut().unwrap().as_mut() {
                // write the report
                for &i in PERIODIC_BUF.iter() {
                    while is_not_ok_to_write_uart5() {}
                    if let Err(_) = nb::block!(uart5.write(i)) {
                        return; // If error writing exit the interrupt handler after clearing the interrupt
                    };
                }
            }
        }
    });
}
