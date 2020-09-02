extern crate alloc;
use alloc::string::String;
// Data needed for reporting the error by every error code (in terms of bytes).
// For example zero data for error code 0 = ERR_CODE_DATA_LEN[0].
#[derive(Debug)]
pub enum Error{
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
    CapacityError
}

impl core::convert::From<core::cell::BorrowMutError> for Error {
    fn from(item: core::cell::BorrowMutError) -> Self {
        Error::BorrowMutError(item)
    }
}
impl core::convert::From<core::option::NoneError> for Error {
    fn from(_:core::option::NoneError) -> Self {
        Error::NoneError
    }
}

impl core::convert::From<()> for Error {
    fn from(_:()) -> Self {
        Error::UnitType
    }
}

pub const ERR_CODE_COUNT:usize = 12;
pub static ERR_CODE_DATA_LEN:[usize;ERR_CODE_COUNT] = [0;ERR_CODE_COUNT];
