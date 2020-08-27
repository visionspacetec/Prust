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
//use cortex_m_semihosting::hprintln;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex; // for sharing LED

use hal::gpio::gpioa::*;
use hal::gpio::*;

static LED1: Mutex<RefCell<Option<PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static LED2: Mutex<RefCell<Option<PA6<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static LED3: Mutex<RefCell<Option<PA7<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static LED4: Mutex<RefCell<Option<PA8<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

/// Utility module for the temporary problem
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

    /// FuncId = "turn_led"
    pub fn turn_led(args:&Vec::<u8>){
        cortex_m::interrupt::free(|cs|{
            if args[0] != 0 { 
                LED1.borrow(cs).borrow_mut().as_mut().unwrap().set_high().unwrap();
            }
            else {
                LED1.borrow(cs).borrow_mut().as_mut().unwrap().set_low().unwrap();
            } 
        });
    }

    pub fn set_led(args:&Vec::<u8>){
        cortex_m::interrupt::free(|cs|{
            if args[0] == 0{
                if args[1] != 0 { 
                    LED1.borrow(cs).borrow_mut().as_mut().unwrap().set_high().unwrap();
                }
                else {
                    LED1.borrow(cs).borrow_mut().as_mut().unwrap().set_low().unwrap();
                }  
            } else if args[0] == 1 {
                if args[1] != 0 { 
                    LED2.borrow(cs).borrow_mut().as_mut().unwrap().set_high().unwrap();
                }
                else {
                    LED2.borrow(cs).borrow_mut().as_mut().unwrap().set_low().unwrap();
                }  
            }else if args[0] == 2 {
                if args[1] != 0 { 
                    LED3.borrow(cs).borrow_mut().as_mut().unwrap().set_high().unwrap();
                }
                else {
                    LED3.borrow(cs).borrow_mut().as_mut().unwrap().set_low().unwrap();
                }  
            }
            else if args[0] == 3 {
                if args[1] != 0 { 
                    LED4.borrow(cs).borrow_mut().as_mut().unwrap().set_high().unwrap();
                }
                else {
                    LED4.borrow(cs).borrow_mut().as_mut().unwrap().set_low().unwrap();
                }  
            }

            
        });
    }
}

use util::*;


// Function reads the packet and parses it and sends parsed packet.
pub fn handle_packets() -> ! {
    // init function map
    let funcs:HashMap<FuncId,fn(&Vec::<u8>)> = pus::map!(
        util::create_func_id("turn_led") => util::turn_led as fn(&Vec::<u8>),
        util::create_func_id("set_led") => util::set_led as fn(&Vec::<u8>)
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
    
    let led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led2 = gpioa.pa6.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led3 = gpioa.pa7.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led4 = gpioa.pa8.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    // set global shared variable led
    cortex_m::interrupt::free(|cs| LED1.borrow(cs).replace(Some(led1)));
    cortex_m::interrupt::free(|cs| LED2.borrow(cs).replace(Some(led2)));
    cortex_m::interrupt::free(|cs| LED3.borrow(cs).replace(Some(led3)));
    cortex_m::interrupt::free(|cs| LED4.borrow(cs).replace(Some(led4)));
    
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