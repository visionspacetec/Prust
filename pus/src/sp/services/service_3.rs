//! Each packet transporting a housekeeping message shall be of service type 3.

use super::*;
/* use crate::error;
use crate::sp::{
    tm::{TmData,TmPacketHeader,TmPacket},
    tc::{TcData,TcPacketHeader,TcPacket},
    Request,PEC_LEN
}; */

const SERVICE_TYPE: u8 = 3;
pub mod service_3_1;
pub mod service_3_25;
pub mod service_3_27;
pub mod service_3_5;
pub mod service_3_5x6;
pub mod service_3_6;
