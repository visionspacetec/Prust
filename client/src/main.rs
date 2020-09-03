extern crate serialport;
use pus::sp::{SpacePacket,tc::TcPacket,services::service_8::*,services};
use serialport::{available_ports, open_with_settings,SerialPortSettings};
use serialport::prelude::*;
use std::time::Duration;
use std::env;
use std::string::String;

 
fn main(){
    let args: Vec<String> = env::args().collect();
    let mut tc_args = Vec::<u8>::new();
    for i in &args[2..] {
        tc_args.push(i.parse().unwrap());
    }
    let tc = SpacePacket::<TcPacket::<Service8_1>>::new(
        2,
        12,
        args[1].clone(),
        1,
        tc_args
    ).unwrap();

    let s = SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000),
    };
    // Open the first serialport available.
    let mut serialport = open_with_settings(&available_ports().expect("No serial port")[0].port_name,&s)
        .expect("Failed to open serial port");
    
    let error_names = [
        "UnsupportedRequest",
        "InvalidPacket",
        "CorruptData",
        "InvalidPacketName",
        "InvalidVersionNo",
        "InvalidApid",
        "InvalidFuncId",
        "PeripheralError",
        "BorrowMutError",
        "NoneError",
        "UnitType",
        "InvalidArg",
        "CapacityError"
    ];
    let mes = tc.to_bytes();
    serialport.write(mes.as_slice()).unwrap();
    //sleep
    serialport.flush().unwrap();
    let mut clone = serialport.try_clone().expect("Failed to clone");
    // Fix this
    std::thread::sleep(std::time::Duration::from_millis(100));
    clone.flush().unwrap();
    
    let mut buf:Vec<u8> = vec![0;1024];
    clone.read_exact(&mut buf[0..6]).unwrap();
    let ph = pus::sp::PrimaryHeader::from_bytes(&buf[0..6]).unwrap();
    let data_len = ph.get_data_len() + 1;
    clone.read_exact(&mut buf[6..data_len+6]).unwrap();
    let data_len = data_len + 6;
    
    let ser_type = pus::sp::get_service_type(&ph,&buf[0..data_len]);

    if ser_type == (1,7) {
        println!("Success");
        let _res_pack = 
        SpacePacket::<pus::sp::tm::TmPacket<services::service_1::ServiceSuccess>>::from_bytes(&buf[0..data_len]).unwrap();
        println!("{:?}",ph);
    } else if ser_type == (1,8){
        println!("Failure"); 
        println!("{:?}",ph);
        let res_pack = 
        SpacePacket::<pus::sp::tm::TmPacket<services::service_1::Service1_8>>::from_bytes(&buf[0..data_len]).unwrap();
        let (code,data) = res_pack.get_err();
        println!("Error Type:{:?}\nError Data:{:?}",error_names[code as usize],String::from_utf8(data).unwrap());
    } else{
        println!("{:?}",buf);
    }

}