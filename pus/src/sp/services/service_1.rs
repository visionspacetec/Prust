//! Each packet transporting a request verification report shall be of service type 1.
use super::*;
use crate::error;
use crate::sp::{
    tm::{TmData,TmPacketHeader,TmPacket},
    Request,PEC_LEN
};

const SERVICE_TYPE:u8 = 1;

/* Field sizes in terms of bytes */
const REQ_ID_LEN:usize = 4;
const STEP_ID_LEN:usize = 2; 
const FAILURE_NOTICE_MIN_LEN:usize = 1;

/*---- Field Structs Of Messages Start ----*/

/// To Identify which request it is. Used in all TM[1,x] packs where x = {1,2,3,4,5,6,7,8,10}.
#[derive(Debug)]
pub struct RequestId{
    /// first 3 bits of the LSB are used.
    pub(crate) ver_no:u8,
    pub(crate) packet_type:bool,
    pub(crate) sec_header_flag:bool,
    /// first 11 bits of the LSB are used.
    pub(crate) apid:u16,
    pub(crate) seq_flags:(bool,bool),
    /// first 14 bits of the LSB are used.
    pub(crate) packet_seq_count:u16
}
impl RequestId {
    fn to_bytes(&self) -> [u8;REQ_ID_LEN] {
        let mut bytes = [0;REQ_ID_LEN];
        let mut first_two_bytes = (self.ver_no as u16) << 13;
        if self.packet_type {
            first_two_bytes = first_two_bytes | 0b0001_0000_0000_0000;
        }
        if self.sec_header_flag {
            first_two_bytes = first_two_bytes | 0b0000_1000_0000_0000;
        }
        first_two_bytes += self.apid;
        let mut second_two_bytes = self.packet_seq_count;
        if self.seq_flags.0{
            second_two_bytes = second_two_bytes | 0b1000_0000_0000_0000
        }
        if self.seq_flags.1{
            second_two_bytes = second_two_bytes | 0b0100_0000_0000_0000
        }
        BigEndian::write_u16(&mut bytes[..2], first_two_bytes);
        BigEndian::write_u16(&mut bytes[2..], second_two_bytes);
        bytes
    }

    fn from_bytes(buffer:&[u8]) -> Result<Self,Error>{
        // TODO do other checks here
        if buffer.len() != REQ_ID_LEN {
            return Err(Error::InvalidPacket);
        };
        // first 3 bits
        let ver_no = buffer[0] >> 5;
        let packet_type = (buffer[0] & 0b0001_0000) != 0;
        let sec_header_flag = (buffer[0] & 0b0000_1000) != 0;
        let apid = buffer[1] as u16;
        let apid = apid + (((buffer[0] & 0b0000_0111) as u16) << 8);
        let seq_flags = ((0b1000_0000 & buffer[2] != 0),(0b0100_0000 & buffer[2] != 0));
        let packet_seq_count = BigEndian::read_u16(&buffer[2..]);
        let packet_seq_count = packet_seq_count & 0b0011_1111_1111_1111;
        Ok(
            RequestId{
                ver_no,packet_type,sec_header_flag,apid,seq_flags,packet_seq_count
            }
        )
    }
}

