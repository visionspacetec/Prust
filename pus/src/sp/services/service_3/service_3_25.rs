//!TM[3,25] create a housekeeping parameter report structure
use super::*;
use tm::{TmData, TmPacket, TmPacketHeader};

#[derive(Debug)]
pub struct Service3_25 {
    pub(crate) housekeeping_id: u8,
    pub(crate) parameter_value: Vec<u8>,
}

const CONST_LEN_TOT: usize = 1;
const MES_SUBTYPE: u8 = 25;

impl Service3_25 {
    /// For creating a TM[3,25] data structure
    /// TODO: IMPORTANT ID'S ARE NOT CHECKED
    pub fn new(housekeeping_id: u8, parameter_value: Vec<u8>) -> Result<Self, Error> {
        if parameter_value.len() == 0 {
            return Err(Error::InvalidPacket);
        }
        Ok(Service3_25 {
            housekeeping_id,
            parameter_value,
        })
    }
    /// For creating a TM[3,25] data structure. Wrapper of "new" function
    pub fn new_service_3_25(housekeeping_id: u8, parameter_value: Vec<u8>) -> Result<Self, Error> {
        Service3_25::new(housekeeping_id, parameter_value)
    }

    pub(crate) fn from_bytes(buffer: &[u8]) -> Result<Self, Error> {
        if buffer.len() < CONST_LEN_TOT {
            return Err(Error::InvalidPacket);
        }
        let housekeeping_id = buffer[0];
        let parameter_value = buffer[1..].to_vec();
        Ok(Service3_25 {
            housekeeping_id,
            parameter_value,
        })
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(CONST_LEN_TOT + self.parameter_value.len());
        bytes.push(self.housekeeping_id);
        bytes.extend(self.parameter_value.to_vec());
        bytes
    }
    fn get_packet_len(parameter_value: &Vec<u8>) -> usize {
        CONST_LEN_TOT
            + parameter_value.len()
            + TmPacketHeader::TM_HEADER_LEN
            + PrimaryHeader::PH_LEN
            + PEC_LEN
    }
    fn header_is_tc_3_25(header: &PrimaryHeader) -> bool {
        header.sec_header_flag && header.ver_no == PrimaryHeader::VER_NO && !header.type_flag
    }
    fn sec_header_is_tc_3_25(header: &TmPacketHeader) -> bool {
        header.service_type == SERVICE_TYPE && header.message_subtype == MES_SUBTYPE
    }
}

impl TmData for Service3_25 {
    // empty
}

/// Implementations of SpacePacket specific to PUS and TM[3,25]
///
/// # Errors
///
/// If not a valid CCSDS 133. 0-B-1 packet for TM[3,25].
/// See page 483 of ECSS-E-ST-70-41C.
impl SpacePacket<TmPacket<Service3_25>> {
    pub fn new_service_3_25(
        destination_id: u16,
        packet_name: u16,
        housekeeping_id: u8,
        parameter_value: Vec<u8>,
    ) -> Result<Self, Error> {
        SpacePacket::<TmPacket<Service3_25>>::new(
            destination_id,
            packet_name,
            housekeeping_id,
            parameter_value,
        )
    }

    /// For creating a TM[3,25] packet data structure
    /// TODO: IMPORTANT ID'S ARE NOT CHECKED
    pub fn new(
        destination_id: u16,
        packet_name: u16,
        housekeeping_id: u8,
        parameter_value: Vec<u8>,
    ) -> Result<Self, Error> {
        let data_len = Service3_25::get_packet_len(&parameter_value);
        let primary_header = PrimaryHeader::new(
            PrimaryHeader::VER_NO,
            false,
            true,
            destination_id,
            (true, true),
            packet_name,
            (data_len - PrimaryHeader::PH_LEN - 1) as u16,
        )?;
        let sec_header = TmPacketHeader::new(SERVICE_TYPE, MES_SUBTYPE, destination_id)?;
        let data = Service3_25::new(housekeeping_id, parameter_value)?;
        Ok(SpacePacket {
            primary_header,
            data: TmPacket {
                header: sec_header,
                user_data: TxUserData::<Service3_25> {
                    // TODO don't ignore
                    packet_error_control: 0,
                    data,
                },
            },
        })
    }

    /// Encodes the object to a byte vector
    pub fn to_bytes(&self) -> Vec<u8> {
        let arr_len = PrimaryHeader::PH_LEN + 1 + self.primary_header.data_len as usize;
        let mut bytes = Vec::with_capacity(arr_len);
        bytes.extend(self.primary_header.to_bytes().to_vec());
        bytes.extend(self.data.header.to_bytes().to_vec());
        bytes.extend(self.data.user_data.data.to_bytes());
        // add the two bytes then modify them to the true value.
        bytes.push(0);
        bytes.push(0);
        let pec_start = arr_len - PEC_LEN;
        BigEndian::write_u16(
            &mut bytes[pec_start..],
            self.data.user_data.packet_error_control,
        );
        bytes
    }

    pub fn from_bytes(buffer: &[u8]) -> Result<Self, Error> {
        if buffer.len() < CONST_LEN_TOT {
            return Err(Error::InvalidPacket);
        }
        let primary_header = PrimaryHeader::from_bytes(&buffer[..PrimaryHeader::PH_LEN])?;
        // If the primary header is not defined properly, give an error accordingly.
        // It has to be have sec_header_flag set, version no to 0, and for TC type_flag should be set.
        if !Service3_25::header_is_tc_3_25(&primary_header) {
            return Err(Error::InvalidPacket);
        };
        let sec_header = TmPacketHeader::from_bytes(
            &buffer[PrimaryHeader::PH_LEN..PrimaryHeader::PH_LEN + TmPacketHeader::TM_HEADER_LEN],
        )?;
        if !Service3_25::sec_header_is_tc_3_25(&sec_header) {
            return Err(Error::InvalidPacket);
        }
        let range_start = TmPacketHeader::TM_HEADER_LEN + PrimaryHeader::PH_LEN;
        let service_data: Service3_25 =
            Service3_25::from_bytes(&buffer[range_start..buffer.len() - PEC_LEN])?;

        // implement this
        let packet_error_control = BigEndian::read_u16(&buffer[(buffer.len() - PEC_LEN)..]);
        Ok(SpacePacket {
            primary_header,
            data: TmPacket::<Service3_25> {
                header: sec_header,
                user_data: TxUserData::<Service3_25> {
                    packet_error_control,
                    data: service_data,
                },
            },
        })
    }

    pub fn get_parameter_values(&self) -> Vec<u8> {
        self.data.user_data.data.parameter_value.to_vec()
    }
}
