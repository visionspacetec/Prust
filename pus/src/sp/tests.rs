use super::*;

/* BEGIN pus::PrimaryHeader::get_bits_u32*/
#[test]
fn get_bits_u32_test_for_one_digit_case_1() {
    assert_eq!(0,get_bits_u32(12,0,1));
}
#[test]
fn get_bits_u32_test_for_one_digit_case_2() {
    assert_eq!(1,get_bits_u32(1,31,32)); 
}
#[test]
fn get_bits_u32_test_for_five_digit_case() {
    assert_eq!(11,get_bits_u32(22,27,31));
}

#[test]
#[should_panic]
/// A test showing that this function requires end > start in parameters
fn get_bits_u32_test_should_panic_when_end_is_not_bigger_case(){
    assert_eq!(3,get_bits_u32(1,31,31));
}
/* END pus::PrimaryHeader::get_bits_u32*/

#[test]
/// A test that creates a SpacePacket struct with "new" method then converts it to a
/// byte array with "to_bytes" method and comparing it with the expected bytes
/// Note: This case tests both to_bytes and new methods for the SpacePacket struct 
fn space_packet_new_and_to_bytes_test_creating_from_options_and_comparing_bytes_case(){
    // PrimaryHeader bytes
    let ph_bytes = vec![0x18,0x07,0xc0,0x22,0,0x1a];
    // Data field bytes
    let sp_data:Vec<u8> = vec![0, 0, 0x2A, 0, 0x1, 0, 0x1, 0x1, 0, 0x1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0x22, 0xC0, 0x1, 0x1, 0x1, 0xD, 0xA7, 0xC];
    // Creating with giving the attributes in rust
    let sp = SpacePacket::<Vec<u8>>::new(0,true,true,7,(true,true),34,sp_data).unwrap();
    // Comparing the bytes of the packet by bytes
    for (i,byte) in sp.primary_header.to_bytes().iter().enumerate(){
        assert_eq!(ph_bytes[i],*byte);
    };
}
/* BEGIN pus::SpacePacket::new*/
#[test]
/// Test case where the given parameter to new ver_no is bigger than its max value. We expect the return of an error.
fn space_packet_new_test_expecting_an_error_returning_for_invalid_ver_no_case() {            
    let _sp = SpacePacket::<Vec<u8>>::new(9,true,true,7,(true,true),34,vec![0;27]).expect_err("Didn't give an error.");
}
#[test]
/// Test case where the given parameter to new apid is bigger than its max value. We expect the return of an error.
fn space_packet_new_test_expecting_an_error_returning_for_invalid_apid_case() {            
    let _sp = SpacePacket::<Vec<u8>>::new(0,true,true,2049,(true,true),34,vec![0;27]).expect_err("Didn't give an error.");
}
#[test]
/// Test case where the given parameter packet_name is bigger than its max value. We expect the return of an error.
fn space_packet_new_test_expecting_an_error_returning_for_invalid_packet_name_case() {            
    let _sp = SpacePacket::<Vec<u8>>::new(0,true,true,7,(true,true),1 << 14 + 1,vec![0;27]).expect_err("Didn't give an error.");
}
#[test]
/* END pus::SpacePacket::new*/


