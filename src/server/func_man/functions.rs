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

/// FuncId = "turn_led"
/// Input = bool
pub fn turn_led(turn: bool) -> Result<(), Error> {
    cortex_m::interrupt::free(|cs| -> Result<(), Error> {
        if turn {
            SHARED_PER
                .borrow(cs)
                .try_borrow_mut()?
                .as_mut()?
                .user1_1
                .set_high()
                .unwrap();
            Ok(())
        } else {
            SHARED_PER
                .borrow(cs)
                .try_borrow_mut()?
                .as_mut()?
                .user1_1
                .set_low()
                .unwrap();
            Ok(())
        }
    })
}

/// Parses the arguments for turn_led
pub fn pre_set_led(args: &Vec<u8>) -> Result<(), Error> {
    if args.len() != 2 {
        return Err(Error::InvalidArg);
    } else {
        set_led(args[0], args[1] != 0)
    }
}
/// FuncId = "set_led"
/// Input = u8,bool
pub fn set_led(led_no: u8, turn: bool) -> Result<(), Error> {
    cortex_m::interrupt::free(|cs| -> Result<(), Error> {
        if led_no == 0 {
            if turn {
                SHARED_PER
                    .borrow(cs)
                    .try_borrow_mut()?
                    .as_mut()?
                    .user1_1
                    .set_high()
                    .unwrap();
                Ok(())
            } else {
                SHARED_PER
                    .borrow(cs)
                    .try_borrow_mut()?
                    .as_mut()?
                    .user1_1
                    .set_low()
                    .unwrap();
                Ok(())
            }
        } else if led_no == 1 {
            if turn {
                SHARED_PER
                    .borrow(cs)
                    .try_borrow_mut()?
                    .as_mut()?
                    .user1_2
                    .set_high()
                    .unwrap();
                Ok(())
            } else {
                SHARED_PER
                    .borrow(cs)
                    .try_borrow_mut()?
                    .as_mut()?
                    .user1_2
                    .set_low()
                    .unwrap();
                Ok(())
            }
        } else {
            Err(Error::InvalidArg)
        }
    })
}
