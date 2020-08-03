
/// Module for Space Packet Protocol
pub mod packets{
    use byteorder::{ByteOrder,BigEndian};
    use std::io::{Error,ErrorKind,Read};


    /// Send packet will be represented in this struct.
    /// Storing operations will be done in Big Endian byte order.
    #[derive(Debug)]
    pub struct SpacePacket{
        primary_header:PrimaryHeader,
        data:Vec<u8>
    }

    impl SpacePacket{

        /// Create a SpacePacket struct from a given byte array
        ///
        /// # Errors
        /// 
        /// Returns InvalidData error if given byte array is not longer than 6 bytes.
        /// 
        pub fn from_bytes(packet:&[u8]) -> Result<Self,Error> {
            if !(packet.len() > 6) {
                return Err(Error::new(ErrorKind::InvalidData,"Packet has incomplete data."));
            };
            let primary_header = PrimaryHeader::from_bytes(&packet[0..6])?;
            if packet.len() - 6 != primary_header.data_len as usize + 1 {
                return Err(Error::new(ErrorKind::InvalidData,"Given byte array is conflicts as a packet."));
            }
            Ok(SpacePacket{
                primary_header,
                data:packet[6..].to_vec()
            })
        }
        
        /// Creates a packet data structure with each field given
        /// 
        /// # Errors
        /// 
        /// Gives an error if ver_no > 8 or apid > 2^11 or packet_name > 2^14 or data.len is not equal
        /// to the data_len field given.
        pub fn new(ver_no:u8, type_flag:bool, sec_header_flag:bool,
            apid:u16, seq_flags:(bool,bool), packet_name:u16, data_len:u16, data:Vec<u8>) -> Result <Self,Error>
        {       
            // check the parameters
            if ver_no > (1 << PrimaryHeader::VER_NO_BITS) ||  apid > (1 << PrimaryHeader::APID_BITS)
                || packet_name > (1 << PrimaryHeader::PACKET_NAME_BITS) || data.len() != data_len as usize + 1
            {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            Ok(SpacePacket{
                primary_header:PrimaryHeader{
                    ver_no,
                    type_flag,
                    sec_header_flag,
                    apid, seq_flags,
                    packet_name,
                    data_len
                },
                data:data
            })
        }

        /// Create a SpacePacket struct from an object that implements read.
        /// 
        /// Note: Sync issues should be handled by the caller.
        ///
        /// # Errors
        /// 
        /// Returns InvalidData when given byte array is not longer than 6 bytes
        /// 
        pub fn from_read(stream:&mut impl Read) -> Result<Self,Error>{
            let mut primary_header = [0; 6];
            stream.read(&mut primary_header)?;
            let primary_header = PrimaryHeader::from_bytes(&primary_header)?;
            if primary_header.get_data_len() < 1 {
                return Err(Error::new(ErrorKind::InvalidData,"Given data array is too short."));
            };

            let mut data:Vec<u8> = vec![0;primary_header.get_data_len()+1];
            let read_bytes = stream.read(&mut data.as_mut_slice())?;
            if read_bytes != data.len() {
                return Err(Error::new(ErrorKind::InvalidData,"Didn't get the expected number of bytes"));
            }

            Ok(SpacePacket{
                primary_header:primary_header,
                data:data[..].to_vec()
            })
        }
        

        /// Sets the packet version number of the CCSDS space packet.
        /// 
        /// # Errors
        /// 
        /// Returns an error if ver_no is bigger than 8. Because ver_no is used in its least significant 3 bits.
        pub fn set_ver_no(&mut self,ver_no:u8) -> Result<(),Error>{
            if ver_no > (1 << PrimaryHeader::VER_NO_BITS) {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            self.primary_header.ver_no = ver_no;
            Ok(())
        }

        /// Gets the packet version number of the CCSDS space packet.
        pub fn get_ver_no(&self) -> u8{
            self.primary_header.ver_no
        }

        /// Sets the packet type of the CCSDS space packet.
        pub fn set_type_flag(&mut self,type_flag:bool){
            self.primary_header.type_flag = type_flag;
        }

        /// Gets the packet type of the CCSDS space packet.
        pub fn get_type_flag(&self) -> bool{
            self.primary_header.type_flag
        }

        /// Sets the secondary header flag of the CCSDS space packet.
        pub fn set_sec_header_flag(&mut self,sec_header_flag:bool){
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
        pub fn set_apid(&mut self,apid:u16) ->  Result<(),Error>{
            if apid > (1 << PrimaryHeader::APID_BITS) {
                return Err(Error::from(ErrorKind::InvalidData));
            }
            self.primary_header.apid = apid;
            Ok(())
        }

        /// Gets the apid of the CCSDS space packet.
        pub fn get_apid(&self) -> u16{
            self.primary_header.apid
        }

        /// Sets the sequence flags of the CCSDS space packet.
        pub fn set_seq_flags(&mut self,seq_1:bool,seq_2:bool) {
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
        pub fn set_packet_name(&mut self,packet_name:u16) -> Result<(),Error> {
            if packet_name > (1 << PrimaryHeader::PACKET_NAME_BITS) {
                return Err(Error::from(ErrorKind::InvalidData));
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

    // Method for debugging
    impl std::fmt::Display for SpacePacket {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SpacePacket {{ \n")?;
            write!(f, "     {:?},\n", self.primary_header)?;
            write!(f, "     Data {:X?}\n",self.data.as_slice())?;
            write!(f, "     PH_HEX {:X?}\n",self.primary_header.to_bytes())?; // delete this
            write!(f, "}}}}")
        }
    }

    /// Big endian is the byte order used in these packages
    #[derive(Debug)]
    pub struct PrimaryHeader{
        // These fields are sorted their order below.

        ver_no:u8, // 3 bits

        // packet identification
        type_flag:bool,
        sec_header_flag:bool,
        apid:u16, // 11 bits

        // packet sequence control
        seq_flags: (bool,bool), // 2 bits
        packet_name: u16, // 14 bits
        
        data_len: u16, // 16 bits
        
    }

    /// Helper function that gets the numeric value between start and end ([start,end)) from num.
    /// 
    /// Start is the bit number starting from the MSB (as 0).
    /// 
    /// # Panics
    /// 
    /// Panics when start >= end
    fn get_bits_u32(num:u32,start:u8,end:u8) -> u32{
        if start >= end{
            panic!("get_bits_u32: invalid arguments");
        }
        let x = 32 - end;
        let mut res = num >> x;
        res = res & ((1 << (end - start)) - 1);
        res
    }


    impl PrimaryHeader{
        const VER_NO_POS:u8 = 0;
        const VER_NO_BITS:u8 = 3;
        const TYPE_FLAG_POS:u8 = 3;
        const _TYPE_FLAG_BITS:u8 = 1;
        const SEC_HEADER_FLAG_POS:u8 = 4;
        const _SEC_HEADER_FLAG_BITS:u8 = 1;
        const APID_POS:u8 = 5;
        const APID_BITS:u8 = 11;
        const SEQ_FLAGS_POS:u8 = 16;
        const _SEQ_FLAGS_BITS:u8 = 2;
        const PACKET_NAME_POS:u8 = 18;
        const PACKET_NAME_BITS:u8 = 14;
        const PACKET_DATA_LEN_POS:u8 = 32;
        const _PACKET_DATA_LEN_BITS:u8 = 16;

        const PH_LEN:u8 = 6;


        /// Creates a PrimaryHeader structure from the given 6 byte array
        /// 
        /// # Errors
        ///
        /// Sends error when `packet.len() != 6`.
        /// 
        pub fn from_bytes(packet:&[u8]) -> Result<Self,Error>{
            
            if packet.len() as u8 != PrimaryHeader::PH_LEN {
                return Err(Error::new(ErrorKind::InvalidData,"Given array should have length 6."));
            }
            
            // Read the first 4 bytes from the packet
            let packet_int = BigEndian::read_u32(&packet[0..4]);            

            let ver_no_:u8 = get_bits_u32(packet_int, PrimaryHeader::VER_NO_POS, PrimaryHeader::TYPE_FLAG_POS) as u8;
            let type_flag_:bool = 1 == get_bits_u32(packet_int ,PrimaryHeader::TYPE_FLAG_POS,PrimaryHeader::SEC_HEADER_FLAG_POS);
            let sec_header_flag_:bool = 1 == get_bits_u32(packet_int,PrimaryHeader::SEC_HEADER_FLAG_POS,PrimaryHeader::APID_POS);
            let apid_:u16 = get_bits_u32(packet_int,PrimaryHeader::APID_POS,PrimaryHeader::SEQ_FLAGS_POS) as u16;
            let seq_flags_:(bool,bool) = (
                                            get_bits_u32(packet_int,PrimaryHeader::SEQ_FLAGS_POS, PrimaryHeader::SEQ_FLAGS_POS+1) == 1,
                                            get_bits_u32(packet_int,PrimaryHeader::SEQ_FLAGS_POS+1, PrimaryHeader::PACKET_NAME_POS) == 1
                                        );
            let packet_name_:u16 = get_bits_u32(packet_int,PrimaryHeader::PACKET_NAME_POS,PrimaryHeader::PACKET_DATA_LEN_POS) as u16;
            let data_len_: u16 = BigEndian::read_u16(&packet[4..6]);

            Ok(PrimaryHeader{
                ver_no: ver_no_,
                type_flag: type_flag_,
                sec_header_flag: sec_header_flag_,
                apid: apid_,
                seq_flags: seq_flags_,
                packet_name: packet_name_,
                data_len: data_len_
            })
        }
        /// Returns the primary header as a fixed size 6 byte u8 array
        pub fn to_bytes(&self) -> [u8;6]{
            let mut res:[u8;6] = [0;6];
            
            // first two bytes of packet
            let mut packet_part:u16 = self.ver_no as u16;
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
            BigEndian::write_u16(&mut res[0..2],packet_part);
            // since this field ends with the LSB it is directly assigned
            packet_part = self.packet_name;
            
            if self.seq_flags.0 {
                packet_part += 1 << (32 - (PrimaryHeader::SEQ_FLAGS_POS + 1))
            }
            if self.seq_flags.1 {
                packet_part += 1 << (32 - (PrimaryHeader::PACKET_NAME_POS))
            }

            BigEndian::write_u16(&mut res[2..4],packet_part);

            // writing the data length is trivial
            BigEndian::write_u16(&mut res[4..6],self.data_len);
            res
        }

        pub fn get_data_len(&self) -> usize {
            self.data_len as usize
        }
    }
    /// Test module
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_bits_u32_test() {
            assert_eq!(0,get_bits_u32(12,0,1));
            assert_eq!(1,get_bits_u32(1,31,32));
            assert_eq!(11,get_bits_u32(22,27,31));
        }

        #[test]
        #[should_panic]
        fn get_bits_u32_test_pan(){
            assert_eq!(3,get_bits_u32(1,31,31));
        }

        #[test]
        fn new_test() {
            let ph_bytes = vec![0x18,0x07,0xc0,0x22,0,0x1a];

            let sp_data:Vec<u8> = vec![0, 0, 0x2A, 0, 0x1, 0, 0x1, 0x1, 0, 0x1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0x22, 0xC0, 0x1, 0x1, 0x1, 0xD, 0xA7, 0xC];
            
            let sp = SpacePacket::new(0,true,true,7,(true,true),34,26,sp_data).unwrap();
            
            for (i,byte) in sp.primary_header.to_bytes().iter().enumerate(){
                assert_eq!(ph_bytes[i],*byte);
            };
            
            let _sp = SpacePacket::new(9,true,true,7,(true,true),34,26,vec![0;27]).expect_err("Didn't give an error.");
            let _sp = SpacePacket::new(0,true,true,2049,(true,true),34,26,vec![0;27]).expect_err("Didn't give an error.");
            let _sp = SpacePacket::new(0,true,true,7,(true,true),1 << 14 + 1,26,vec![0;27]).expect_err("Didn't give an error.");
            let _sp = SpacePacket::new(0,true,true,7,(true,true),34,27,vec![0;27]).expect_err("Didn't give an error.");
            let _sp = SpacePacket::new(0,true,true,7,(true,true),34,26,vec![0;26]).expect_err("Didn't give an error.");
        }

        #[test]
        fn from_bytes_test(){
            let mut arg1 = vec![0x18,0x07,0xc0,0x22,0,0x1a];
            let mut data1 = vec![0;27];
            arg1.append(&mut data1);
            let mut arg2 = vec![0x18,0x07,0xc0,0x22,0,0x1a];
            let mut data2 = vec![0;28];
            arg2.append(&mut data2);

            SpacePacket::from_bytes(&arg1).unwrap();
            SpacePacket::from_bytes(&arg2).expect_err("Didn't give error");
        }

        #[test]
        fn getter_setters_test(){
            let mut sp = SpacePacket::new(0,false,false,0,(false,false),0,0,vec![0]).unwrap();
            
            sp.set_apid(1).unwrap();
            assert_eq!(sp.get_apid(),1);
            sp.set_apid(2050).expect_err("Didn't get an err");

            sp.set_packet_name(12).unwrap();
            assert_eq!(sp.get_packet_name(),12);
            sp.set_packet_name(1 << 15).expect_err("Didn't get an err");

            sp.set_seq_flags(true,false);
            assert_eq!(sp.get_seq_flags(),(true,false));
            
            sp.set_sec_header_flag(true);
            assert_eq!(sp.get_sec_header_flag(),true);

            sp.set_ver_no(9).expect_err("Didn't get an error");
            sp.set_ver_no(1).unwrap();
            assert_eq!(1,sp.get_ver_no());

            assert_eq!(0,sp.get_data_len());
        }
    }
}

