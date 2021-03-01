//! Module that contains PUS TC packet implementations of SpacePacket struct.
//! Packets defined here are compliant to ECSS-E-ST-70-41C.

use crate::error::*;
use crate::sp::{SpacePacketDataField, TxUserData}; // Including Generic Packet
use byteorder::{BigEndian, ByteOrder}; // For writing the numbers to byte arrays

extern crate alloc; // link the allocator
/// Header of the TcPackets, secondary header of a SpacePacket.
#[derive(Debug)]
pub struct TcPacketHeader {
    /// Only 4 least significant bits used. When creating always set to 2.
    pub(crate) pus_ver_no: u8,
    /// TODO don't ignore
    pub(crate) acknowledgement_flags: (bool, bool, bool, bool),
    /// Service type of the TC pack.
    pub(crate) service_type: u8,
    /// Message Subtype of the TC pack.
    pub(crate) message_subtype: u8,
    /// TODO don't ignore
    pub(crate) source_id: u16,
}
/// implementation of TcPacketHeader. While creating the TcPacketHeader PUS standard are checked according to TC rules generally.
impl TcPacketHeader {
    pub(crate) const PUS_VER_NO: u8 = 2;
    pub(crate) const TC_HEADER_LEN: usize = 5;

    /// Method to create a TcPacketHeader with specified parameters.
    pub fn new(
        acknowledgement_flags: (bool, bool, bool, bool),
        service_type: u8,
        message_subtype: u8,
        source_id: u16,
    ) -> Result<Self, Error> {
        // TODO : Do checks for source_id and acknowledgement flags.
        Ok(TcPacketHeader {
            pus_ver_no: TcPacketHeader::PUS_VER_NO,
            acknowledgement_flags,
            service_type,
            message_subtype,
            source_id,
        })
    }
    /// Creates TcPacketHeader structure a byte array
    ///
    /// # Errors
    ///
    /// Returns error when `packet.len() != 5`.
    ///
    pub fn from_bytes(buffer: &[u8]) -> Result<Self, Error> {
        // the length of a primary header is constant so it will return an error if it is not 5
        if buffer.len() != TcPacketHeader::TC_HEADER_LEN {
            return Err(Error::InvalidPacket);
        }
        let ver_no_and_flags = buffer[0];
        let service_type = buffer[1];
        let message_subtype = buffer[2];
        let source_id = BigEndian::read_u16(&buffer[3..5]);
        // TODO: check source_id, flags and ver_no
        // TODO: use constants
        let ver_no = ver_no_and_flags >> 4;
        let flags = (
            ver_no_and_flags & 0b0000_0001 != 0,
            ver_no_and_flags & 0b0000_0010 != 0,
            ver_no_and_flags & 0b0000_0100 != 0,
            ver_no_and_flags & 0b0000_1000 != 0,
        );

        Ok(TcPacketHeader {
            pus_ver_no: ver_no,
            acknowledgement_flags: flags,
            service_type,
            message_subtype,
            source_id,
        })
    }
    /// Encodes the header to a fixed size byte array
    pub fn to_bytes(&self) -> [u8; TcPacketHeader::TC_HEADER_LEN] {
        let mut bytes: [u8; TcPacketHeader::TC_HEADER_LEN] = [0; TcPacketHeader::TC_HEADER_LEN];
        let mut ver_no_and_flags = self.pus_ver_no << 4;
        if self.acknowledgement_flags.0 {
            ver_no_and_flags += 0b0000_0001
        };
        if self.acknowledgement_flags.1 {
            ver_no_and_flags += 0b0000_0010
        };
        if self.acknowledgement_flags.2 {
            ver_no_and_flags += 0b0000_0100
        };
        if self.acknowledgement_flags.3 {
            ver_no_and_flags += 0b0000_1000
        };
        bytes[0] = ver_no_and_flags;
        bytes[1] = self.service_type;
        bytes[2] = self.message_subtype;
        BigEndian::write_u16(&mut bytes[3..5], self.source_id);

        bytes
    }
}
pub trait TcData {
    /* intentionally empty*/
}
/// Generic Telecommand packet part.
/// This part represents packet data field of the CCSDS 133. 0-B-1 packet.
#[derive(Debug)]
pub struct TcPacket<T: TcData> {
    /// Secondary Header of CCSDS packet.
    pub(crate) header: TcPacketHeader,
    pub(crate) user_data: TxUserData<T>,
}

impl<T: TcData> SpacePacketDataField for TcPacket<T> {
    /* intentionally empty*/
}