/* BEGIN SpacePacket::from_bytes*/
#[test]
/// Creating an SpacePacket struct from the given bytes successfully
fn from_bytes_test_create_pus_success_case(){
    let mut arg1 = vec![0x18,0x07,0xc0,0x22,0,0x1a];
    let mut data1 = vec![0;27];
    arg1.append(&mut data1);

    SpacePacket::<Vec<u8>>::from_bytes(&arg1).unwrap();
}
#[test]
/// Creating an SpacePacket struct from the given bytes with failure because data length is too much
fn from_bytes_test_create_pus_failure_case(){
    let mut arg2 = vec![0x18,0x07,0xc0,0x22,0,0x1a];
    let mut data2 = vec![0;28];
    arg2.append(&mut data2);

    SpacePacket::<Vec<u8>>::from_bytes(&arg2).expect_err("Didn't give an error");
}
/* END SpacePacket::<Vec<u8>>::from_bytes*/
#[test]
/// If there is a failure here check setters and getters.
/// All the parameter check of setters and getters are tested here.
fn getter_setters_test_cases(){
    let mut sp = SpacePacket::<Vec<u8>>::new(0,false
        ,false,0
        ,(false,false),0
        ,vec![0]).unwrap();
    
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

#[test]
fn builder_data_loss_check_tc_header(){
    let tc_sec_header = tc::TcPacketHeader::new(
        (true,false,true,false),
        8,
        1,
        12
    ).unwrap();

    let tc_bytes = tc_sec_header.to_bytes();
    let dup_pack = tc::TcPacketHeader::from_bytes(&tc_bytes);
    let dup_pack_bytes = dup_pack.unwrap().to_bytes();

    assert_eq!(tc_bytes.len(),dup_pack_bytes.len());
    for i in 0..tc_bytes.len() {
        assert_eq!(dup_pack_bytes[i],tc_bytes[i]);
    }
}

#[test]
fn builder_data_loss_check_tm_header(){
    let tm_sec_header = tm::TmPacketHeader::new(
        1,
        1,
        12
    ).unwrap();

    let tm_bytes = tm_sec_header.to_bytes();
    let dup_pack = tm::TmPacketHeader::from_bytes(&tm_bytes);
    let dup_pack_bytes = dup_pack.unwrap().to_bytes();

    assert_eq!(tm_bytes.len(),dup_pack_bytes.len());
    for i in 0..tm_bytes.len() {
        assert_eq!(dup_pack_bytes[i],tm_bytes[i]);
    }
}

#[test]
/// Case for checking the SeviceSuccesCase builder crudely
fn builder_for_service_success_case(){
    use crate::sp::services::{service_1::*};
    use crate::sp::tm::TmPacket;
    let tc = SpacePacket::<_>::new_service_8_1(
        2,
        1,
        "turn_led".to_string(),
        1,
        vec![1]
    ).unwrap();

    let tm_1_1 = SpacePacket::<TmPacket::<ServiceSuccess>>::new(
        &tc,
        1,
        0,
        0
    ).unwrap();
    
    let tm_bytes_1 = tm_1_1.to_bytes().to_vec();
    let tm2 = SpacePacket::<TmPacket::<ServiceSuccess>>::from_bytes(&tm_bytes_1).unwrap();
    let tm_bytes_2 = tm2.to_bytes();
    assert_eq!(tm_bytes_1.len(),tm_bytes_2.len());
    for i in 0..tm_bytes_1.len() {
        assert_eq!(tm_bytes_1[i],tm_bytes_2[i]);
    }
}
#[test]
/// Case for checking the SeviceFailCase builder crudely
fn builder_for_service_fail_case(){
    use crate::sp::services::{service_1::*};
    use crate::sp::tm::TmPacket;
    let tc = SpacePacket::<_>::new_service_8_1(
        2,
        1,
        "turn_led".to_string(),
        1,
        vec![1]
    ).unwrap();

    let tm_1_1 = SpacePacket::<TmPacket::<ServiceFail>>::new_service_1_2(
        &tc,
        1,1,0,vec![]).unwrap();
    
    let tm_bytes_1 = tm_1_1.to_bytes().to_vec();
    let tm2 = SpacePacket::<TmPacket::<ServiceFail>>::from_bytes(&tm_bytes_1).unwrap();
    let tm_bytes_2 = tm2.to_bytes();
    assert_eq!(tm_bytes_1.len(),tm_bytes_2.len());
    for i in 0..tm_bytes_1.len() {
        assert_eq!(tm_bytes_1[i],tm_bytes_2[i]);
    }
}

#[test]
/// Case for checking the SeviceSuccesStepCase builder crudely
fn builder_for_service_success_step_case(){
    use crate::sp::services::{service_1::*};
    use crate::sp::tm::TmPacket;
    let tc = SpacePacket::<_>::new_service_8_1(
        2,
        1,
        "turn_led".to_string(),
        1,
        vec![1]
    ).unwrap();

    let tm_1_1 = SpacePacket::<_>::new_service_1_5(
        &tc,
        1,1,0).unwrap();
    
    let tm_bytes_1 = tm_1_1.to_bytes().to_vec();
    let tm2 = SpacePacket::<TmPacket::<Service1_5>>::from_bytes(&tm_bytes_1).unwrap();
    let tm_bytes_2 = tm2.to_bytes();
    assert_eq!(tm_bytes_1.len(),tm_bytes_2.len());
    for i in 0..tm_bytes_1.len() {
        assert_eq!(tm_bytes_1[i],tm_bytes_2[i]);
    }
}
#[test]
/// Case for checking the SeviceFailStepCase builder crudely
fn builder_for_service_fail_step_case(){
    use crate::sp::services::{service_1::*};
    use crate::sp::tm::TmPacket;
    let tc = SpacePacket::<_>::new_service_8_1(
        2,
        1,
        "turn_led".to_string(),
        1,
        vec![1]
    ).unwrap();

    let tm_1_1 = SpacePacket::<_>::new_service_1_6(
        &tc,
        1,1,0,vec![],1).unwrap();
    
    let tm_bytes_1 = tm_1_1.to_bytes().to_vec();
    let tm2 = SpacePacket::<TmPacket::<Service1_6>>::from_bytes(&tm_bytes_1).unwrap();
    let tm_bytes_2 = tm2.to_bytes();
    assert_eq!(tm_bytes_1.len(),tm_bytes_2.len());
    for i in 0..tm_bytes_1.len() {
        assert_eq!(tm_bytes_1[i],tm_bytes_2[i]);
    } 
}
#[test]
fn housekeeping_service_3_1_tc_data_field_generation_case(){
    use crate::sp::services::service_3::service_3_1::Service3_1;

    let tc1 = Service3_1::new(0,0,1,vec![1]).unwrap();
    let tc1_bytes = tc1.to_bytes();
    let tc2 = Service3_1::from_bytes(&tc1_bytes).unwrap();
    let tc2_bytes = tc2.to_bytes();
    assert_eq!(tc1_bytes.len(),tc2_bytes.len());
    for i in 0..tc1_bytes.len() {
        assert_eq!(tc1_bytes[i],tc2_bytes[i]);
    }

}
#[test]
fn housekeeping_service_3_1_tc_space_pack_generation_case(){
    use crate::sp::services::service_3::service_3_1::Service3_1;
    use crate::sp::tc::*;
    let tc1 = SpacePacket::new_service_3_1(
        0,0,0,0,2,alloc::vec![1,2]
    ).unwrap();
    let tc1_bytes = tc1.to_bytes();
    let tc2 = SpacePacket::<TcPacket::<Service3_1>>::from_bytes(&tc1_bytes).unwrap();
    let tc2_bytes = tc2.to_bytes();
    assert_eq!(tc1_bytes.len(),tc2_bytes.len());
    for i in 0..tc1_bytes.len() {
        assert_eq!(tc1_bytes[i],tc2_bytes[i]);
    }
}

