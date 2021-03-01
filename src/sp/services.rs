use super::*;
use alloc::{string::String, vec::Vec};
// Each packet transporting a request verification report shall be of service type 1.
pub mod service_1;
// Each packet transporting a housekeeping message shall be of service type 3.
pub mod service_3;
// Each packet transporting a function management message shall be of service type 8.
pub mod service_8;
