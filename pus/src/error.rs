// Data needed for reporting the error by every error code (in terms of bytes).
// For example zero data for error code 0 = ERR_CODE_DATA_LEN[0].
pub const ERR_CODE_COUNT:usize = 3;
pub static ERR_CODE_DATA_LEN:[usize;ERR_CODE_COUNT] = [
    0,
    0,
    0,
];
