//! Decoder/Encoder for LXNavigation/XCSoar/WinPilot/LK8000/ClearNav file format.
//!
//! The [decode_file] function can be used to decode FlarmNet files. The
//! [encode_file] function can be used to write such files.

mod decode;
mod encode;
mod fields;

pub use decode::*;
pub use encode::*;
