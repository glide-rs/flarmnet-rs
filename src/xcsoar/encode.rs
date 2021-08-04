use super::fields::*;
use crate::{File, Record};
use encoding_rs::mem::{encode_latin1_lossy, is_str_latin1};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
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
/// assert_eq!(result, r#"00007b
/// 334545334337546f62696173204269656e69656b2020202020202045444b4120202020202020202020202020202020204c5336612020202020202020202020202020202020442d30383136205347203133302e353330
/// "#);
/// ```
pub fn encode_file(file: &File) -> Result<String, EncodeError> {
    let version = format!("{:06x?}", file.version);
    let records = file
        .records
        .iter()
        .map(|record| encode_record(&record))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");

    Ok(format!("{}\n{}\n", version, records))
}

/// Encodes a single FlarmNet file record.
///
/// # Examples
///
/// ```
/// # use flarmnet::Record;
/// let record = Record {
///     flarm_id: "3EE3C7".to_string(),
///     pilot_name: "Tobias Bieniek".to_string(),
///     airfield: "EDKA".to_string(),
///     plane_type: "LS6a".to_string(),
///     registration: "D-0816".to_string(),
///     call_sign: "SG".to_string(),
///     frequency: "130.530".to_string(),
/// };
///
/// assert_eq!(
///     flarmnet::xcsoar::encode_record(&record).unwrap(),
///     "334545334337546f62696173204269656e69656b2020202020202045444b4120202020202020202020202020202020204c5336612020202020202020202020202020202020442d30383136205347203133302e353330",
/// );
/// ```
pub fn encode_record(record: &Record) -> Result<String, EncodeError> {
    let fields = vec![
        encode_str(&record.flarm_id, FLARM_ID_LENGTH)?,
        encode_str(&record.pilot_name, PILOT_NAME_LENGTH)?,
        encode_str(&record.airfield, AIRFIELD_LENGTH)?,
        encode_str(&record.plane_type, PLANE_TYPE_LENGTH)?,
        encode_str(&record.registration, REGISTRATION_LENGTH)?,
        encode_str(&record.call_sign, CALL_SIGN_LENGTH)?,
        encode_str(&record.frequency, FREQUENCY_LENGTH)?,
    ];
    Ok(fields.join(""))
}

fn encode_str(value: &str, length: usize) -> Result<String, EncodeError> {
    if !is_str_latin1(value) {
        return Err(EncodeError::InvalidEncoding(value.to_string()));
    }

    let latin1_encoded = encode_latin1_lossy(&value);
    let mut encoded: Vec<_> = latin1_encoded
        .iter()
        .take(length)
        .map(|byte| format!("{:02x?}", byte))
        .collect();

    while encoded.len() < length {
        encoded.push("20".to_string());
    }

    let joined = encoded.join("");
    Ok(joined)
}

#[cfg(test)]
mod tests {
    use super::encode_str;
    use insta::assert_debug_snapshot;

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
