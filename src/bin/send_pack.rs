//use spp::primary_header;
//extern crate spp;
use spp::headers;

fn main(){
    let mes = "selman";
    let pack = headers::PrimaryHeader::new(mes);
    println!("{:?}",pack);
}