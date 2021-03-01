//! TC[3,5] enable the periodic generation of housekeeping parameter reports
use super::*;
use service_3::service_3_5x6::*;
use tc::TcPacket;

const MES_SUBTYPE: u8 = 5;

/// Implementations of SpacePacket specific to PUS and TC[3,5]
///
/// # Errors
///
/// If not a valid CCSDS 133. 0-B-1 packet for TC[3,5].
/// See page 483 of ECSS-E-ST-70-41C.
impl SpacePacket<TcPacket<Service3_5x6>> {
    /// Wrapper for creating a TC[3,5] packet
    pub fn new_service_3_5(
        apid: u16,
        packet_name: u16,
        n: u8,
        report_ids: Vec<u8>,
    ) -> Result<Self, Error> {
        SpacePacket::<TcPacket<Service3_5x6>>::new_service_3_5x6(
            MES_SUBTYPE,
            apid,
            packet_name,
            n,
            report_ids,
        )
    }

    /// Wrapper for converting a TC[3,5] packet from bytes
    pub fn from_bytes_service_3_5(buffer: &[u8]) -> Result<Self, Error> {
        let res = SpacePacket::from_bytes_service_3_5x6(&buffer)?;
        if res.data.header.message_subtype != MES_SUBTYPE {
            return Err(Error::CorruptData);
        }
        Ok(res)
    }
}
