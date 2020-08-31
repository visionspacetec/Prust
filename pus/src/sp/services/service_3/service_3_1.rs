//!TC[3,1] create a housekeeping parameter report structure
use super::*;
use tc::{TcData, TcPacket,TcPacketHeader};

/// TC[3,1] data field some fields are not imlemented.
pub struct Service3_1{
    pub(crate) housekeeping_report_id:u8,
    // not implemented
    pub(crate) collection_interval:u8,
    pub(crate) n1:u8,
    pub(crate) parameter_ids:Vec<u8>,
    // not supported, should be always 0
    pub(crate) nfa:u8
    // Rest fields are not implemented.
}

const CONST_LEN_TOT:usize = 4;

impl Service3_1 {
    /// For creating a TM[3,1] data structure
    /// TODO: IMPORTANT ID'S ARE NOT CHECKED
    pub fn new(
        housekeeping_report_id:u8,
        collection_interval:u8,
        n1:u8,
        parameter_ids:Vec<u8>
    ) -> Result<Self,()> {
        if parameter_ids.len() != n1 as usize{
            return Err(());
        }
        Ok(
            Service3_1{
                housekeeping_report_id,collection_interval,n1,parameter_ids,nfa:0
            }
        )
    }
    /// For creating a TM[3,1] data structure. Wrapper of "new" function
    pub fn new_service_3_1(
        housekeeping_report_id:u8,
        collection_interval:u8,
        n1:u8,
        parameter_ids:Vec<u8>,
    ) -> Result<Self,()> 
    {
        Service3_1::new(housekeeping_report_id,collection_interval,n1,parameter_ids)
    }
    
    pub(crate) fn from_bytes(buffer:&[u8]) -> Result<Self,()>{
        if buffer.len() < CONST_LEN_TOT
        || buffer.len() != CONST_LEN_TOT + buffer[2] as usize {
            return Err(());
        }
        let housekeeping_report_id = buffer[0];
        let collection_interval = buffer[1];
        let n1 = buffer[2];
        let parameter_ids = buffer[3..buffer.len()-1].to_vec();
        let nfa = buffer[buffer.len()-1];
        if nfa != 0 {
            return Err(());
        }
        Ok(
            Service3_1{
                housekeeping_report_id,collection_interval,n1,parameter_ids,nfa
            }
        )
    }
    
    pub(crate) fn to_bytes(&self) -> Vec<u8>{
        let mut bytes = Vec::with_capacity(
            CONST_LEN_TOT + self.n1 as usize
        );
        bytes.push(self.housekeeping_report_id);
        bytes.push(self.collection_interval);
        bytes.push(self.n1);
        bytes.extend(self.parameter_ids.to_vec());
        bytes.push(self.nfa);
        bytes
    }
    fn get_packet_len(n1:u8) -> usize {
        CONST_LEN_TOT + n1 as usize + TcPacketHeader::TC_HEADER_LEN + PrimaryHeader::PH_LEN
    }
    
}

impl TcData for Service3_1 {
    // empty
}
const MES_SUBTYPE:u8 = 1;
/// Implementations of SpacePacket specific to PUS and TC[3,1]
/// 
/// # Errors
/// 
/// If not a valid CCSDS 133. 0-B-1 packet for TC[3,1].
/// See page 483 of ECSS-E-ST-70-41C.
impl SpacePacket<TcPacket<Service3_1>>{

    /// For creating a TM[3,1] packet data structure 
    /// TODO: IMPORTANT ID'S ARE NOT CHECKED
    pub fn new(
        apid:u16,
        packet_name:u16,
        housekeeping_report_id:u8,
        collection_interval:u8,
        n1:u8,
        parameter_ids:Vec<u8>
    ) -> Result<Self,()> {

        if parameter_ids.len() != n1 as usize{
            return Err(());
        }
        let data_len = Service3_1::get_packet_len(n1);
        let primary_header = PrimaryHeader::new(
            PrimaryHeader::VER_NO,
            true,
            true,
            apid,
            (true,true),
            packet_name,
            (data_len - PrimaryHeader::PH_LEN - 1) as u16
        )?;
        let sec_header = TcPacketHeader::new(
            (false,false,false,false),
            3,
            1,0
        )?;
        let data =Service3_1::new(housekeeping_report_id, collection_interval, n1, parameter_ids)?;
        Ok(
            SpacePacket{
                primary_header,
                data:TcPacket{
                    header:sec_header,
                    user_data:TxUserData::<Service3_1>{
                        // TODO don't ignore
                        packet_error_control:0,
                        data
                    }
                }
            }
        )
    }
    
    
}