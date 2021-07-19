mod decode;
mod fields;

pub use decode::*;

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
