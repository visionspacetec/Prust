//! Module that contains PUS TM packet implementations of SpacePacket struct.
//! Packets defined here are compliant to ECSS-E-ST-70-41C.
use super::*;
use crate::sp::{SpacePacketDataField, TxUserData};
use byteorder::{BigEndian, ByteOrder}; // For writing the numbers to byte arrays // Including Generic Packet
extern crate alloc; // link the allocator

/// Header of the TmPackets, secondary header of a SpacePacket.
#[derive(Debug)]
pub struct TmPacketHeader {
    /// Only 4 least significant bits are used. When creating always set to 2.
    pub(crate) pus_ver_no: u8,
    /// Only 4 least significant bits are used.
    /// TODO don't ignore
    pub(crate) time_ref_status: u8,
    /// Service type of the TM pack.
    pub(crate) service_type: u8,
    /// Message Subtype of the TM pack.
    pub(crate) message_subtype: u8,
    /// If not capable of counting set to 0.
    pub(crate) message_type_counter: u16,
    /// TODO don't ignore
    pub(crate) destination_id: u16,
    /// TODO decide on time
    /// TODO change val type
    pub(crate) abs_time: u16,
}
/// implementation of TmPacketHeader. While creating the TmPacketHeader PUS standard are checked according to TM rules generally.
impl TmPacketHeader {
    pub(crate) const PUS_VER_NO: u8 = 2;
    pub(crate) const TM_HEADER_LEN: usize = 9;
    pub(crate) const ABS_TIME_LEN: usize = 2;

    /// Method to create a TmPacketHeader with specified parameters.
    pub fn new(service_type: u8, message_subtype: u8, destination_id: u16) -> Result<Self, Error> {
        // TODO : Do checks for destination_id and acknowledgement flags.
        Ok(TmPacketHeader {
            pus_ver_no: TmPacketHeader::PUS_VER_NO,
            // TODO fill
            time_ref_status: 0,
            service_type,
            message_subtype,
            message_type_counter: 0,
            destination_id,
            // TODO fill
            abs_time: 0,
        })
    }
    /// Creates TmPacketHeader structure a byte array
    ///
    /// # Errors
    ///
    /// Returns error when `packet.len() != TM_HEADER_LEN`.
    ///
    pub fn from_bytes(buffer: &[u8]) -> Result<Self, Error> {
        // the length of a primary header is constant so it will return an error if it is not TM_HEADER_LEN
        if buffer.len() != TmPacketHeader::TM_HEADER_LEN {
            return Err(Error::InvalidPacket);
        }
        let ver_no_and_status = buffer[0];
        let service_type = buffer[1];
        let message_subtype = buffer[2];
        let message_type_counter = BigEndian::read_u16(&buffer[3..5]);
        let destination_id = BigEndian::read_u16(&buffer[5..7]);
        // TODO: chech the size
        let abs_time = BigEndian::read_u16(&buffer[7..7 + TmPacketHeader::ABS_TIME_LEN]);
        // TODO: check dest_id_id, flags and ver_no
        // TODO: use constants
        let ver_no = ver_no_and_status >> 4;
        let status = ver_no_and_status & 0b0000_1111;

        Ok(TmPacketHeader {
            pus_ver_no: ver_no,
            time_ref_status: status,
            service_type,
            message_subtype,
            message_type_counter,
            destination_id,
            abs_time,
        })
    }
    /// Encodes the header to a fixed size byte array
    pub fn to_bytes(&self) -> [u8; TmPacketHeader::TM_HEADER_LEN] {
        let mut bytes: [u8; TmPacketHeader::TM_HEADER_LEN] = [0; TmPacketHeader::TM_HEADER_LEN];
        let mut ver_no_and_status = self.pus_ver_no << 4;
        ver_no_and_status += self.time_ref_status;
        bytes[0] = ver_no_and_status;
        bytes[1] = self.service_type;
        bytes[2] = self.message_subtype;
        BigEndian::write_u16(&mut bytes[3..5], self.message_type_counter);
        BigEndian::write_u16(&mut bytes[5..7], self.destination_id);
        BigEndian::write_u16(&mut bytes[7..9], self.abs_time);

        bytes
    }
}
pub trait TmData {
    /* intentionally empty*/
}
/// Generic Telecommand packet part.
/// This part represents packet data field of the CCSDS 133. 0-B-1 packet.
#[derive(Debug)]
pub struct TmPacket<T: TmData + core::fmt::Debug> {
    /// Secondary Header of CCSDS packet.
    pub(crate) header: TmPacketHeader,
    pub(crate) user_data: TxUserData<T>,
}

impl<T: TmData + core::fmt::Debug> SpacePacketDataField for TmPacket<T> {
    /* intentionally empty*/
}
