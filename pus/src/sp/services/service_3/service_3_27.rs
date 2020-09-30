//!TC[3,27] create a housekeeping parameter report structure
use super::*;
use tc::{TcData, TcPacket, TcPacketHeader};

pub struct Service3_27 {
    pub(crate) n: u8,
    pub(crate) report_ids: Vec<u8>,
}

const CONST_LEN_TOT: usize = 1;
const MES_SUBTYPE: u8 = 27;

impl Service3_27 {
    /// For creating a TC[3,27] data structure
    /// TODO: IMPORTANT ID'S ARE NOT CHECKED
    pub fn new(n: u8, report_ids: Vec<u8>) -> Result<Self, Error> {
        if report_ids.len() != n as usize {
            return Err(Error::InvalidPacket);
        }
        Ok(Service3_27 { n, report_ids })
    }
    /// For creating a TC[3,27] data structure. Wrapper of "new" function
    pub fn new_service_3_27(n: u8, report_ids: Vec<u8>) -> Result<Self, Error> {
        Service3_27::new(n, report_ids)
    }

    pub(crate) fn from_bytes(buffer: &[u8]) -> Result<Self, Error> {
        if buffer.len() < CONST_LEN_TOT || buffer.len() != CONST_LEN_TOT + buffer[0] as usize {
            return Err(Error::InvalidPacket);
        }
        let n = buffer[0];
        let report_ids = buffer[1..].to_vec();
        Ok(Service3_27 { n, report_ids })
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(CONST_LEN_TOT + self.n as usize);
        bytes.push(self.n);
        bytes.extend(self.report_ids.to_vec());
        bytes
    }
    fn get_packet_len(n: u8) -> usize {
        CONST_LEN_TOT + n as usize + TcPacketHeader::TC_HEADER_LEN + PrimaryHeader::PH_LEN + PEC_LEN
    }
    fn header_is_tc_3_27(header: &PrimaryHeader) -> bool {
        header.sec_header_flag && header.ver_no == PrimaryHeader::VER_NO && header.type_flag
    }
    fn sec_header_is_tc_3_27(header: &TcPacketHeader) -> bool {
        header.service_type == SERVICE_TYPE && header.message_subtype == MES_SUBTYPE
    }
}

impl TcData for Service3_27 {
    // empty
}

/// Implementations of SpacePacket specific to PUS and TC[3,27]
///
/// # Errors
///
/// If not a valid CCSDS 133. 0-B-1 packet for TC[3,27].
/// See page 483 of ECSS-E-ST-70-41C.
impl SpacePacket<TcPacket<Service3_27>> {
    pub fn new_service_3_27(
        apid: u16,
        packet_name: u16,
        n: u8,
        report_ids: Vec<u8>,
    ) -> Result<Self, Error> {
        SpacePacket::<TcPacket<Service3_27>>::new(apid, packet_name, n, report_ids)
    }

    /// For creating a TC[3,27] packet data structure
    /// TODO: IMPORTANT ID'S ARE NOT CHECKED
    pub fn new(apid: u16, packet_name: u16, n: u8, report_ids: Vec<u8>) -> Result<Self, Error> {
        if report_ids.len() != n as usize {
            return Err(Error::InvalidPacket);
        }
        let data_len = Service3_27::get_packet_len(n);
        let primary_header = PrimaryHeader::new(
            PrimaryHeader::VER_NO,
            true,
            true,
            apid,
            (true, true),
            packet_name,
            (data_len - PrimaryHeader::PH_LEN - 1) as u16,
        )?;
        // TODO UPDATE FLAG AVAILIBILITY
        let sec_header =
            TcPacketHeader::new((false, false, false, false), SERVICE_TYPE, MES_SUBTYPE, 0)?;
        let data = Service3_27::new(n, report_ids)?;
        Ok(SpacePacket {
            primary_header,
            data: TcPacket {
                header: sec_header,
                user_data: TxUserData::<Service3_27> {
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
        if !Service3_27::header_is_tc_3_27(&primary_header) {
            return Err(Error::InvalidPacket);
        };
        let sec_header = TcPacketHeader::from_bytes(
            &buffer[PrimaryHeader::PH_LEN..PrimaryHeader::PH_LEN + TcPacketHeader::TC_HEADER_LEN],
        )?;
        if !Service3_27::sec_header_is_tc_3_27(&sec_header) {
            return Err(Error::InvalidPacket);
        }
        let range_start = TcPacketHeader::TC_HEADER_LEN + PrimaryHeader::PH_LEN;
        let service_data: Service3_27 =
            Service3_27::from_bytes(&buffer[range_start..buffer.len() - PEC_LEN])?;

        // implement this
        let packet_error_control = BigEndian::read_u16(&buffer[(buffer.len() - PEC_LEN)..]);
        Ok(SpacePacket {
            primary_header,
            data: TcPacket::<Service3_27> {
                header: sec_header,
                user_data: TxUserData::<Service3_27> {
                    packet_error_control,
                    data: service_data,
                },
            },
        })
    }

    pub fn get_hk_report_struct_ids(&self) -> &Vec<u8> {
        &self.data.user_data.data.report_ids
    }
}
