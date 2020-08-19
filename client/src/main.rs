extern crate serialport;
use pus::sp::{SpacePacket,tc::TcPacket,tc::service_8::Service8_1};
use serialport::{available_ports, open_with_settings,SerialPortSettings};
use serialport::prelude::*;
use std::time::Duration;
use std::env;

 
fn main(){
    let args: Vec<String> = env::args().collect();
    let switch:u8 = args[2].parse().unwrap();
    let tm = SpacePacket::<TcPacket::<Service8_1>>::new(
        2,
        0,
        args[1].clone(),
        1,
        vec![switch]
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

    let mes = tm.to_bytes();
    serialport.write(mes.as_slice()).unwrap();
}