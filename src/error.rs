extern crate alloc;
use alloc::{borrow::ToOwned, string::String, vec::Vec};
#[derive(Debug)]
pub enum Error {
    UnsupportedRequest,
    InvalidPacket,
    CorruptData,
    InvalidPacketName,
    InvalidVersionNo,
    InvalidApid,
    InvalidFuncId(String),
    PeripheralError,
    BorrowMutError(core::cell::BorrowMutError),
    NoneError,
    UnitType,
    InvalidArg,
    CapacityError,
    HalTimoutError,
    FreeRtosTimoutError
}

impl core::convert::From<core::cell::BorrowMutError> for Error {
    fn from(item: core::cell::BorrowMutError) -> Self {
        Error::BorrowMutError(item)
    }
}
impl core::convert::From<core::option::NoneError> for Error {
    fn from(_: core::option::NoneError) -> Self {
        Error::NoneError
    }
}

impl core::convert::From<()> for Error {
    fn from(_: ()) -> Self {
        Error::UnitType
    }
}

pub fn get_err_code_n_data(err: Error) -> (u8, Vec<u8>) {
    match err {
        Error::UnsupportedRequest => (0, Vec::default()),
        Error::InvalidPacket => (1, Vec::default()),
        Error::InvalidPacketName => (2, Vec::default()),
        Error::InvalidVersionNo => (3, Vec::default()),
        Error::CorruptData => (4, Vec::default()),
        Error::InvalidApid => (5, Vec::default()),
        Error::InvalidFuncId(f_id) => (6, f_id.to_owned().into()),
        Error::PeripheralError => (7, Vec::default()),
        Error::BorrowMutError(_) => (8, Vec::default()),
        Error::NoneError => (9, Vec::default()),
        Error::UnitType => (10, Vec::default()),
        Error::InvalidArg => (11, Vec::default()),
        Error::CapacityError => (12, Vec::default()),
        Error::HalTimoutError => (13, Vec::default()),
        Error::FreeRtosTimoutError => (14, Vec::default())
    }
}
pub const ERR_CODE_COUNT: usize = 13;
