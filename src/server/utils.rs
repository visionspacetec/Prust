use super::*;
use byteorder::{ByteOrder,BigEndian};

// Helper functions to check the bits if its ok to read from usart
pub fn is_not_ok_to_read_uart5() -> bool {
    let isr = unsafe { &(*hal::device::UART5::ptr()).isr.read() };
    isr.rxne().bit_is_clear() && isr.ore().bit_is_clear()
}

pub fn is_not_ok_to_write_uart5() -> bool {
    let isr = unsafe { &(*hal::device::UART5::ptr()).isr.read() };
    isr.txe().bit_is_clear()
}

// Some debugging functions. Also to illustrate how registers are manipulated
pub fn _set_oversampling8() {
    unsafe { &(*hal::stm32::UART5::ptr()).cr1.modify(|_,w| w.over8().set_bit()) };
}

pub fn _is_oversampling8() -> bool {
    *unsafe { &(*hal::stm32::UART5::ptr()).cr1.read().over8().is_oversampling8()}
}

pub fn _get_baudrate() -> u32 {
    *unsafe { &(*hal::stm32::UART5::ptr()).brr.read().bits()}
}
// if its tm or tc
pub fn mes_type_from_bytes(buf:&[u8]) -> (u8,u8){
    (buf[PrimaryHeader::PH_LEN+1],buf[PrimaryHeader::PH_LEN+2])
}

pub fn get_param_0() -> Result<u16,Error> {
    cortex_m::interrupt::free(|cs| -> Result<u16,Error> 
        {
            let mut res:u16 = 0;
            if let Some(sp) = SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut() {
                res = sp.adc1.read(&mut sp.user4_4).unwrap();
            }
            Ok(res)
        }   
    )
}

pub fn generate_one_shot_report(tc:&Tc3_27,report:&mut Vec<u8>){
    let hk_structs = tc.get_hk_report_struct_ids();
    free(
        |cs| {
            let hk_reports = HK_REPORTS.borrow(cs).try_borrow_mut().unwrap();
            for i in hk_structs.iter() {
                // TODO: Doesn't check if invalid id
                match hk_reports.get(i)
                {
                   Some((sp,_)) => {
                       let mut res = Vec::<u8>::new();
                       for &p in sp.get_params().iter() {
                           match p {       
                                0 => {
                                    res.push(0);
                                    res.push(0);
                                    let len = res.len();
                                    BigEndian::write_u16(&mut res[len-2..len],get_param_0().unwrap());
                                },
                                _=> continue
                            }
                        }
        
                        let tm3_25 = Tm3_25::new_service_3_25(
                            42,
                            0,
                            *i, 
                            res.to_vec());
                        match tm3_25 {
                            Err(e) => {
                                let (err_code,err_data) = error::get_err_code_n_data(e);
                                let err_report = SpacePacket::<_>::new_service_1_8(
                                    sp,0,0,err_code,err_data
                                ).unwrap();
                                report.extend(err_report.to_bytes().iter());
                            }
                            Ok(tm) => {
                                report.extend(tm.to_bytes());
                            }
                        };
                    }
                    
                    None => {}
                }
        }
    });
    
    //} 
}