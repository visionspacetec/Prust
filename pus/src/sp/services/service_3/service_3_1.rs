//!TC[3,1] create a housekeeping parameter report structure
use super::*;
use tc::{TcData, TcPacket,TcPacketHeader};

/// TC[3,1] data field some fields are not imlemented.
#[derive(Debug)]
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
const MES_SUBTYPE:u8 = 1;

impl Service3_1 {
    /// For creating a TC[3,1] data structure
    /// TODO: IMPORTANT ID'S ARE NOT CHECKED
    pub fn new(
        housekeeping_report_id:u8,
        collection_interval:u8,
        n1:u8,
        parameter_ids:Vec<u8>
    ) -> Result<Self,Error> {
        if parameter_ids.len() != n1 as usize{
            return Err(Error::InvalidPacket);
        }
        Ok(
            Service3_1{
                housekeeping_report_id,collection_interval,n1,parameter_ids,nfa:0
            }
        )
    }
    /// For creating a TC[3,1] data structure. Wrapper of "new" function
    pub fn new_service_3_1(
        housekeeping_report_id:u8,
        collection_interval:u8,
        n1:u8,
        parameter_ids:Vec<u8>,
    ) -> Result<Self,Error> 
    {
        Service3_1::new(housekeeping_report_id,collection_interval,n1,parameter_ids)
    }
    
    pub(crate) fn from_bytes(buffer:&[u8]) -> Result<Self,Error>{
        if buffer.len() < CONST_LEN_TOT
        || buffer.len() != CONST_LEN_TOT + buffer[2] as usize {
            return Err(Error::InvalidPacket);
        }
        let housekeeping_report_id = buffer[0];
        let collection_interval = buffer[1];
        let n1 = buffer[2];
        let parameter_ids = buffer[3..buffer.len()-1].to_vec();
        let nfa = buffer[buffer.len()-1];
        if nfa != 0 {
            return Err(Error::InvalidPacket);
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
        CONST_LEN_TOT + n1 as usize + TcPacketHeader::TC_HEADER_LEN + PrimaryHeader::PH_LEN + PEC_LEN
    }
    fn header_is_tc_3_1(header:&PrimaryHeader)-> bool{
        header.sec_header_flag && header.ver_no == PrimaryHeader::VER_NO && header.type_flag 
    }
    fn sec_header_is_tc_3_1(header:&TcPacketHeader)-> bool{
        header.service_type == SERVICE_TYPE && header.message_subtype == MES_SUBTYPE
    }

}

impl TcData for Service3_1 {
    // empty
}

/// Implementations of SpacePacket specific to PUS and TC[3,1]
/// 
/// # Errors
/// 
/// If not a valid CCSDS 133. 0-B-1 packet for TC[3,1].
/// See page 483 of ECSS-E-ST-70-41C.
impl SpacePacket<TcPacket<Service3_1>>{

    pub fn new_service_3_1(
        apid:u16,
        packet_name:u16,
        housekeeping_report_id:u8,
        collection_interval:u8,
        n1:u8,
        parameter_ids:Vec<u8>
    ) -> Result<Self,Error> {
        SpacePacket::<TcPacket::<Service3_1>>::new(apid,packet_name,housekeeping_report_id,collection_interval,n1,parameter_ids)
    }

    /// For creating a TC[3,1] packet data structure 
    /// TODO: IMPORTANT ID'S ARE NOT CHECKED
    pub fn new(
        apid:u16,
        packet_name:u16,
        housekeeping_report_id:u8,
        collection_interval:u8,
        n1:u8,
        parameter_ids:Vec<u8>
    ) -> Result<Self,Error> {

        if parameter_ids.len() != n1 as usize{
            return Err(Error::InvalidPacket);
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
    
    /// Encodes the object to a byte vector
    pub fn to_bytes(&self) -> Vec<u8>{
        let arr_len = PrimaryHeader::PH_LEN + 1 + self.primary_header.data_len as usize;
        let mut bytes = Vec::with_capacity(arr_len);
        bytes.extend(self.primary_header.to_bytes().to_vec());
        bytes.extend(self.data.header.to_bytes().to_vec());
        bytes.extend(self.data.user_data.data.to_bytes());
        // add the two bytes then modify them to the true value.
        bytes.push(0);
        bytes.push(0);
        let pec_start = arr_len - PEC_LEN;
        BigEndian::write_u16(&mut bytes[pec_start..],self.data.user_data.packet_error_control);
        bytes
    }
    
    pub fn from_bytes(buffer:&[u8]) -> Result<Self,Error> {
        if buffer.len() < CONST_LEN_TOT {
            return Err(Error::InvalidPacket);
        }
        let primary_header = PrimaryHeader::from_bytes(&buffer[..PrimaryHeader::PH_LEN])?;
        // If the primary header is not defined properly, give an error accordingly.
        // It has to be have sec_header_flag set, version no to 0, and for TC type_flag should be set.
        if !Service3_1::header_is_tc_3_1(&primary_header) {
            return Err(Error::InvalidPacket);
        };
        let sec_header = TcPacketHeader::from_bytes(
            &buffer[PrimaryHeader::PH_LEN..PrimaryHeader::PH_LEN+TcPacketHeader::TC_HEADER_LEN]
        )?;
        if !Service3_1::sec_header_is_tc_3_1(&sec_header) {
            return Err(Error::InvalidPacket);
        }
        let range_start = TcPacketHeader::TC_HEADER_LEN+PrimaryHeader::PH_LEN;
        let service_data:Service3_1 = Service3_1::from_bytes(&buffer[range_start..buffer.len()-PEC_LEN])?;
        
        // implement this
        let packet_error_control = BigEndian::read_u16(&buffer[(buffer.len()-PEC_LEN)..]);
        Ok(
            SpacePacket{
                primary_header,
                data:TcPacket::<Service3_1> {
                    header:sec_header,
                    user_data:TxUserData::<Service3_1>{
                        packet_error_control,
                        data:service_data
                    }
                }
            }
        )
    }

    pub fn hk_id(&self) -> u8 {
        self.data.user_data.data.housekeeping_report_id
    }

    pub fn get_params(&self) -> &Vec<u8> {
        &self.data.user_data.data.parameter_ids
    }
}