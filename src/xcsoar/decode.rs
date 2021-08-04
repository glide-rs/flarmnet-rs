use super::fields::*;
use crate::Record;
use encoding_rs::mem::decode_latin1;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("missing file version")]
    MissingVersion,
    #[error("invalid file version: {0}")]
    InvalidVersion(String),
    #[error("unexpected line length: {0} characters")]
    UnexpectedLineLength(usize),
    #[error("unexpected character(s): {0}")]
    UnexpectedCharacter(String),
    #[error("invalid FLARM id: {0}")]
    InvalidFlarmId(String),
}

#[derive(Debug)]
pub struct DecodedFile {
    pub version: u32,
    pub records: Vec<Result<Record, DecodeError>>,
}

/// Decodes a FlarmNet file.
///
/// # Examples
///
/// ```
/// # use flarmnet::Record;
/// let file = r#"006fb0
/// 3030303030304dfc6c6c6572202020202020202020202020202020442d3231383820202020202020202020202020202041534b2d3133202020202020202020202020202020442d32313838202020203132332e313530
/// 30303030303120202020202020202020202020202020202020202030303030303020202020202020202020202020202050617261676c6964657220202020202020202020203030303030302020202020202020202020
/// 303030303066202020202020202020202020202020202020202020442d39353237202020202020202020202020202020415357203237202020202020202020202020202020442d393532372058323720202020202020
/// "#;
///
/// let result = flarmnet::xcsoar::decode_file(file).unwrap();
/// assert_eq!(result.version, 28592);
/// assert_eq!(result.records.iter().filter(|it| it.is_ok()).count(), 3);
/// ```
pub fn decode_file(file: &str) -> Result<DecodedFile, DecodeError> {
    let mut lines = file.lines();

    let version = lines.next().ok_or(DecodeError::MissingVersion)?;
    let version = u32::from_str_radix(version, 16)
        .map_err(|_| DecodeError::InvalidVersion(version.to_string()))?;

    let records = lines
        .filter(|it| !it.is_empty())
        .map(|it| decode_record(it))
        .collect();

    Ok(DecodedFile { version, records })
}

/// Decodes a single FlarmNet file record.
///
/// # Examples
///
/// ```
/// # use flarmnet::Record;
/// let line = "334545334337546f62696173204269656e69656b2020202020202045444b4120202020202020202020202020202020204c5336612020202020202020202020202020202020442d30383136205347203133302e353330";
///
/// let result = flarmnet::xcsoar::decode_record(line);
/// assert_eq!(result.unwrap(), Record {
///     flarm_id: "3EE3C7".to_string(),
///     pilot_name: "Tobias Bieniek".to_string(),
///     airfield: "EDKA".to_string(),
///     plane_type: "LS6a".to_string(),
///     registration: "D-0816".to_string(),
///     call_sign: "SG".to_string(),
///     frequency: "130.530".to_string(),
/// });
/// ```
pub fn decode_record(line: &str) -> Result<Record, DecodeError> {
    let line_length = line.len();
    if line_length != LINE_LENGTH {
        return Err(DecodeError::UnexpectedLineLength(line_length));
    }

    let flarm_id = decode_str(&line[FLARM_ID_RANGE])?;
    if u32::from_str_radix(&flarm_id, 16).is_err() {
        return Err(DecodeError::InvalidFlarmId(flarm_id));
    }

    let pilot_name = decode_str(&line[PILOT_NAME_RANGE])?;
    let airfield = decode_str(&line[AIRFIELD_RANGE])?;
    let plane_type = decode_str(&line[PLANE_TYPE_RANGE])?;
    let registration = decode_str(&line[REGISTRATION_RANGE])?;
    let call_sign = decode_str(&line[CALL_SIGN_RANGE])?;
    let frequency = decode_str(&line[FREQUENCY_RANGE])?;

    Ok(Record {
        flarm_id,
        pilot_name,
        airfield,
        plane_type,
        registration,
        call_sign,
        frequency,
    })
}

fn decode_str(value: &str) -> Result<String, DecodeError> {
    debug_assert_eq!(value.len() % 2, 0, "argument length must be even");

    let bytes = (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| DecodeError::UnexpectedCharacter(value.to_string()))?;

    Ok(decode_latin1(&bytes).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::{decode_file, decode_record};
    use insta::assert_debug_snapshot;

    #[test]
    fn decoding_fails_for_empty_file() {
        let file = "";
        assert_debug_snapshot!(decode_file(file).unwrap_err(), @"MissingVersion");
    }

    #[test]
    fn decoding_fails_for_invalid_file_version() {
        let file = "0123x4\n";
        assert_debug_snapshot!(
            decode_file(file).unwrap_err(),
            @r###"
        InvalidVersion(
            "0123x4",
        )
        "###
        );
    }

    #[test]
    fn decoding_fails_for_short_line() {
        let row = "3030303030304dfc6c6c6572202020202020202020202020202020442d3231383820202020202020202020202020202041534b2d3133202020202020202020202020202020442d32313838202020203132332e31353";
        assert_debug_snapshot!(
            decode_record(row).unwrap_err(),
            @r###"
        UnexpectedLineLength(
            171,
        )
        "###
        );
    }

    #[test]
    fn decoding_fails_for_long_line() {
        let row = "3030303030304dfc6c6c6572202020202020202020202020202020442d3231383820202020202020202020202020202041534b2d3133202020202020202020202020202020442d32313838202020203132332e3135301";
        assert_debug_snapshot!(
            decode_record(row).unwrap_err(),
            @r###"
        UnexpectedLineLength(
            173,
        )
        "###
        );
    }

    #[test]
    fn decoding_fails_for_unexpected_characters() {
        let row = "30XX303030304dfc6c6c6572202020202020202020202020202020442d3231383820202020202020202020202020202041534b2d3133202020202020202020202020202020442d32313838202020203132332e313530";
        assert_debug_snapshot!(
            decode_record(row).unwrap_err(),
            @r###"
        UnexpectedCharacter(
            "30XX30303030",
        )
        "###
        );
    }

    #[test]
    fn decoding_fails_for_empty_flarm_id() {
        let row = "2020202020204dfc6c6c6572202020202020202020202020202020442d3231383820202020202020202020202020202041534b2d3133202020202020202020202020202020442d32313838202020203132332e313530";
        assert_debug_snapshot!(
            decode_record(row).unwrap_err(),
            @r###"
        InvalidFlarmId(
            "",
        )
        "###
        );
    }

    #[test]
    fn decoding_fails_for_invalid_flarm_id() {
        let row = "3030573030304dfc6c6c6572202020202020202020202020202020442d3231383820202020202020202020202020202041534b2d3133202020202020202020202020202020442d32313838202020203132332e313530";
        assert_debug_snapshot!(
            decode_record(row).unwrap_err(),
            @r###"
        InvalidFlarmId(
            "00W000",
        )
        "###
        );
    }
}
