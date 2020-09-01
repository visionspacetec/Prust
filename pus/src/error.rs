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
    InvalidFuncId,
}
pub const ERR_CODE_COUNT:usize = 7;
pub static ERR_CODE_DATA_LEN:[usize;ERR_CODE_COUNT] = [0;ERR_CODE_COUNT];
