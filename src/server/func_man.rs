use super::*;
use cortex_m::peripheral::NVIC;
use hal::adc::{OversamplingShift,RegularOversampling,Adc};
use stm32::ADC1;

const BLANK_VEC: [u8; FUNC_ID_LEN] = [0 as u8; FUNC_ID_LEN];
// A temp helper function
pub fn create_func_id(name: &str) -> FuncId {
    let mut res = String::from(name);
    res.push_str(&String::from_utf8(BLANK_VEC[name.len()..].to_vec()).unwrap());
    let res = FuncId::from(&res).unwrap();
    res
}

pub mod functions;

const LED_COUNT:usize = 3;
/// Change Here If An External Function Needs To Access Peripheral Data
pub struct SharedPeripherals {
    pub user1_en: PCx<Output<PushPull>>,
    pub user4_en: PGx<Output<PushPull>>,
    pub leds: [PFx<Output<PushPull>>;LED_COUNT],
    pub adc1: Adc<ADC1>,
    pub user4_4: PC5<Analog>,
}

pub fn init() -> UART5RXType {
    let dp = stm32::Peripherals::take().unwrap(); // get the device peripheral
    let mut rcc = dp.RCC.constrain(); // get the Rcc's abstract struct
    let mut ahb2 = rcc.ahb2;
    let mut apb1r1 = rcc.apb1r1;
    let flash = dp.FLASH.constrain();
    let mut acr = flash.acr;
    let mut pwr = dp.PWR.constrain(&mut apb1r1);

    // For powering all of the pins
    unsafe { ((*hal::device::PWR::ptr()).cr2).modify(|_, w| w.iosv().set_bit()) }

    // Configuring GPIO's
    let mut gpioc = dp.GPIOC.split(&mut ahb2);
    let mut gpiod = dp.GPIOD.split(&mut ahb2);
    let mut gpiof = dp.GPIOF.split(&mut ahb2);
    let mut gpiog = dp.GPIOG.split(&mut ahb2);

    // Could set to 57_200.bps for debugging
    let cfg = serial::Config::default().baudrate(57_600.bps());
    let adc_cfg = hal::adc::Config::default();
    let clocks = rcc.cfgr.sysclk(SYS_FREQ);
    let clocks = clocks.freeze(&mut acr, &mut pwr);

    // Enabling timer
    unsafe { NVIC::unmask(hal::stm32::Interrupt::TIM2) };
    let timer = Timer::tim2(dp.TIM2, MIN_SAMPL_DIV.hz(), clocks, &mut apb1r1);
    // Setting ADC1
    let mut adc1 = hal::adc::Adc::adc1(dp.ADC1, adc_cfg, &mut ahb2, &mut rcc.ccipr);
    adc1.set_oversampling_shift(OversamplingShift::S0);
    adc1.set_regular_oversampling(RegularOversampling::On);
    let mut user3_en = gpiog
        .pg15
        .into_push_pull_output(&mut gpiog.moder, &mut gpiog.otyper)
        .downgrade();
    let mut user4_en = gpiog
        .pg12
        .into_push_pull_output(&mut gpiog.moder, &mut gpiog.otyper)
        .downgrade();
    let mut user1_en = gpioc
        .pc9
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper)
        .downgrade();
    // Setting enable pins
    user4_en.set_high().unwrap();
    user1_en.set_high().unwrap();
    user3_en.set_high().unwrap();
    // Enabling uart
    let uart5 = hal::serial::Serial::uart5(
        dp.UART5,
        (
            gpioc.pc12.into_af8(&mut gpioc.moder, &mut gpioc.afrh),
            gpiod.pd2.into_af8(&mut gpiod.moder, &mut gpiod.afrl),
        ),
        cfg,
        clocks,
        &mut apb1r1,
    );
    let (tx, rx) = uart5.split();
    // Configuring used pins
    let user1_1 = gpiof
        .pf13
        .into_push_pull_output(&mut gpiof.moder, &mut gpiof.otyper)
        .downgrade();
    let user1_2 = gpiof
        .pf15
        .into_push_pull_output(&mut gpiof.moder, &mut gpiof.otyper)
        .downgrade();  
    let user3_1 = gpiof
        .pf12
        .into_push_pull_output(&mut gpiof.moder, &mut gpiof.otyper)
        .downgrade();

    let leds = [user1_1, user1_2, user3_1];

    let user4_4 = gpioc
        .pc5
        .into_analog_with_adc(&mut gpioc.moder, &mut gpioc.pupdr);
    // Replacing the Shared Peripheral
    // Also change here to if you changed SharedPeripherals
    cortex_m::interrupt::free(|cs| {
        SHARED_PER.borrow(cs).replace(Some(SharedPeripherals {
            user1_en,
            user4_en,
            leds,
            adc1,
            user4_4,
        }));
    });

    // Initializing UART5 global variable
    cortex_m::interrupt::free(|cs| {
        UART5TX.borrow(cs).replace(Some(tx));
    });
    // Initializing TIMER7 global variable
    cortex_m::interrupt::free(|cs| {
        TIMER2.borrow(cs).replace(Some(timer));
    });
    rx
}
