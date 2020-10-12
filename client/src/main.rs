extern crate serialport;
use byteorder::{BigEndian, ByteOrder};
use pus::sp::{services, SpacePacket};
use serialport::prelude::*;
use serialport::{available_ports, open_with_settings, SerialPortSettings};
use std::string::String;
use std::thread;
use std::time::Duration;
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rustyline::{config::Configurer, error::ReadlineError};
use rustyline::Editor;

const ERROR_NAMES: [&str; pus::error::ERR_CODE_COUNT] = [
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
    "CapacityError",
];
fn main() {
    // CLI Setup
    let app = App::new("tc").global_setting(AppSettings::NoBinaryName)
        .subcommand(
            SubCommand::with_name("q").about("quit")
        )
        .subcommand(
            SubCommand::with_name("exec_func")
                .about("Sends a request to execute a function defined")
                .arg(
                    Arg::with_name("func_name").required(true)
                        .help("func_id of the function")
                        .number_of_values(1),
                )
                .arg(
                    Arg::with_name("args").required(true)
                        .min_values(0)
                        .help("arguments of the function"),
                ),
        )
        .subcommand(
            SubCommand::with_name("new_report")
                .about("Creates new housekeeping report structure")
                .arg(
                    Arg::with_name("hk_id")
                        .number_of_values(1).required(true)
                        .help("housekeeping structure id"),
                )
                .arg(
                    Arg::with_name("param_ids").required(true)
                        .min_values(1)
                        .help("parameters that will be reported in this structure"),
                ),
        )
        .subcommand(
            SubCommand::with_name("one_shot")
                .about("Sends a one shot request for the specified hk id")
                .arg(
                    Arg::with_name("hk_ids").required(true)
                        .min_values(1)
                        .help("Housekeeping structure id to be reported"),
                ),
        )
        .subcommand(
            SubCommand::with_name("periodic_en")
                .about("Enables peridic report of parameters of the given struct ids")
                .arg(
                    Arg::with_name("structure_ids").required(true)
                        .min_values(1).required(true)
                        .help("Housekeeping structure id to be configured"),
                ),
        )
        .subcommand(
            SubCommand::with_name("periodic_dis")
                .about("Disables peridic report of parameters of the given struct ids")
                .arg(
                    Arg::with_name("structure_ids").required(true)
                        .min_values(1)
                        .help("Housekeeping structure id to be configured"),
                ),
        );

    let mut rl = Editor::<()>::new();
    rl.set_auto_add_history(true);


    // Etablishing serial connetion
    let s = SerialPortSettings {
        baud_rate: 57_600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000),
    };
    // Open the first serialport available.
    let mut serialport = open_with_settings(
        &available_ports()
            .expect("No serial port")
            .get(0)
            .expect("No serial port listed")
            .port_name,
        &s,
    )
    .expect("Failed to open serial port");

    

    //sleep
    let mut clone = serialport.try_clone().expect("Failed to clone");
    std::thread::sleep(std::time::Duration::from_millis(100));

    // get tm
    thread::spawn(move || {
        // Create a path to the output file
        let path = Path::new("out.txt");
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(file) => file,
        };
        loop {
            // Get response
            let mut buf: Vec<u8> = vec![0; 1024];
            while clone.read_data_set_ready().unwrap() {}
            if clone.read_exact(&mut buf[0..6]).is_err() {
                continue;
            };
            let ph = match pus::sp::PrimaryHeader::from_bytes(&buf[0..6]) {
                Ok(p) => p,
                Err(_) => {
                    continue;
                }
            };

            let data_len = ph.get_data_len() + 1;
            // if it exceeds the buffer ignore
            if data_len + 6 > 1024 || clone.read_exact(&mut buf[6..data_len + 6]).is_err() {
                continue;
            }
            let data_len = data_len + 6;
            let ser_type = match pus::sp::get_service_type(&buf[0..data_len]) {
                Ok(res) => res,
                _ => continue,
            };
            if ser_type == (1, 7) {
                writeln!(file, "TM SUCCESS RESPONSE").unwrap();
                let res_pack = SpacePacket::<
                    pus::sp::tm::TmPacket<services::service_1::ServiceSuccess>,
                >::from_bytes(&buf[0..data_len])
                .unwrap();
                writeln!(file, "TM pack:\n{:#?}", res_pack).unwrap();
            } else if ser_type == (1, 8) {
                writeln!(file, "TM FAILURE RESPONSE").unwrap();
                let res_pack =
                SpacePacket::<pus::sp::tm::TmPacket<services::service_1::Service1_8>>::from_bytes(&buf[0..data_len]).unwrap();
                let (code, data) = res_pack.get_err();
                writeln!(file, "TM pack:\n{:#?}", res_pack).unwrap();
                writeln!(
                    file,
                    "Error Type:{:?}\nError Data:{:?}",
                    ERROR_NAMES[code as usize],
                    String::from_utf8(data).unwrap()
                )
                .unwrap();
            } else if ser_type == (3, 25) {
                writeln!(file,"Tm Parameter Response").unwrap();
                let res_pack = SpacePacket::<
                    pus::sp::tm::TmPacket<services::service_3::service_3_25::Service3_25>,
                >::from_bytes(&buf[0..data_len])
                .unwrap();
                writeln!(file, "TM pack:\n{:#?}", res_pack).unwrap();
                writeln!(
                    file,
                    "PERIPHERAL VALUE RECIEVED:{}",
                    BigEndian::read_u16(res_pack.get_parameter_values().as_slice())
                )
                .unwrap();
            // Just for the demo
            } else {
                // if unrecognized ignore
                continue;
            }
            writeln!(
                file,
                "The packet recieved (in bytes):\n{:?}",
                &buf[0..data_len]
            )
            .unwrap();
        }
    });

    // send tc
    loop {
        //print!("> ");
        //std::io::stdout().flush().expect("Couldn't flush stdout");
        //let mut input = String::new();
        // std::io::stdin()
        //    .read_line(&mut input)
        //    .expect("Error reading input.");
        let input = match rl.readline(">> ") 
         {
            Ok(l) => l,
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                std::process::exit(0x0000)
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                std::process::exit(0x0000)
            },
            Err(err) => {
                println!("Error: {:?}", err);
                std::process::exit(0x0100)
            }

         };
        let args: Vec<&str> = input.split_ascii_whitespace().collect();
        let matc = match app.clone().get_matches_from_safe(args){
            Ok(m)=> m,
            Err(e) => {
                println!("{}",e.message); 
                continue
            }
        };

        // Matching subcommands
        let mes: Vec<u8> = match matc.subcommand() {
            ("exec_func", Some(exec_matches)) => {
                let func_id = exec_matches.value_of("func_name").unwrap();
                let args_field: Vec<u8> = exec_matches
                    .values_of("args")
                    .unwrap_or_default()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                SpacePacket::new_service_8_1(
                    42,
                    0,
                    func_id.to_string(),
                    args_field.len() as u8,
                    args_field,
                )
                .unwrap()
                .to_bytes()
            }
            ("new_report", Some(new_matches)) => {
                let report_id = new_matches
                    .value_of("hk_id")
                    .unwrap()
                    .parse()
                    .expect("hk_id should be u8");
                let params: Vec<u8> = new_matches
                    .values_of("param_ids")
                    .unwrap()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                SpacePacket::new_service_3_1(42, 0, report_id, 1, params.len() as u8, params)
                    .unwrap()
                    .to_bytes()
            }
            ("periodic_en", Some(ids)) => {
                let ids: Vec<u8> = ids
                    .values_of("structure_ids")
                    .unwrap()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                SpacePacket::new_service_3_5(42, 0, ids.len() as u8, ids)
                    .unwrap()
                    .to_bytes()
            }
            ("periodic_dis", Some(ids)) => {
                let ids: Vec<u8> = ids
                    .values_of("structure_ids")
                    .unwrap()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                SpacePacket::new_service_3_6(42, 0, ids.len() as u8, ids)
                    .unwrap()
                    .to_bytes()
            }
            ("one_shot", Some(shot_matches)) => {
                let params: Vec<u8> = shot_matches
                    .values_of("hk_ids")
                    .unwrap()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                SpacePacket::new_service_3_27(42, 0, params.len() as u8, params)
                    .unwrap()
                    .to_bytes()
            }
            ("q",_) => {
                std::process::exit(0x0000)
            }
            _ => Vec::new(),
        };
        if mes.len() == 0 {
            panic!("Enter a valid subcommand");
        }
        // printing the message send
        println!("The packet send:\n{:?}", mes);
        serialport.write(mes.as_slice()).unwrap();
    }

}
