use stm32l4xx_hal as hal; // HAL alias

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