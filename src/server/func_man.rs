
use super::*;
use stm32::USART2;
use serial::Serial;


const BLANK_VEC:[u8;FUNC_ID_LEN] = [0 as u8;FUNC_ID_LEN];
// A temp helper function
pub fn create_func_id(name:&str) -> FuncId{
    let mut res = String::from(name);
    res.push_str(&String::from_utf8(BLANK_VEC[name.len()..].to_vec()).unwrap());
    let res = FuncId::from(&res).unwrap();
    res
}


/// FuncId = "turn_led"
pub fn turn_led(args:&Vec::<u8>) -> Result<(),Error>{
    cortex_m::interrupt::free(|cs| -> Result<(),Error> {
        if args[0] != 0 {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led1.set_high()?;
            Ok(())
        }
        else {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led1.set_low()?;
            Ok(())
        } 
    })
}

/// FuncId = "set_led"
pub fn set_led(args:&Vec::<u8>) -> Result<(),Error>{
    if args.len() != 2 {
        return Err(Error::InvalidArg);
    }
    cortex_m::interrupt::free(|cs| -> Result<(),Error>{
        if args[0] == 0{
            if args[1] != 0 { 
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led1.set_high()?;
                Ok(())
            }
            else {
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led1.set_low()?;
                Ok(())
            }  
        } else if args[0] == 1 {
            if args[1] != 0 { 
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led2.set_high()?;
                Ok(())
            }
            else {
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led2.set_low()?;
                Ok(())
            }    
        }else if args[0] == 2 {
            if args[1] != 0 { 
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led3.set_high()?;
                Ok(())
            }
            else {
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led3.set_low()?;
                Ok(())
            }     
        }
        else if args[0] == 3 {
            if args[1] != 0 { 
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led4.set_high()?;
                Ok(())
            }
            else {
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led4.set_low()?;
                Ok(())
            }     
        }
        else {
            Err(Error::InvalidArg)
        }
    })
}


/// FuncId = "new_led"
pub fn new_led(args:&Vec::<u8>) -> Result<(),Error>{
    cortex_m::interrupt::free(|cs| -> Result<(),Error> {
        if args[0] != 0 {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led5.set_high()?;
            Ok(())
        }
        else {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led5.set_low()?;
            Ok(())
        } 
    })
}
/// Change Here If An External Function Needs To Access Peripheral Data
pub struct SharedPeripherals{
    pub led1:PA5<Output<PushPull>>,
    pub led2:PA6<Output<PushPull>>,
    pub led3:PA7<Output<PushPull>>,
    pub led4:PA8<Output<PushPull>>,
    pub led5:PA9<Output<PushPull>>,
} 

pub fn init() -> Serial<USART2, (PA2<Alternate<AF7, Input<Floating>>>, PA3<Alternate<AF7, Input<Floating>>>)>{
    
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
    
    let usart2 = hal::serial::Serial::usart2(dp.USART2,
        (gpioa.pa2.into_af7(&mut gpioa.moder,&mut gpioa.afrl),
        gpioa.pa3.into_af7(&mut gpioa.moder,&mut gpioa.afrl)),
        cfg,clocks,
        &mut apb1r1);
    
    let led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led2 = gpioa.pa6.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led3 = gpioa.pa7.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led4 = gpioa.pa8.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    
    let led5 = gpioa.pa9.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);

    // Replacing the Shared Peripheral
    // Also change here to if you changed SharedPeripherals
    cortex_m::interrupt::free(|cs|{
        SHARED_PER.borrow(cs).replace(Some(
            SharedPeripherals{led1,led2,led3,led4,led5}
        ));
    });
    usart2
}