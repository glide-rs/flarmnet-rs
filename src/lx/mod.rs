//! Decoder/Encoder for LXNav/Naviter file format.
//!
//! The file format is based on XML and "encrypted" by shifting each byte
//! by one.
//!
//! After decryption the XML structure looks like this:
//! ```xml
//! <?xml version="1.0" encoding="UTF-8" ?>
//! <FLARMNET Version="012345">
//!   <FLARMDATA FlarmID="000001">
//!     <NAME>John Doe</NAME>
//!     <AIRFIELD>EDKA</AIRFIELD>
//!     <TYPE>ASG 29</TYPE>
//!     <REG>D-KESH</REG>
//!     <COMPID>AS</COMPID>
//!     <FREQUENCY>123.500</FREQUENCY>
//!   </FLARMDATA>
//! </FLARMNET>
//! ```
//!
//! The [decode_file] function can be used to decode FlarmNet files in
//! LXNav/Naviter file format. The [encode_file] function can be used to write
//! such files.

pub mod cipher;
mod decode;
mod encode;

pub use decode::*;
pub use encode::*;
