use super::*;
use byteorder::{BigEndian, ByteOrder}; // For writing the numbers to byte arrays
use crate::sp::services::service_1::RequestId;
use alloc::vec::Vec; // Return struct of the trait Request

pub trait SpacePacketDataField {}

/// Send packet will be represented in this struct.
/// Storing operations will be done in Big Endian byte order.
#[derive(Debug)]
pub struct SpacePacket<T: SpacePacketDataField> {
    primary_header: PrimaryHeader,
    data: T,
}

// SpacePacketDataField is a byte vector in general case.
impl SpacePacketDataField for Vec<u8> {
    /* intentionally empty*/
}

/// Generic SpacePacket method implementations
impl<T: SpacePacketDataField> SpacePacket<T> {
    /// Sets the packet version number of the CCSDS space packet.
    ///
    /// # Errors
    ///
    /// Returns an error if ver_no is bigger than 8. Because ver_no is used in its least significant 3 bits.
    pub fn set_ver_no(&mut self, ver_no: u8) -> Result<(), Error> {
        if ver_no > (1 << PrimaryHeader::VER_NO_BITS) {
            return Err(Error::InvalidVersionNo);
        }
        self.primary_header.ver_no = ver_no;
        Ok(())
    }

    /// Gets the packet version number of the CCSDS space packet.
    pub fn get_ver_no(&self) -> u8 {
        self.primary_header.ver_no
    }

    /// Sets the packet type of the CCSDS space packet.
    pub fn set_type_flag(&mut self, type_flag: bool) {
        self.primary_header.type_flag = type_flag;
    }

    /// Gets the packet type of the CCSDS space packet.
    pub fn get_type_flag(&self) -> bool {
        self.primary_header.type_flag
    }

    /// Sets the secondary header flag of the CCSDS space packet.
    pub fn set_sec_header_flag(&mut self, sec_header_flag: bool) {
        self.primary_header.sec_header_flag = sec_header_flag;
    }

    /// Gets the secondary header flag of the CCSDS space packet.
    pub fn get_sec_header_flag(&self) -> bool {
        self.primary_header.sec_header_flag
    }

    /// Sets the packet version number of the CCSDS space packet.
    ///
    /// # Errors
    ///
    /// Returns an error if apid is bigger than 2^11 = 2048. Because apid is used in its least significant 11 bits.
    pub fn set_apid(&mut self, apid: u16) -> Result<(), Error> {
        if apid > (1 << PrimaryHeader::APID_BITS) {
            return Err(Error::InvalidApid);
        }
        self.primary_header.apid = apid;
        Ok(())
    }

    /// Gets the apid of the CCSDS space packet.
    pub fn get_apid(&self) -> u16 {
        self.primary_header.apid
    }

    /// Sets the sequence flags of the CCSDS space packet.
    pub fn set_seq_flags(&mut self, seq_1: bool, seq_2: bool) {
        self.primary_header.seq_flags.0 = seq_1;
        self.primary_header.seq_flags.1 = seq_2;
    }

    /// Gets the sequence flags of the CCSDS space packet.
    pub fn get_seq_flags(&self) -> (bool, bool) {
        self.primary_header.seq_flags
    }

    /// Sets the packet sequence count or packet name of the CCSDS space packet.
    ///
    /// # Errors
    ///
    /// Returns an error if packet_name is bigger than 2^14 = 16384. Because packet_name is used in its least significant 14 bits.
    pub fn set_packet_name(&mut self, packet_name: u16) -> Result<(), Error> {
        if packet_name > (1 << PrimaryHeader::PACKET_NAME_BITS) {
            return Err(Error::InvalidPacketName);
        }
        self.primary_header.packet_name = packet_name;
        Ok(())
    }

    /// Gets the packet sequence count or packet name of the CCSDS space packet.
    pub fn get_packet_name(&self) -> u16 {
        self.primary_header.packet_name
    }

    /// Gets the packet data length of the CCSDS space packet.
    pub fn get_data_len(&self) -> u16 {
        self.primary_header.data_len
    }
}

impl SpacePacket<Vec<u8>> {
    /// Create a SpacePacket struct from a given byte array
    ///
    /// # Errors
    ///
    /// Returns InvalidData error if given byte array is not longer than 6 bytes.
    ///
    pub fn from_bytes(packet: &[u8]) -> Result<Self, Error> {
        // a packet should be least 7 bytes
        if !(packet.len() > 6) {
            return Err(Error::InvalidPacket);
        };
        let primary_header = PrimaryHeader::from_bytes(&packet[0..6])?;
        // data packet length should be 1 + data_len field
        if packet.len() - 6 != primary_header.data_len as usize + 1 {
            return Err(Error::InvalidPacket);
        }
        let data: Vec<u8> = Vec::from(&packet[6..]);
        Ok(SpacePacket {
            primary_header,
            data,
        })
    }