/// Failure Notice Field Struct. Used in all fail response packs. TM[1,x] packs where x = {2,4,6,8,10}.
#[derive(Debug)]
struct FailureNotice{
    pub(crate) err_code:u8,
    pub(crate) err_data:Vec<u8>
}
impl FailureNotice{
    /// Creates a Failure Notice Field Struct.
    /// TODO check values
    pub fn new(err_code:u8,err_data:Vec<u8>) -> Result<Self,Error>{
        Ok(FailureNotice{
            err_code,err_data
        })
    }
    pub fn from_bytes(buffer:&[u8])->Result<Self,Error>{
        if buffer.len() < FAILURE_NOTICE_MIN_LEN || (buffer[0] as usize) >= error::ERR_CODE_COUNT {
            return Err(Error::InvalidPacket);
        };
        let err_code = buffer[0];
        let err_data = buffer[1..].to_vec();
        Ok(
            FailureNotice{
                err_code,err_data
            }
        )
    }
    pub fn to_byte(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.err_data.len()+1);
        bytes.push(self.err_code);
        bytes.extend(self.err_data.to_vec());
        bytes
    } 
}
/// Step id field seen in TM[1,5] and TM[1,6]
#[derive(Debug)]
struct StepId{
    pub step_id:u16,
}
impl StepId{
    /// Creates a Step Id Field Struct.
    /// TODO learn context
    pub fn new(step_id:u16) ->Result<Self,Error>{
        Ok(StepId{step_id})
    }
    pub fn from_bytes(buffer:&[u8]) -> Result<Self,Error>{
        if buffer.len() != STEP_ID_LEN{
            return Err(Error::InvalidPacket);
        }
        let step_id = BigEndian::read_u16(&buffer);
        Ok(StepId{
            step_id
        })
    }
    pub fn to_bytes(&self)-> Vec<u8>{
        let mut bytes = alloc::vec![0 as u8; STEP_ID_LEN];
        BigEndian::write_u16(&mut bytes,self.step_id);
        bytes
    }
}
/*---- Field Structs Of Messages End ----*/

/*---- PUS TM[1,1], TM[1,3] and TM[1,7] Packets Declaration Start ----*/
#[derive(Debug)]
pub struct ServiceSuccess{
    request_id:RequestId
}
// Getting TmData Trait
impl TmData for ServiceSuccess{ /* Empty */ }

/// Source data for PUS TM[1,1] packet.
/// PUS TM[1,1] successful acceptance verification report
pub type Service1_1 = ServiceSuccess;
/// Source data for PUS TM[1,3] packet.
/// PUS TM[1,3] successful start of execution verification report
pub type Service1_3 = ServiceSuccess;
/// Source data for PUS TM[1,7] packet.
/// PUS TM[1,7] successful completion of execution verification report
pub type Service1_7 = ServiceSuccess;
/*---- PUS TM[1,1], TM[1,3] and TM[1,7] Packets Declaration End ----*/

/*---- PUS TM[1,2], TM[1,4], TM[1,8]and TM[1,10] Packets Declaration Start ----*/
#[derive(Debug)]
pub struct ServiceFail{
    request_id:RequestId,
    failure_notice:FailureNotice
}
// Getting TmData Trait
impl TmData for ServiceFail{ /* Empty */ }
/// Source data for PUS TM[1,2] packet.
/// PUS TM[1,2] failed acceptance verification report
pub type Service1_2 = ServiceFail;
/// Source data for PUS TM[1,4] packet.
/// TM[1,4] failed start of execut<<ion verification report
pub type Service1_4 = ServiceFail;
/// Source data for PUS TM[1,8] packet.
/// PUS TM[1,8] failed completion of execution verification report
pub type Service1_8 = ServiceFail;
/// Source data for PUS TM[1,10] packet.
/// PUS TM[1,10] failed routing verification report
pub type Service1_10 = ServiceFail;
/*---- PUS TM[1,2], TM[1,4], TM[1,8]and TM[1,10] Packets Declaration End ----*/

/*---- PUS TM[1,5] Packet Declaration Start ----*/
#[derive(Debug)]
pub struct ServiceSuccessStep {
    request_id:RequestId,
    step_id:StepId
}
// Getting TmData Trait
impl TmData for ServiceSuccessStep{ /* Empty */ }

/// Source data for PUS TM[1,5] packet.
/// PUS TM[1,5] successful progress of execution verification report
pub type Service1_5 = ServiceSuccessStep;
/*---- PUS TM[1,5] Packet Declaration End ----*/

/*---- PUS TM[1,6] Packet Declaration Start ----*/
#[derive(Debug)]
pub struct ServiceFailStep{
    request_id:RequestId,
    step_id:StepId,
    failure_notice:FailureNotice
}
// Getting TmData Trait
impl TmData for ServiceFailStep{ /* Empty */ }
/// Source data for PUS TM[1,6] packet.
/// PUS TM[1,6] failed progress of execution verification report
pub type Service1_6 = ServiceFailStep;
/*---- PUS TM[1,6] Packet Declaration End ----*/


pub mod service_success;
pub mod service_success_step;
pub mod service_fail;
pub mod service_fail_step;

// TODO: improve aliases