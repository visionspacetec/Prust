
use super::*;
use stm32::{ADC1,UART5};
use serial::Serial;
use hal::rcc::*;
use hal::adc::Adc;

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
    if args.len() != 1 {
        return Err(Error::InvalidArg);
    }
    cortex_m::interrupt::free(|cs| -> Result<(),Error> {
        if args[0] != 0 {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led1.set_high().unwrap();
            Ok(())
        }
        else {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led1.set_low().unwrap();
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
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led1.set_high().unwrap();
                Ok(())
            }
            else {
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led1.set_low().unwrap();
                Ok(())
            }  
        } else if args[0] == 1 {
            if args[1] != 0 { 
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led2.set_high().unwrap();
                Ok(())
            }
            else {
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led2.set_low().unwrap();
                Ok(())
            }    
        }else if args[0] == 2 {
            if args[1] != 0 { 
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led3.set_high().unwrap();
                Ok(())
            }
            else {
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led3.set_low().unwrap();
                Ok(())
            }     
        }
        else if args[0] == 3 {
            if args[1] != 0 { 
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led4.set_high().unwrap();
                Ok(())
            }
            else {
                SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led4.set_low().unwrap();
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
    if args.len() != 1 {
        return Err(Error::InvalidArg);
    }
    cortex_m::interrupt::free(|cs| -> Result<(),Error> {
        if args[0] != 0 {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led5.set_high().unwrap();
            Ok(())
        }
        else {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led5.set_low().unwrap();
            Ok(())
        } 
    })
}
/// Change Here If An External Function Needs To Access Peripheral Data
pub struct SharedPeripherals{
    pub led1:PFx<Output<PushPull>>,
    pub led2:PFx<Output<PushPull>>,
    pub led3:PA7<Output<PushPull>>,
    pub led4:PA8<Output<PushPull>>,
    pub led5:PA9<Output<PushPull>>,
    pub adc1:Adc<ADC1>,
    pub potent:PC5<Analog>
} 

pub fn init() -> Serial<UART5, (PC12<Alternate<AF8, Input<Floating>>>, PD2<Alternate<AF8, Input<Floating>>>)>{
    
    let dp = stm32::Peripherals::take().unwrap(); // get the device peripheral
    let mut rcc = dp.RCC.constrain(); // get the Rcc's abstract struct
    let mut ahb2 = rcc.ahb2;
    let mut apb1r1 =rcc.apb1r1;
    let flash = dp.FLASH.constrain();
    let mut acr = flash.acr;
    let mut pwr = dp.PWR.constrain(&mut apb1r1);

    let mut gpioa = dp.GPIOA.split(&mut ahb2);
    let mut gpiob = dp.GPIOB.split(&mut ahb2);
    let mut gpioc = dp.GPIOC.split(&mut ahb2);
    let mut gpiod = dp.GPIOD.split(&mut ahb2);
    let mut gpioe = dp.GPIOE.split(&mut ahb2);
    let mut gpiof = dp.GPIOF.split(&mut ahb2);
    let mut gpiog = dp.GPIOG.split(&mut ahb2);

    // Could set to 115_200.bps for debugging
    let cfg = serial::Config::default().baudrate(57_600.bps());
    let adc_cfg = hal::adc::Config::default();
    //let pll_cfg = PllConfig::new(1, 8,PllDivider::Di);
    let common_cfg = hal::adc::CommonConfig::default();
    let clocks = rcc.cfgr.sysclk(64.mhz());
    let clocks = clocks.freeze(&mut acr,&mut pwr); 
    
    
    let mut adc1 = hal::adc::Adc::adc1(dp.ADC1,adc_cfg,&mut ahb2,&mut rcc.ccipr);
    //let mut user1_en = gpioc.pc9.into_push_pull_output(&mut gpioc.moder,&mut gpioc.otyper);
    let mut user4_en = gpiog.pg12.into_push_pull_output(&mut gpiog.moder,&mut gpiog.otyper);
    let mut user3_en = gpiog.pg15.into_push_pull_output(&mut gpiog.moder,&mut gpiog.otyper);
    user4_en.set_high().unwrap();
    user3_en.set_low().unwrap();
    //let mut usart2_en = gpiog.pg2.into_push_pull_output(&mut gpiog.moder, &mut gpiog.otyper);
    //usart2_en.set_high().unwrap();
    //user1_en.set_high().unwrap();
    let mut usart2 = hal::serial::Serial::uart5(dp.UART5,
        (gpioc.pc12.into_af8(&mut gpioc.moder,&mut gpioc.afrh),
        gpiod.pd2.into_af8(&mut gpiod.moder,&mut gpiod.afrl)),
        cfg,clocks,
        &mut apb1r1);
    
    let led1 = gpiof.pf13.into_push_pull_output(&mut gpiof.moder,&mut gpiof.otyper).downgrade();
    //led1.set_high().unwrap();
    let led2 = gpiof.pf15.into_push_pull_output(&mut gpiof.moder,&mut gpiof.otyper).downgrade();
    let led3 = gpioa.pa7.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led4 = gpioa.pa8.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led5 = gpioa.pa9.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    hal::adc::adc_global_setup(common_cfg,&mut ahb2,&mut rcc.ccipr);
    let mut user4_4 = gpioc.pc5.into_analog_with_adc(&mut gpioc.moder,&mut gpioc.pupdr);
    let mut user3_2 = gpiob.pb1.into_analog_with_adc(&mut gpiob.moder,&mut gpiob.pupdr);
    
    loop {
        let res = adc1.read(&mut user3_2);
        
        if res.is_ok() {
            let res:u16 = res.unwrap();
            hprintln!("VAL:{}", res).unwrap();
        }
        else {
            hprintln!("FAIL").unwrap();
        }
        cortex_m::asm::delay(80_000_00);
    }
    // Replacing the Shared Peripheral
    // Also change here to if you changed SharedPeripherals
    /* cortex_m::interrupt::free(|cs|{
        SHARED_PER.borrow(cs).replace(Some(
            SharedPeripherals{led1,led2,led3,led4,led5,adc1,potent:user4_4}
        ));
    }); */
    usart2
}

pub fn experiments() -> !{
    
    let dp = stm32::Peripherals::take().unwrap(); // get the device peripheral
    let mut rcc = dp.RCC.constrain(); // get the Rcc's abstract struct
    let mut ahb2 = rcc.ahb2;
    let mut apb1r1 =rcc.apb1r1;
    let flash = dp.FLASH.constrain();
    let mut acr = flash.acr;
    let mut pwr = dp.PWR.constrain(&mut apb1r1);
    unsafe {
        ((*hal::device::PWR::ptr()).cr2).modify(|_,w| w.iosv().set_bit())
    }
    
    let mut gpioa = dp.GPIOA.split(&mut ahb2);
    let mut gpiob = dp.GPIOB.split(&mut ahb2);
    let mut gpioc = dp.GPIOC.split(&mut ahb2);
    let mut gpiod = dp.GPIOD.split(&mut ahb2);
    let mut gpioe = dp.GPIOE.split(&mut ahb2);
    let mut gpiof = dp.GPIOF.split(&mut ahb2);
    let mut gpiog = dp.GPIOG.split(&mut ahb2);

    // Could set to 115_200.bps for debugging
    let cfg = serial::Config::default().baudrate(57_600.bps());
    let adc_cfg = hal::adc::Config::default();
    //let pll_cfg = PllConfig::new(1, 8,PllDivider::Di);
    let common_cfg = hal::adc::CommonConfig::default();
    let clocks = rcc.cfgr.sysclk(80.mhz());
    let clocks = clocks.freeze(&mut acr,&mut pwr); 
    
    let mut adc1 = hal::adc::Adc::adc1(dp.ADC1,adc_cfg,&mut ahb2,&mut rcc.ccipr);
    //let mut user1_en = gpioc.pc9.into_push_pull_output(&mut gpioc.moder,&mut gpioc.otyper);
    let mut user4_en = gpiog.pg12.into_push_pull_output(&mut gpiog.moder,&mut gpiog.otyper);
    //let mut user3_en = gpiog.pg15.into_push_pull_output(&mut gpiog.moder,&mut gpiog.otyper);
    user4_en.set_high().unwrap();
    //user3_en.set_high().unwrap();
    //let mut usart2_en = gpiog.pg2.into_push_pull_output(&mut gpiog.moder, &mut gpiog.otyper);
    //usart2_en.set_high().unwrap();
    //user1_en.set_high().unwrap();
    
    let mut user4_4 = gpioc.pc5.into_analog_with_adc(&mut gpioc.moder,&mut gpioc.pupdr);
    //let mut user3_2 = gpiob.pb1.into_analog_with_adc(&mut gpiob.moder,&mut gpiob.pupdr);
    
    hal::adc::adc_global_setup(common_cfg,&mut ahb2,&mut rcc.ccipr);
    loop {
        let res = adc1.read(&mut user4_4);
        
        if res.is_ok() {
            let res:u16 = res.unwrap();
            hprintln!("VAL:{}", res).unwrap();
        }
        else {
            hprintln!("FAIL").unwrap();
        }
        cortex_m::asm::delay(8_000_00);
    }
    // Replacing the Shared Peripheral
    // Also change here to if you changed SharedPeripherals
    /* cortex_m::interrupt::free(|cs|{
        SHARED_PER.borrow(cs).replace(Some(
            SharedPeripherals{led1,led2,led3,led4,led5,adc1,potent:user4_4}
        ));
    }); */
}