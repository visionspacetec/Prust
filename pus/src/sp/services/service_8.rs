//! Each packet transporting a function management message shall be of service type 8.
use super::*;
use crate::{sp::{PEC_LEN},FuncId,FUNC_ID_LEN};
use tc::{TcData,TcPacket,TcPacketHeader};
use crate::sp::alloc::borrow::ToOwned;
use crate::sp::alloc::string::ToString;

pub(crate) const SERVICE_TYPE:u8 = 8;

/// User data. Used as application data on TC and source data as TM.
pub struct Service8_1{
    func_id:crate::FuncId,
    n:u8,
    /// Contains argument id and argument value.
    args_field:Vec<u8>
}

impl TcData for Service8_1 {
    // empty
}

/// Implementations of SpacePacket specific to PUS and TC[8,1]
/// 
/// # Errors
/// 
/// If not a valid CCSDS 133. 0-B-1 packet for TC[8,1].
/// See page 483 of ECSS-E-ST-70-41C.
impl SpacePacket<TcPacket<Service8_1>>{
    const MES_SUBTYPE:u8 = 1;

    pub fn from_bytes(buffer:&[u8]) -> Result<Self,Error> {
        if buffer.len() < 7 {
            return Err(Error::InvalidPacket);
        }
        let primary_header = PrimaryHeader::from_bytes(&buffer[..PrimaryHeader::PH_LEN])?;
        // If the primary header is not defined properly, give an error accordingly.
        // It has to be have sec_header_flag set, version no to 0, and for TC type_flag should be set.
        if !primary_header.sec_header_flag || primary_header.ver_no != 0 || !primary_header.type_flag {
            return Err(Error::InvalidPacket);
        };
        let sec_header = TcPacketHeader::from_bytes(
            &buffer[PrimaryHeader::PH_LEN..PrimaryHeader::PH_LEN+TcPacketHeader::TC_HEADER_LEN]
        )?;
        if sec_header.service_type != SERVICE_TYPE || sec_header.message_subtype != SpacePacket::MES_SUBTYPE {
            return Err(Error::InvalidPacket);
        }
        // slice for the range of func_len
        let range_start = TcPacketHeader::TC_HEADER_LEN+PrimaryHeader::PH_LEN;
        let func_range = range_start..range_start+FUNC_ID_LEN;
        let func_id_slice = &buffer[func_range];
        if !func_id_slice.is_ascii() {
            return Err(Error::InvalidPacket);
        }
        let func_fixed_slice = array_ref![buffer,range_start,FUNC_ID_LEN];
        let func_id = FuncId::from_byte_string(func_fixed_slice);
        if func_id.is_err(){
            return Err(Error::InvalidPacket);
        }
        let func_id = func_id.unwrap();
        let range_start = range_start + FUNC_ID_LEN + 1;
        let args_field = buffer[range_start..(buffer.len()-PEC_LEN)].to_vec();
        let app_data = Service8_1 {func_id,n:buffer[range_start-1],args_field};
        
        let packet_error_control = BigEndian::read_u16(&buffer[(buffer.len()-PEC_LEN)..]);
        Ok(
            SpacePacket{
                primary_header,
                data:TcPacket::<Service8_1> {
                    header:sec_header,
                    user_data:TxUserData::<Service8_1>{
                        packet_error_control,
                        data:app_data
                    }
                }
            }
        )
    }
    
    pub fn new_service_8_1(
        apid:u16,
        packet_name:u16,
        func_id:String,
        n:u8,
        args_field:Vec<u8>) -> Result<Self,Error>
    {
            SpacePacket::<TcPacket::<Service8_1>>::new(apid,packet_name,func_id,n,args_field)
    }


    ///
    /// # Errors
    /// 
    /// returns error if func_id.len() > FUNC_ID_LEN
    /// or returns error if Primary or Secondary Headers are invalid
    /// 
    pub fn new(
        apid:u16,
        packet_name:u16,
        func_id:String,
        n:u8,
        args_field:Vec<u8>) -> Result<Self,Error>
        {
            // +1 is for N field
            let data_len:usize = PrimaryHeader::PH_LEN  + 
                TcPacketHeader::TC_HEADER_LEN  + crate::sp::PEC_LEN + 
                FUNC_ID_LEN + 1 + args_field.len(); 
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
                8,
                1,0
            )?;
            if !func_id.is_ascii() || func_id.len() > FUNC_ID_LEN{
                return Err(Error::InvalidFuncId(func_id.as_str().to_owned()));
            }
            // Convert to fixed size string structure
            let func_id = FuncId::from(func_id.as_str());
            if func_id.is_err(){
                return Err(Error::CapacityError);
            }
            let mut func_id = func_id.unwrap();
            // Add 0 for remaining parts
            for _i in func_id.len()..FUNC_ID_LEN {
                func_id.push(0 as char);
            } 
            Ok(
                SpacePacket{
                    primary_header,
                    data:TcPacket{
                        header:sec_header,
                        user_data:TxUserData::<Service8_1>{
                            // TODO don't ignore
                            packet_error_control:0,
                            data:Service8_1{
                                func_id,
                                n,
                                args_field
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
        bytes.extend(self.data.user_data.data.func_id.bytes());
        for _i in self.data.user_data.data.func_id.len()..FUNC_ID_LEN{
            bytes.push(0);
        }
        bytes.push(self.data.user_data.data.n);
        bytes.extend(self.data.user_data.data.args_field.to_vec());
        // add the two bytes then modify them to the true value.
        bytes.push(0);
        bytes.push(0);
        let pec_start = arr_len - PEC_LEN;
        BigEndian::write_u16(&mut bytes[pec_start..],self.data.user_data.packet_error_control);
        bytes
    }

    pub fn exec_func(&self,func_map:&hashbrown::HashMap::<FuncId,fn(&Vec::<u8>)->Result<(),Error>>) -> Result<(),Error>{
        let func_id = self.data.user_data.data.func_id;
        if let Some (to_exec) = func_map.get(func_id.as_str()){
            to_exec(&self.data.user_data.data.args_field)?;
            Ok(())
        }
        else {
            Err(Error::InvalidFuncId(func_id.as_str().to_string().to_owned()))
        }
    }
}