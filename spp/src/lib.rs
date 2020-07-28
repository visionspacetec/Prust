

pub mod packets{
    use byteorder::{ByteOrder,BigEndian};
    use std::io::{Error,ErrorKind,Read};

    #[derive(Debug)]
    pub struct SpacePacket{
        primary_header:PrimaryHeader,
        data:Vec<u8>
    }

    impl SpacePacket{
        pub fn from_bytes(packet:&[u8]) -> Self {
            if !(packet.len() > 6) {
                panic!("Packet has incomplete data.");
            };
            SpacePacket{
                primary_header:PrimaryHeader::from_bytes(&packet[0..6]),
                data:packet[6..].to_vec()
            }
        }
        
        pub fn from_read(stream:&mut impl Read) -> Result<Self,Error>{
            let mut primary_header = [0; 6];
            stream.read(&mut primary_header)?;
            let primary_header = PrimaryHeader::from_bytes(&primary_header);
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
         
        pub fn set_ver_no(&mut self,ver_no:u8){
            self.primary_header.ver_no = ver_no;
        }

        pub fn get_ver_no(&self) -> u8{
            self.primary_header.ver_no
        }

        pub fn set_type_flag(&mut self,type_flag:bool){
            self.primary_header.type_flag = type_flag;
        }

        pub fn get_type_flag(&self) -> bool{
            self.primary_header.type_flag
        }

        pub fn set_sec_header_flag(&mut self,sec_header_flag:bool){
            self.primary_header.sec_header_flag = sec_header_flag;
        }

        pub fn get_sec_header_flag(&self) -> bool {
            self.primary_header.sec_header_flag
        }

        pub fn set_apid(&mut self,apid:u16){
            self.primary_header.apid = apid;
        }

        pub fn get_apid(&self) -> u16{
            self.primary_header.apid
        }

        pub fn set_seq_flags(&mut self,seq_1:bool,seq_2:bool) {
            self.primary_header.seq_flags.0 = seq_1;
            self.primary_header.seq_flags.1 = seq_2;
        }

        pub fn get_seq_flags(&self) -> (bool, bool) {
            self.primary_header.seq_flags
        }

        pub fn get_packet_name(&self) -> u16 {
            self.primary_header.packet_name
        }

        pub fn set_packet_name(&mut self,packet_name:u16) {
            self.primary_header.packet_name = packet_name;
        }

        pub fn get_data_len(&self) -> u16 {
            self.primary_header.data_len
        }
    }

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

    // TODO check parameters
    fn get_bits_u32(num:u32,start:u8,end:u8) -> u32{
        let x = 32 - end;
        let mut res = num >> x;
        res = res & ((1 << (end - start)) - 1);
        res
    }


    impl PrimaryHeader{
        const VER_NO_POS:u8 = 0;
        const TYPE_FLAG_POS:u8 = 3;
        const SEC_HEADER_FLAG_POS:u8 = 4;
        const APID_POS:u8 = 5;
        const SEQ_FLAGS_POS:u8 = 16;
        const PACKET_NAME_POS:u8 = 18;
        const PACKET_DATA_LEN_POS:u8 = 32;

        const PH_LEN:u8 = 6;

        /// Creates a PrimaryHeader structure from the given 6 byte array
        /// 
        /// # Panics
        ///
        /// Panics when `packet.len() != 6`.
        /// 
        pub fn from_bytes(packet:&[u8]) -> Self{
            // TODO: Return result
            if packet.len() as u8 != PrimaryHeader::PH_LEN {
                panic!("PrimaryHeader::new: given array should have length 6.");
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

            PrimaryHeader{
                ver_no: ver_no_,
                type_flag: type_flag_,
                sec_header_flag: sec_header_flag_,
                apid: apid_,
                seq_flags: seq_flags_,
                packet_name: packet_name_,
                data_len: data_len_
            }
        }
        // Returns a fixed size 6 byte u8 array
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
    
}