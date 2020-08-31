//! Implementations of SpacePacket specific to PUS TM[1,6].
//! These are the situations where return indicates success with step_id field.
//! These builder functions returns error if it is not a valid CCSDS 133. 0-B-1 packet for TM[1,6].
//! See page 483 of ECSS-E-ST-70-41C.
use super::*;

impl SpacePacket<TmPacket<ServiceFailStep>>{
    
    /// Wrapper for "new" function specific to TM[1,6].
    pub fn new_service_1_6 <T:Request>(
        request:&T,
        destination_id:u16,
        packet_name:u16,
        err_code:u8,
        err_data:Vec<u8>,
        step_id:u16
    ) -> Result<Self,()>{
        SpacePacket::<TmPacket::<ServiceFailStep>>::new(request,destination_id,packet_name,err_code,err_data,step_id)
    }
    
    /// Creates a the packet data structures with the given parameters
    /// 
    /// # Errors
    /// 
    /// Returns error if Primary or Secondary Headers are invalid
    pub fn  new<T:Request>(
        request:&T,
        destination_id:u16,
        packet_name:u16,
        err_code:u8,
        err_data:Vec<u8>,
        step_id:u16
    ) -> Result<Self,() >
    {
        let req_id = request.to_request();
        let data_len = TmPacketHeader::TM_HEADER_LEN + REQ_ID_LEN + err_data.len() + crate::sp::PEC_LEN;
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
        let header =  TmPacketHeader::new(SERVICE_TYPE,6,destination_id)?;
        Ok(
            SpacePacket::<TmPacket::<ServiceFailStep>>{
                primary_header,
                data:TmPacket{
                    header,
                    user_data: TxUserData::<ServiceFailStep>{
                        packet_error_control,
                        data:ServiceFailStep{
                            request_id:req_id,
                            failure_notice:FailureNotice::new(
                                err_code,err_data
                            )?,
                            step_id:StepId::new(step_id)?
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
    /// if the byte array is not compliant to TM[1,6]
    pub fn from_bytes(buffer:&[u8]) -> Result<Self,()> {
        if buffer.len() < PrimaryHeader::PH_LEN + TmPacketHeader::TM_HEADER_LEN + REQ_ID_LEN + FAILURE_NOTICE_MIN_LEN + PEC_LEN{
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
        || sec_header.message_subtype != 6 
        {
            return Err(());
        };
        let range_start = PrimaryHeader::PH_LEN+TmPacketHeader::TM_HEADER_LEN;
        let request_id = RequestId::from_bytes(&buffer[range_start..range_start+REQ_ID_LEN])?;
        let failure_notice_start = range_start+REQ_ID_LEN;
        if buffer[failure_notice_start] as usize > error::ERR_CODE_COUNT {
            return Err(());
        }
        let failure_notice_len = error::ERR_CODE_DATA_LEN[buffer[failure_notice_start] as usize] + 1;
        let failure_notice = FailureNotice::from_bytes(&buffer[failure_notice_start..failure_notice_start+failure_notice_len])?;
        
        let range_start = failure_notice_start+failure_notice_len;
        let step_id = StepId::from_bytes(&buffer[range_start..range_start+STEP_ID_LEN])?;
        
        let packet_error_control = BigEndian::read_u16(&buffer[(buffer.len()-PEC_LEN)..]);

        Ok(
            SpacePacket::<TmPacket::<ServiceFailStep>>{
                primary_header,
                data:TmPacket{
                    header:sec_header,
                    user_data: TxUserData::<ServiceFailStep>{
                        packet_error_control,
                        data:ServiceFailStep{
                            request_id,failure_notice,step_id
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
        bytes.extend(self.data.user_data.data.failure_notice.to_byte().to_vec());
        bytes.extend(self.data.user_data.data.step_id.to_bytes().to_vec());

        // add the two bytes then modify them to the true value.
        bytes.push(0);
        bytes.push(0);
        let pec_start = arr_len - crate::sp::PEC_LEN;
        BigEndian::write_u16(&mut bytes[pec_start..],self.data.user_data.packet_error_control);
        bytes
    }
}

