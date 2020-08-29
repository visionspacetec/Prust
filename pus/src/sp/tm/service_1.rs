use super::*;
use crate::sp::{Request,PEC_LEN};

const SERVICE_TYPE:u8 = 1;

/* Field sizes in terms of bytes */
const REQ_ID_LEN:usize = 4;
const STEP_ID_LEN:usize = 2; 
const FAILURE_NOTICE_MIN_LEN:usize = 1;

/*---- Field Structs Of Messages Start ----*/

/// To Identify which request it is. Used in all TM[1,x] packs where x = {1,2,3,4,5,6,7,8,10}.
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

    fn from_bytes(buffer:&[u8]) -> Result<Self,()>{
        // TODO do other checks here
        if buffer.len() != REQ_ID_LEN {
            return Err(());
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
struct FailureNotice{
    err_code:u8,
    err_data:Vec<u8>
}
impl FailureNotice{
    /// Creates a Failure Notice Field Struct.
    /// TODO check values
    pub fn new(err_code:u8,err_data:Vec<u8>) -> Result<Self,()>{
        Ok(FailureNotice{
            err_code,err_data
        })
    }
}
/// Step id field seen in TM[1,5] and TM[1,6]
struct StepId{
    step_id:u16,
}
impl StepId{
    /// Creates a Step Id Field Struct.
    /// TODO learn context
    pub fn new(step_id:u16) ->Result<Self,()>{
        Ok(StepId{step_id})
    }
}
/*---- Field Structs Of Messages End ----*/

/*---- PUS TM[1,1], TM[1,3] and TM[1,7] Packets Declaration Start ----*/
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
/*---- PUS TM[1,6] Packet Declaration Start ----*/

/// Implementations of SpacePacket specific to PUS TM[1,1], TM[1,3], TM[1,7].
/// These are the situations where return indicates success without step_id field.
/// 
/// #Errors
/// 
/// If not a valid CCSDS 133. 0-B-1 packet for TM[1,1], TM[1,3] or TM[1,7].
/// See page 483 of ECSS-E-ST-70-41C.
impl SpacePacket<TmPacket<ServiceSuccess>>{
    
    ///
    /// 
    /// # Errors
    /// 
    /// Returns error if Primary or Secondary Headers are invalid
    /// 
    pub fn  new<T:Request>(request:&T,message_subtype:u8,destination_id:u16,packet_name:u16) -> Result<Self,() >{
            let req_id = request.to_request();
            let data_len = PrimaryHeader::PH_LEN + TmPacketHeader::TM_HEADER_LEN + REQ_ID_LEN + crate::sp::PEC_LEN;
            let data_len = data_len as u16;
            // TODO: Implement this feature
            let packet_error_control = 0;
            let primary_header = PrimaryHeader::new(
                PrimaryHeader::VER_NO,
                false,
                true,
                req_id.apid,
                (true,true),
                packet_name,
                data_len
            )?;
            let header =  TmPacketHeader::new(SERVICE_TYPE,message_subtype,destination_id)?;
            Ok(
                SpacePacket::<TmPacket::<ServiceSuccess>>{
                    primary_header,
                    data:TmPacket{
                        header,
                        user_data: TxUserData::<ServiceSuccess>{
                            packet_error_control,
                            data:ServiceSuccess{
                                request_id:req_id
                            }
                        }
                    }
                }
            )
        
    }
    /// Creates the struct from a byte array slice
    /// 
    /// Errors
    /// 
    /// if buffer.len() !=  PH_LEN (6) +  TM_HEADER_LEN (9) + REQUEST_ID_LEN (4)
    /// or message subtype is not 1,3 or 7
    pub fn from_bytes(buffer:&[u8]) -> Result<Self,()> {
        if buffer.len() != PrimaryHeader::PH_LEN + TmPacketHeader::TM_HEADER_LEN + REQ_ID_LEN {
            return Err(());
        }
        let primary_header = PrimaryHeader::from_bytes(&buffer[..PrimaryHeader::PH_LEN])?;
        // If the primary header is not defined properly, give an error accordingly.
        // It has to be have sec_header_flag set, version no to 0, and for TM type_flag should be clear.
        if !primary_header.sec_header_flag || primary_header.ver_no != 0 || primary_header.type_flag {
            return Err(());
        };
        let sec_header = TmPacketHeader::from_bytes(
            &buffer[PrimaryHeader::PH_LEN..PrimaryHeader::PH_LEN+TmPacketHeader::TM_HEADER_LEN]
        )?;
        // If service type or message subtype doesn't match 
        if sec_header.service_type != SERVICE_TYPE 
            || !(sec_header.message_subtype == 1 
            || sec_header.message_subtype == 3 
            || sec_header.message_subtype == 7)
        {
            return Err(());
        };
        let req_id_start = PrimaryHeader::PH_LEN+TmPacketHeader::TM_HEADER_LEN;
        let req_id = RequestId::from_bytes(&buffer[req_id_start..req_id_start+REQ_ID_LEN])?;
        let packet_error_control = BigEndian::read_u16(&buffer[(buffer.len()-PEC_LEN)..]);
        Ok(
            SpacePacket::<TmPacket::<ServiceSuccess>>{
                primary_header,
                data:TmPacket{
                    header:sec_header,
                    user_data: TxUserData::<ServiceSuccess>{
                        packet_error_control,
                        data:ServiceSuccess{
                            request_id:req_id
                        }
                    }
                }
            }
        )
    }
    
    /// Encodes the object to a byte vector
    pub fn to_bytes(&self) -> Vec<u8>{
        let arr_len = PrimaryHeader::PH_LEN + 1 + self.primary_header.data_len as usize;
        let mut bytes = Vec::with_capacity(arr_len);
        bytes.extend(self.primary_header.to_bytes().to_vec());
        bytes.extend(self.data.header.to_bytes().to_vec());
        bytes.extend(self.data.user_data.data.request_id.to_bytes().to_vec());
        // add the two bytes then modify them to the true value.
        bytes.push(0);
        bytes.push(0);
        let pec_start = arr_len - crate::sp::PEC_LEN;
        BigEndian::write_u16(&mut bytes[pec_start..],self.data.user_data.packet_error_control);
        bytes
    }
}