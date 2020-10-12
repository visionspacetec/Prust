//! Functions are defined here
use super::*;

/// Parses the arguments for turn_led
pub fn pre_turn_led(args: &Vec<u8>) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::InvalidArg);
    } else {
        turn_led(args[0] != 0)
    }
}

/// Uses user1_1 from SHARED_PER global variable.
pub fn turn_led(turn: bool) -> Result<(), Error> {
    cortex_m::interrupt::free(|cs| -> Result<(), Error> {
        if turn {
            SHARED_PER
                .borrow(cs)
                .try_borrow_mut()?
                .as_mut()?
                .leds[0]
                .set_high()
                .unwrap();
            Ok(())
        } else {
            SHARED_PER
                .borrow(cs)
                .try_borrow_mut()?
                .as_mut()?
                .leds[0]
                .set_low()
                .unwrap();
            Ok(())
        }
    })
}

/// Parses the arguments for turn_led
pub fn pre_set_led(args: &Vec<u8>) -> Result<(), Error> {
    if args.len() != 2 && usize::from(args[1]) < LED_COUNT{
        return Err(Error::InvalidArg);
    } else {
        set_led(args[0], args[1] != 0)
    }
}
/// Uses user1_1, user1_2 from SHARED_PER global variable.
pub fn set_led(led_no: u8, turn: bool) -> Result<(), Error> {
    cortex_m::interrupt::free(|cs| -> Result<(), Error> {

        if turn {
            SHARED_PER
                .borrow(cs)
                .try_borrow_mut()?
                .as_mut()?
                .leds[led_no as usize]
                .set_high()
                .unwrap();
            Ok(())
        } else {
            SHARED_PER
                .borrow(cs)
                .try_borrow_mut()?
                .as_mut()?
                .leds[led_no as usize]
                .set_low()
                .unwrap();
            Ok(())
        }
    })
}