    /// Creates a packet data structure with each field given
    ///
    /// # Errors
    ///
    /// Gives an error if ver_no > 8 or apid > 2^11 or packet_name > 2^14 or if data.len() == 0
    pub fn new(
        ver_no: u8,
        type_flag: bool,
        sec_header_flag: bool,
        apid: u16,
        seq_flags: (bool, bool),
        packet_name: u16,
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        // check the parameters
        if ver_no > (1 << PrimaryHeader::VER_NO_BITS)
            || apid > (1 << PrimaryHeader::APID_BITS)
            || packet_name > (1 << PrimaryHeader::PACKET_NAME_BITS)
            || data.len() == 0
        {
            return Err(Error::InvalidPacket);
        }
        Ok(SpacePacket {
            primary_header: PrimaryHeader::new(
                ver_no,
                type_flag,
                sec_header_flag,
                apid,
                seq_flags,
                packet_name,
                data.len() as u16 - 1,
            )?,
            data,
        })
    }
}

// Method for debugging
impl core::fmt::Display for SpacePacket<Vec<u8>> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "SpacePacket {{ \n")?;
        write!(f, "     {:?},\n", self.primary_header)?;
        write!(f, "     Data {:X?}\n", self.data)?;
        write!(f, "     PH_HEX {:X?}\n", self.primary_header.to_bytes())?; // delete this
        write!(f, "}}}}")
    }
}

/// Big endian is the byte order used in these packages
#[derive(Debug)]
pub struct PrimaryHeader {
    // These fields are sorted their order below.
    ver_no: u8, // 3 bits

    // packet identification
    type_flag: bool,
    sec_header_flag: bool,
    apid: u16, // 11 bits

    // packet sequence control
    seq_flags: (bool, bool), // 2 bits
    packet_name: u16,        // 14 bits

    data_len: u16, // 16 bits
}

/// Helper function that gets the numeric value between start and end ([start,end)) from num.
///
/// Start is the bit number starting from the MSB (as 0).
///
/// # Panics
///
/// Panics when start >= end
fn get_bits_u32(num: u32, start: u8, end: u8) -> u32 {
    if start >= end {
        panic!("get_bits_u32: invalid arguments");
    }
    let x = 32 - end;
    let mut res = num >> x;
    res = res & ((1 << (end - start)) - 1);
    res
}

impl PrimaryHeader {
    /* Places and the length of the fields in the primary header */
    const VER_NO_POS: u8 = 0;
    const VER_NO_BITS: u8 = 3;
    const TYPE_FLAG_POS: u8 = 3;
    const _TYPE_FLAG_BITS: u8 = 1;
    const SEC_HEADER_FLAG_POS: u8 = 4;
    const _SEC_HEADER_FLAG_BITS: u8 = 1;
    const APID_POS: u8 = 5;
    const APID_BITS: u8 = 11;
    const SEQ_FLAGS_POS: u8 = 16;
    const _SEQ_FLAGS_BITS: u8 = 2;
    const PACKET_NAME_POS: u8 = 18;
    const PACKET_NAME_BITS: u8 = 14;
    const PACKET_DATA_LEN_POS: u8 = 32;
    const _PACKET_DATA_LEN_BITS: u8 = 16;
    /* primary header length */
    pub const PH_LEN: usize = 6;
    /* default ver_no value for CCSDS 133. 0-B-1 packet*/
    const VER_NO: u8 = 0;

    /// Creates a packet header data structure with each field given
    ///
    /// # Errors
    ///
    /// Gives an error if ver_no > 8 or apid > 2^11 or packet_name > 2^14 or data.len is not equal
    /// to the data_len field given.
    pub fn new(
        ver_no: u8,
        type_flag: bool,
        sec_header_flag: bool,
        apid: u16,
        seq_flags: (bool, bool),
        packet_name: u16,
        data_len: u16,
    ) -> Result<Self, Error> {
        // check the parameters
        if ver_no > (1 << PrimaryHeader::VER_NO_BITS)
            || apid > (1 << PrimaryHeader::APID_BITS)
            || packet_name > (1 << PrimaryHeader::PACKET_NAME_BITS)
        {
            return Err(Error::InvalidPacket);
        }
        Ok(PrimaryHeader {
            ver_no,
            type_flag,
            sec_header_flag,
            apid,
            seq_flags,
            packet_name,
            data_len,
        })
    }

