#![feature(allow_internal_unstable)]
#![cfg_attr(not(any(test,feature="debug")),no_std)] // #![no_std] if not testing
#[macro_use]
extern crate arrayref;

pub const FUNC_ID_LEN:usize = 16;
// Type alias for storing func_id
pub type FuncId = arrayvec::ArrayString::<[u8;FUNC_ID_LEN]>;

/// Macro for debugging
/// 
#[macro_export]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! debug {
    () => ({
            #[cfg(feature="debug")]{
                $crate::std::print!("\n");
            }
    });
    ($($arg:tt)*) => ({
        #[cfg(feature="debug")]{
            {
                std::io::_print(std::format_args_nl!($($arg)*));
            }
        }
    })
}

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
        };
);

/// Module for Generic CCSDS Space Packet. SpacePacket struct has only primary header and data field. 
pub mod sp;


// TODO : Implement Own Error Types