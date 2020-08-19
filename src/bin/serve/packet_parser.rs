use stm32l4xx_hal as hal; // HAL alias
use hal::{prelude::*,stm32,serial};
// pus sp
use pus::*;
// Data structure utilities
use heapless::consts;
use alloc::string::String;
use hashbrown::HashMap; // for storing function names
extern crate alloc; // link the allocator
use alloc::vec::Vec;
use nb; // for non blocking operations
use cortex_m_semihosting::hprintln;
use core::cell::RefCell;

static LED: Mutex<RefCell<Option<stm32l4xx_hal::gpio::gpioa::PA5<stm32l4xx_hal::gpio::Output<stm32l4xx_hal::gpio::PushPull>>>>> =
    Mutex::new(RefCell::new(None));

#[macro_use]
pub mod util{
    use super::*;
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

    const BLANK_VEC:[u8;FUNC_ID_LEN] = [0 as u8;FUNC_ID_LEN];
    // A temp helper function
    pub fn create_func_id(name:&str) -> FuncId{
        let mut res = String::from(name);
        res.push_str(&String::from_utf8(BLANK_VEC[name.len()..].to_vec()).unwrap());
        let res = FuncId::from(&res).unwrap();
        res
    }

    /// FuncId in ascii is "turn_led"
    pub fn turn_led(args:&Vec::<u8>){
        cortex_m::interrupt::free(|cs|{
            if args[0] != 0 { LED.borrow(cs).borrow_mut().as_mut().unwrap().set_high().unwrap()}
            else { LED.borrow(cs).borrow_mut().as_mut().unwrap().set_low().unwrap()} 
        });
    }
}

use util::*;
use cortex_m::interrupt::Mutex;
/* lazy_static!{
    pub static ref LED:Mutex<RefCell<stm32l4xx_hal::gpio::gpioa::PA5<stm32l4xx_hal::gpio::Output<stm32l4xx_hal::gpio::PushPull>>>> = {
        let dp;
        //unsafe {
            dp = stm32::Peripherals::steal().unwrap(); // get the device peripheral
        //}

        let rcc = dp.RCC.constrain();
        let mut ahb2 = rcc.ahb2;
        let mut gpioa = dp.GPIOA.split(&mut ahb2);
        let led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
        drop(dp);
        Mutex::new(RefCell::new(led))
    };
} */

// Function reads the packet and parses it and sends parsed packet.
pub fn handle_packets() -> ! {
    // init function map
    let funcs:HashMap<FuncId,fn(&Vec::<u8>)> = pus::map!(
        util::create_func_id("turn_led") => util::turn_led as fn(&Vec::<u8>)
    );
    
    let dp = stm32::Peripherals::take().unwrap(); // get the device peripheral

    let rcc = dp.RCC.constrain(); // get the Rcc's abstract struct
    let mut ahb2 = rcc.ahb2;
    let mut apb1r1 =rcc.apb1r1;
    let flash = dp.FLASH.constrain();
    let mut acr = flash.acr;

    let mut gpioa = dp.GPIOA.split(&mut ahb2);
    
    // Could set to 115_200.bps for debugging
    let cfg = serial::Config::default().baudrate(115_200.bps());

    let clocks = rcc.cfgr.sysclk(72.mhz());

    let clocks = clocks.freeze(&mut acr); 
    
    
    
    let mut usart2 = hal::serial::Serial::usart2(dp.USART2,
        (gpioa.pa2.into_af7(&mut gpioa.moder,&mut gpioa.afrl),
        gpioa.pa3.into_af7(&mut gpioa.moder,&mut gpioa.afrl)),
        cfg,clocks,
        &mut apb1r1);
    
    let led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    cortex_m::interrupt::free(|cs| LED.borrow(cs).replace(Some(led)));
    
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
        //hprintln!("{:?}",&buffer[0..data_len]);
        let space_packet = sp::SpacePacket::< pus::sp::tc::TcPacket< pus::sp::tc::service_8::Service8_1>>::from_bytes(&buffer[0..data_len]).unwrap();
        // print packet
        space_packet.exec_func(&funcs);
    }   
}