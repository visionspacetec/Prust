

pub mod headers{
    use byteorder::{ByteOrder,LittleEndian};
    /*
    Little endian is the byte order used.
    */

    #[derive(Debug)]
    pub struct PrimaryHeader{
        
        ver_no:u8, // 3 bits
        // packet identification
        type_flag:bool,
        sec_header_flag:bool,
        apid:u16, // 11 bits

        // packet sequence control
        seq_flags: (bool,bool), // 2 bits
        packet_name: u16, // 14 bits at most
        
        data_len: u16, // 16 bits
        
    }
    
    // TODO check parameters
    fn get_bits_u32(num:u32,len:u8,start:u8,end:u8) -> u32{
        let x = len - end;
        let mut res = num >> x;
        res = res & ((1 << (end - start)) -1);
        res
    }


    impl PrimaryHeader{
        // TODO write endianness
        
        pub fn new(packet:&str) -> Self{
            // TODO check length of packet

            let packet_int = LittleEndian::read_u32(&packet.as_bytes()[0..4]);            

            let ver_no_:u8 = get_bits_u32(packet_int,32,0,3) as u8;
            let type_flag_:bool = 1 == get_bits_u32(packet_int,32,3,4);
            let sec_header_flag_:bool = 1 == get_bits_u32(packet_int,32,4,5);
            let apid_:u16 = get_bits_u32(packet_int,32,5,16) as u16;
            let seq_flags_:(bool,bool) = (
                                            get_bits_u32(packet_int,32,16,17) == 1,
                                            get_bits_u32(packet_int,32,17,18) == 1
                                        );
            let packet_name_:u16 = get_bits_u32(packet_int,32,18,32) as u16;
            let data_len_: u16 = LittleEndian::read_u16(&packet.as_bytes()[4..6]);

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
        // Returns a static 6 byte u8 array
        pub fn to_bytes(&self) -> [u8;6]{
            let mut res:[u8;6] = [0;6];
            // TODO write res[0..4]
            // writing the data length is trivial
            LittleEndian::write_u16(&mut res[4..6],self.data_len);
            res
        }

    }
    
}