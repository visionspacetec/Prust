//! Implementations of SpacePacket specific to PUS TM[1,1], TM[1,3], TM[1,7].
//! These are the situations where return indicates success without step_id field.
//! These builder functions returns error if it is not a valid CCSDS 133. 0-B-1 packet for TM[1,1], TM[1,3] or TM[1,7].
//! See page 483 of ECSS-E-ST-70-41C.
use super::*;

impl SpacePacket<TmPacket<ServiceSuccess>>{
    
    /// Wrapper for "new" function specific to TM[1,1].
    pub fn new_service_1_1 <T:Request>(
        request:&T,
        destination_id:u16,
        packet_name:u16
    ) -> Result<Self,Error>{
        SpacePacket::<TmPacket::<ServiceSuccess>>::new(request,1,destination_id,packet_name)
    }

    /// Wrapper for "new" function specific to TM[1,3].
    pub fn new_service_1_3 <T:Request>(
        request:&T,
        destination_id:u16,
        packet_name:u16
    ) -> Result<Self,Error>{
        SpacePacket::<TmPacket::<ServiceSuccess>>::new(request,3,destination_id,packet_name)
    }

    /// Wrapper for "new" function specific to TM[1,7].
    /// TM[1,7] successful completion of execution verification report
    pub fn new_service_1_7 <T:Request>(
        request:&T,
        destination_id:u16,
        packet_name:u16
    ) -> Result<Self,Error>{
        SpacePacket::<TmPacket::<ServiceSuccess>>::new(request,7,destination_id,packet_name)
    }


    /// Creates a the packet data structures with the given parameters
    /// 
    /// # Errors
    /// 
    /// Returns error if Primary or Secondary Headers are invalid
    pub fn  new<T:Request>(
        request:&T,
        message_subtype:u8,
        destination_id:u16,
        packet_name:u16
    ) -> Result<Self,Error >
        {
            let req_id = request.to_request();
            let data_len = TmPacketHeader::TM_HEADER_LEN + REQ_ID_LEN + crate::sp::PEC_LEN - 1;
            let data_len = data_len as u16;
            if message_subtype != 1 && message_subtype != 3 && message_subtype != 7 {
                return Err(Error::InvalidPacket);
            }
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
    /// if buffer.len() !=  PH_LEN (6) +  TM_HEADER_LEN (9) + PEC_LEN (2) +REQUEST_ID_LEN (4)
    /// or not compliant to message subtype is not 1,3 or 7
    pub fn from_bytes(buffer:&[u8]) -> Result<Self,Error> {
        if buffer.len() != PrimaryHeader::PH_LEN + TmPacketHeader::TM_HEADER_LEN + REQ_ID_LEN + PEC_LEN{
            return Err(Error::InvalidPacket);
        }
        let primary_header = PrimaryHeader::from_bytes(&buffer[..PrimaryHeader::PH_LEN])?;
        // If the primary header is not defined properly, give an error accordingly.
        // It has to be have sec_header_flag set, version no to 0, and for TM type_flag should be clear.
        if !primary_header.sec_header_flag || primary_header.ver_no != 0 || primary_header.type_flag {
            return Err(Error::InvalidPacket);
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
            return Err(Error::InvalidPacket);
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

