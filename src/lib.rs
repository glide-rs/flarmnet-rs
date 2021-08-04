#[cfg(feature = "lx")]
pub mod lx;
#[cfg(feature = "xcsoar")]
pub mod xcsoar;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Record {
    pub flarm_id: String,
    pub pilot_name: String,
    pub airfield: String,
    pub plane_type: String,
    pub registration: String,
    pub call_sign: String,
    pub frequency: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct File {
    pub version: u32,
    pub records: Vec<Record>,
}
