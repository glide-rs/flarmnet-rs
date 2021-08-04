use super::fields::*;
use crate::{File, Record};
use encoding_rs::mem::{encode_latin1_lossy, is_str_latin1};
use std::io::{Cursor, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    // the value could not be converted to valid latin1
    #[error("invalid encoding: {0}")]
    InvalidEncoding(String),
}

/// Encodes a FlarmNet file.
///
/// # Examples
///
/// ```
/// # use flarmnet::Record;
/// let file = flarmnet::File {
///     version: 123,
///     records: vec![
///         Record {
///             flarm_id: "3EE3C7".to_string(),
///             pilot_name: "Tobias Bieniek".to_string(),
///             airfield: "EDKA".to_string(),
///             plane_type: "LS6a".to_string(),
///             registration: "D-0816".to_string(),
///             call_sign: "SG".to_string(),
///             frequency: "130.530".to_string(),
///         }
///     ]
/// };
///
/// let result = flarmnet::xcsoar::encode_file(&file).unwrap();
/// assert_eq!(result, br#"00007b
/// 334545334337546f62696173204269656e69656b2020202020202045444b4120202020202020202020202020202020204c5336612020202020202020202020202020202020442d30383136205347203133302e353330
/// "#);
/// ```
pub fn encode_file(file: &File) -> Result<Vec<u8>, EncodeError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write(file)?;

    let buffer = writer.into_inner().into_inner();

    Ok(buffer)
}

#[derive(Clone)]
pub struct Writer<W: Write> {
    writer: W,
}

impl<W: Write> Writer<W> {
    pub fn new(inner: W) -> Self {
        Self { writer: inner }
    }

    pub fn write(&mut self, file: &File) -> Result<(), EncodeError> {
        self.write_version(file.version)?;
        for record in &file.records {
            self.write_record(record)?;
        }

        Ok(())
    }

    fn write_version(&mut self, version: u32) -> Result<(), EncodeError> {
        self.writer.write_fmt(format_args!("{:06x?}", version))?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }

    fn write_record(&mut self, record: &Record) -> Result<(), EncodeError> {
        self.write_str(&record.flarm_id, FLARM_ID_LENGTH)?;
        self.write_str(&record.pilot_name, PILOT_NAME_LENGTH)?;
        self.write_str(&record.airfield, AIRFIELD_LENGTH)?;
        self.write_str(&record.plane_type, PLANE_TYPE_LENGTH)?;
        self.write_str(&record.registration, REGISTRATION_LENGTH)?;
        self.write_str(&record.call_sign, CALL_SIGN_LENGTH)?;
        self.write_str(&record.frequency, FREQUENCY_LENGTH)?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }

    pub(crate) fn write_str(&mut self, value: &str, length: usize) -> Result<(), EncodeError> {
        if !is_str_latin1(value) {
            return Err(EncodeError::InvalidEncoding(value.to_string()));
        }

        let bytes = encode_latin1_lossy(&value);
        for byte in bytes.iter().take(length) {
            self.writer.write_fmt(format_args!("{:02x?}", byte))?;
        }

        let bytes_len = bytes.len();
        if bytes_len < length {
            let placeholders = b"20".repeat(length - bytes_len);
            self.writer.write_all(&placeholders)?;
        }

        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use super::{EncodeError, Writer};
    use insta::assert_debug_snapshot;
    use std::io::Cursor;

    fn encode_str(value: &str, length: usize) -> Result<String, EncodeError> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write_str(value, length)?;
        let bytes = writer.into_inner().into_inner();
        Ok(String::from_utf8(bytes).unwrap())
    }

    #[test]
    fn encoding_works() {
        assert_eq!(encode_str("D-4711", 7).unwrap(), "442d3437313120");
        assert_eq!(encode_str("1234567890", 7).unwrap(), "31323334353637");
        assert_eq!(encode_str("Müller", 10).unwrap(), "4dfc6c6c657220202020");
    }

    #[test]
    fn encoding_fails_for_non_latin1() {
        assert_debug_snapshot!(
            encode_str("😅", 7).unwrap_err(),
            @r###"
            InvalidEncoding(
                "😅",
            )
            "###);
    }
}