    /// Creates a PrimaryHeader structure from the given 6 byte array
    ///
    /// # Errors
    ///
    /// Sends error when `packet.len() != 6`.
    ///
    pub fn from_bytes(packet: &[u8]) -> Result<Self, Error> {
        // the length of a primary header is constant so it will return an error if it is not 6
        if packet.len() != PrimaryHeader::PH_LEN {
            return Err(Error::InvalidPacket);
        }
        // Read the first 4 bytes from the packet as an u32 integer
        let packet_int = BigEndian::read_u32(&packet[0..4]);
        // parsing the u32 packet_int it got by the help of the get_bits_u32 method
        // indicate their position in the bit array and store them
        let ver_no_: u8 = get_bits_u32(
            packet_int,
            PrimaryHeader::VER_NO_POS,
            PrimaryHeader::TYPE_FLAG_POS,
        ) as u8;
        let type_flag_: bool = 1
            == get_bits_u32(
                packet_int,
                PrimaryHeader::TYPE_FLAG_POS,
                PrimaryHeader::SEC_HEADER_FLAG_POS,
            );
        let sec_header_flag_: bool = 1
            == get_bits_u32(
                packet_int,
                PrimaryHeader::SEC_HEADER_FLAG_POS,
                PrimaryHeader::APID_POS,
            );
        let apid_: u16 = get_bits_u32(
            packet_int,
            PrimaryHeader::APID_POS,
            PrimaryHeader::SEQ_FLAGS_POS,
        ) as u16;
        let seq_flags_: (bool, bool) = (
            get_bits_u32(
                packet_int,
                PrimaryHeader::SEQ_FLAGS_POS,
                PrimaryHeader::SEQ_FLAGS_POS + 1,
            ) == 1,
            get_bits_u32(
                packet_int,
                PrimaryHeader::SEQ_FLAGS_POS + 1,
                PrimaryHeader::PACKET_NAME_POS,
            ) == 1,
        );
        let packet_name_: u16 = get_bits_u32(
            packet_int,
            PrimaryHeader::PACKET_NAME_POS,
            PrimaryHeader::PACKET_DATA_LEN_POS,
        ) as u16;
        // read the data len directly as bytes since it is 2 bytes
        let data_len_: u16 = BigEndian::read_u16(&packet[4..6]);
        // return the created struct
        if ver_no_ != 0 {
            return Err(Error::InvalidVersionNo);
        }
        Ok(PrimaryHeader {
            ver_no: ver_no_,
            type_flag: type_flag_,
            sec_header_flag: sec_header_flag_,
            apid: apid_,
            seq_flags: seq_flags_,
            packet_name: packet_name_,
            data_len: data_len_,
        })
    }
    /// Returns the primary header as a fixed size 6 byte u8 array
    pub fn to_bytes(&self) -> [u8; PrimaryHeader::PH_LEN] {
        let mut res: [u8; PrimaryHeader::PH_LEN] = [0; PrimaryHeader::PH_LEN];

        // first two bytes of packet
        let mut packet_part: u16 = self.ver_no as u16;
        packet_part = packet_part << (16 - PrimaryHeader::TYPE_FLAG_POS);
        // setting the type_flag fields
        if self.type_flag {
            packet_part += 1 << (16 - PrimaryHeader::SEC_HEADER_FLAG_POS);
        }
        if self.sec_header_flag {
            packet_part += 1 << (16 - PrimaryHeader::APID_POS);
        }
        // adding the apid
        packet_part += self.apid;
        // writing the calculated 2 bytes
        BigEndian::write_u16(&mut res[0..2], packet_part);
        // since this field ends with the LSB it is directly assigned
        packet_part = self.packet_name;

        if self.seq_flags.0 {
            packet_part += 1 << (32 - (PrimaryHeader::SEQ_FLAGS_POS + 1))
        }
        if self.seq_flags.1 {
            packet_part += 1 << (32 - (PrimaryHeader::PACKET_NAME_POS))
        }

        BigEndian::write_u16(&mut res[2..4], packet_part);

        // writing the data length is trivial
        BigEndian::write_u16(&mut res[4..6], self.data_len);
        res
    }
    // getter for the data_len field of the PrimaryHeader
    pub fn get_data_len(&self) -> usize {
        self.data_len as usize
    }
}
/// Test module for SpacePacket struct and its inner struct PrimaryHeader.
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct TxUserData<T> {
    packet_error_control: u16,
    /// Application (TC) or Source (TM) data field.
    data: T,
}
/// Trait Indicating The Packet is a PUS request type.
pub trait Request {
    fn to_request(&self) -> RequestId;
}

/// Trait implementation that will create RequestId from generic SpacePacket
impl<T: SpacePacketDataField> Request for SpacePacket<T> {
    fn to_request(&self) -> RequestId {
        RequestId {
            ver_no: self.primary_header.ver_no,
            packet_type: self.primary_header.type_flag,
            sec_header_flag: self.primary_header.sec_header_flag,
            apid: self.primary_header.apid,
            seq_flags: self.primary_header.seq_flags,
            packet_seq_count: self.primary_header.packet_name,
        }
    }
}

pub fn get_service_type(buf: &[u8]) -> Result<(u8, u8), ()> {
    // unsupported
    if buf.len() <= PrimaryHeader::PH_LEN + sp::tc::TcPacketHeader::TC_HEADER_LEN {
        Err(())
    } else {
        Ok((
            buf[PrimaryHeader::PH_LEN + 1],
            buf[PrimaryHeader::PH_LEN + 2],
        ))
    }
}

/// packet_error_control len
const PEC_LEN: usize = 2;
pub mod services;

/// Module for Telecommand packet compliant to PUS.
pub mod tc;
/// Module for Telemetry packet compliant to PUS.
pub mod tm;
