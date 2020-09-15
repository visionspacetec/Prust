extern crate serialport;
use pus::sp::{SpacePacket,services};
use serialport::{available_ports, open_with_settings,SerialPortSettings};
use serialport::prelude::*;
use std::time::Duration;
use std::string::String;
use byteorder::{ByteOrder,BigEndian};
extern crate clap;
use clap::{Arg, App, SubCommand};
 
const ERROR_NAMES:[&str; pus::error::ERR_CODE_COUNT]= [
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
fn main(){
    // CLI Setup
    let app = App::new("client")
        .arg(Arg::with_name("exit_after_n").short("n")
            .number_of_values(1).default_value("2")
            .help("Number of packets to recieve before termination")
        )
        .subcommand(
            SubCommand::with_name("exec_func")
                .arg(Arg::with_name("func_name").short("f").takes_value(true).help("func_id of the function"))
                .arg(Arg::with_name("args").short("a").min_values(0).value_delimiter(",").help("arguments of the function"))
                .about("Sends a request to execute a function defined")
        )
        .subcommand(
            SubCommand::with_name("new_report")
                .about("Creates new housekeeping report structure")
                .arg(
                    Arg::with_name("hk_id")
                    .number_of_values(1)
                    .help("housekeeping structure id")
                )
                .arg(
                    Arg::with_name("param_ids")
                    .min_values(1)
                    .help("parameters that will be reported in this structure")
                )
        )
        .subcommand(
            SubCommand::with_name("one_shot")
            .about("Sends a one shot request for the specified hk id")
            .arg(
                Arg::with_name("hk_ids")
                .min_values(1)
                .help("Housekeeping structure to be reported")
            )
        ).get_matches();
    
    // get the loop count
    let mut n:u16 = app.value_of("exit_after_n").unwrap().parse().unwrap();

    // Matching subcommands
    let mes:Vec<u8> =  match app.subcommand() {
        ("exec_func", Some(exec_matches)) => {
            let func_id = exec_matches.value_of("func_name").unwrap();
            let args_field:Vec<u8> = exec_matches.values_of("args").unwrap()
            .map(
                |s| s.parse::<u8>().unwrap()
            ).collect();
            SpacePacket::new_service_8_1(
                42, 0, 
                func_id.to_string(), 
                args_field.len() as u8, args_field
            ).unwrap().to_bytes()
        }
        ("new_report",Some(new_matches)) => {
            let report_id = new_matches.value_of("hk_id").unwrap().parse().expect("hk_id should be u8");
            let params:Vec<u8> = new_matches.values_of("param_ids").unwrap()
            .map(
                |s| s.parse::<u8>().unwrap()
            ).collect();
            SpacePacket::new_service_3_1(
                42, 0, 
                report_id, 0, 
                params.len() as u8,params
            ).unwrap().to_bytes()
        }
        ("one_shot",Some(shot_matches)) => {
            let params:Vec<u8> = shot_matches.values_of("hk_ids").unwrap()
            .map(
                |s| s.parse::<u8>().unwrap()
            ).collect();
            SpacePacket::new_service_3_27(
                42, 0, 
                params.len() as u8, params
            ).unwrap().to_bytes()
        }
        _ => Vec::new()
    };
    if mes.len() == 0 {
        panic!("Enter a valid subcommand");
    }
    // Etablishing serial connetion
    let s = SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000),
    };
    // Open the first serialport available.
    let mut serialport = open_with_settings(
        &available_ports().expect("No serial port").get(0)
        .expect("No serial port listed").port_name,&s
    ).expect("Failed to open serial port");
    
    // printing the message send
    println!("The packet send:\n{:?}",mes);
    serialport.write(mes.as_slice()).unwrap();
    
    //sleep
    serialport.flush().unwrap();
    let mut clone = serialport.try_clone().expect("Failed to clone");
    std::thread::sleep(std::time::Duration::from_millis(100));
    clone.flush().unwrap();
    
    while n > 0{
        // Get response
        let mut buf:Vec<u8> = vec![0;1024];
        while clone.read_data_set_ready().unwrap() {};
        if clone.read_exact(&mut buf[0..6]).is_err(){
            continue;
        };
        let ph = pus::sp::PrimaryHeader::from_bytes(&buf[0..6]).unwrap();
        let data_len = ph.get_data_len() + 1;
        if clone.read_exact(&mut buf[6..data_len+6]).is_err(){
            continue;
        }
        let data_len = data_len + 6;
        let ser_type = pus::sp::get_service_type(&ph,&buf[0..data_len]);

        if ser_type == (1,7) {
            println!("TM SUCCESS RESPONSE");
            let res_pack = 
            SpacePacket::<pus::sp::tm::TmPacket<services::service_1::ServiceSuccess>>::from_bytes(&buf[0..data_len]).unwrap();
            println!("TM pack:\n{:#?}",res_pack);
        } else if ser_type == (1,8){
            println!("TM FAILURE RESPONSE"); 
            let res_pack = 
            SpacePacket::<pus::sp::tm::TmPacket<services::service_1::Service1_8>>::from_bytes(&buf[0..data_len]).unwrap();
            let (code,data) = res_pack.get_err();
            println!("Error Type:{:?}\nError Data:{:?}",ERROR_NAMES[code as usize],String::from_utf8(data).unwrap());
            println!("TM pack:\n{:#?}",res_pack);
        } else if ser_type == (3,25){
            println!("Tm One Shot Response"); 
            let res_pack = 
            SpacePacket::<pus::sp::tm::TmPacket<services::service_3::service_3_25::Service3_25>>::from_bytes(&buf[0..data_len]).unwrap();
            println!("PERIPHERAL VALUE RECIEVED:{}",BigEndian::read_u16(res_pack.get_parameter_values().as_slice()));
            println!("TM pack:\n{:#?}",res_pack);
            // Just for the demo
        } else{
            println!("Other");
            println!("{:?}",buf);
        }
        println!("The packet recieved (in bytes):\n{:?}",&buf[0..data_len]);
        n-=1;
    }
}