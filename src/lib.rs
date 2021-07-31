mod decode;
mod encode;
mod fields;
#[cfg(feature = "lx")]
pub mod lx;

pub use decode::*;
pub use encode::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Record {
    pub flarm_id: String,
    pub pilot_name: String,
    pub airfield: String,
    pub plane_type: String,
    pub registration: String,
    pub call_sign: String,
    pub frequency: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct File {
    pub version: u32,
    pub records: Vec<Record>,
}
