use crate::lx::cipher;
use crate::Record;
use minidom::{Element, NSChoice};
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error(transparent)]
    Xml(#[from] minidom::Error),
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error("missing XML element: {0}")]
    MissingElement(String),
    #[error("missing file version")]
    MissingVersion,
    #[error("invalid file version: {0}")]
    InvalidVersion(String),
    #[error("missing FLARM id")]
    MissingFlarmId,
    #[error("invalid FLARM id: {0}")]
    InvalidFlarmId(String),
}

#[derive(Debug)]
pub struct DecodedFile {
    pub version: u32,
    pub records: Vec<Result<Record, DecodeError>>,
}

/// Decodes a FlarmNet file in LX format.
///
/// # Examples
///
/// ```
/// # use flarmnet::Record;
/// let file = r#"=@ynm!wfstjpo>#2/1#!fodpejoh>#VUG.9#@?\u{b}=GMBSNOFU!Wfstjpo>#117gc1#?\u{b}=GMBSNEBUB!GmbsnJE>#111111#?\u{b}
/// =OBNF?NĽmmfs=0OBNF?\u{b}
/// =BJSGJFME?E.3299=0BJSGJFME?\u{b}
/// =UZQF?BTL.24=0UZQF?\u{b}
/// =SFH?E.3299=0SFH?\u{b}
/// =DPNQJE?=0DPNQJE?\u{b}
/// =GSFRVFODZ?234/261=0GSFRVFODZ?\u{b=0GMBSNEBUB?\u{b=GMBSNEBUB!GmbsnJE>#111112#?\u{b}
/// =OBNF?=0OBNF?\u{b}
/// =BJSGJFME?111111=0BJSGJFME?\u{b}
/// =UZQF?Qbsbhmjefs=0UZQF?\u{b}
/// =SFH?111111=0SFH?\u{b}
/// =DPNQJE?=0DPNQJE?\u{b}
/// =GSFRVFODZ?=0GSFRVFODZ?\u{b=0GMBSNEBUB?\u{b=GMBSNEBUB!GmbsnJE>#11111G#?\u{b}
/// =OBNF?=0OBNF?\u{b}
/// =BJSGJFME?E.:638=0BJSGJFME?\u{b}
/// =UZQF?BTX!38=0UZQF?\u{b}
/// =SFH?E.:638=0SFH?\u{b}
/// =DPNQJE?Y38=0DPNQJE?\u{b}
/// =GSFRVFODZ?=0GSFRVFODZ?\u{b}=0GMBSNEBUB?=0GMBSNOFU?
/// "#;
///
/// let result = flarmnet::lx::decode_file(file).unwrap();
/// assert_eq!(result.version, 28592);
/// assert_eq!(result.records.iter().filter(|it| it.is_ok()).count(), 3);
/// ```
pub fn decode_file(file: &str) -> Result<DecodedFile, DecodeError> {
    let decrypted = cipher::decrypt(file)?;

    let root: Element = decrypted.parse()?;
    if root.name() != "FLARMNET" {
        return Err(DecodeError::MissingElement("FLARMNET".to_string()));
    }

    let version = root.attr("Version").ok_or(DecodeError::MissingVersion)?;
    let version = u32::from_str_radix(version, 16)
        .map_err(|_| DecodeError::InvalidVersion(version.to_string()))?;

    let records = root
        .children()
        .filter(|child| child.name() == "FLARMDATA")
        .map(|child| convert(child))
        .collect();

    Ok(DecodedFile { version, records })
}

/// Converts a `minidom::Element` to a `flarmnet::Record`.
///
/// Expected structure:
///
/// ```xml
/// <FLARMDATA FlarmID="000001">
///   <NAME></NAME>
///   <AIRFIELD>000000</AIRFIELD>
///   <TYPE>Paraglider</TYPE>
///   <REG>000000</REG>
///   <COMPID></COMPID>
///   <FREQUENCY></FREQUENCY>
/// </FLARMDATA>
/// ```
fn convert(element: &Element) -> Result<Record, DecodeError> {
    let flarm_id = element
        .attr("FlarmID")
        .ok_or(DecodeError::MissingFlarmId)?
        .to_string();

    if u32::from_str_radix(&flarm_id, 16).is_err() {
        return Err(DecodeError::InvalidFlarmId(flarm_id));
    }

    let pilot_name = element
        .get_child("NAME", NSChoice::Any)
        .map(|e| e.text())
        .unwrap_or_default();

    let airfield = element
        .get_child("AIRFIELD", NSChoice::Any)
        .map(|e| e.text())
        .unwrap_or_default();

    let plane_type = element
        .get_child("TYPE", NSChoice::Any)
        .map(|e| e.text())
        .unwrap_or_default();

    let registration = element
        .get_child("REG", NSChoice::Any)
        .map(|e| e.text())
        .unwrap_or_default();

    let call_sign = element
        .get_child("COMPID", NSChoice::Any)
        .map(|e| e.text())
        .unwrap_or_default();

    let frequency = element
        .get_child("FREQUENCY", NSChoice::Any)
        .map(|e| e.text())
        .unwrap_or_default();

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

#[cfg(test)]
mod tests {
    use crate::lx::cipher::encrypt;
    use crate::lx::decode::{convert, decode_file};
    use insta::assert_debug_snapshot;
    use minidom::Element;

    #[test]
    fn decoding_fails_for_empty_file() {
        let file = "";
        assert_debug_snapshot!(decode_file(file).unwrap_err(), @r###"
        Xml(
            EndOfDocument,
        )
        "###);
    }

    #[test]
    fn decoding_fails_for_invalid_file() {
        let file = "foo";
        assert_debug_snapshot!(decode_file(file).unwrap_err(), @r###"
        Xml(
            EndOfDocument,
        )
        "###);
    }

    #[test]
    fn decoding_fails_for_missing_root_element() {
        let file = encrypt(
            r#"<?xml version="1.0" encoding="UTF-8"?>
                <FOO>
                </FOO>"#,
        )
        .unwrap();
        assert_debug_snapshot!(decode_file(&file).unwrap_err(), @r###"
        MissingElement(
            "FLARMNET",
        )
        "###);
    }

    #[test]
    fn decoding_fails_for_missing_file_version() {
        let file = encrypt(
            r#"<?xml version="1.0" encoding="UTF-8"?>
                <FLARMNET>
                </FLARMNET>"#,
        )
        .unwrap();
        assert_debug_snapshot!(decode_file(&file).unwrap_err(), @"MissingVersion");
    }

    #[test]
    fn decoding_fails_for_invalid_file_version() {
        let file = encrypt(
            r#"<?xml version="1.0" encoding="UTF-8"?>
                <FLARMNET Version="foo">
                </FLARMNET>"#,
        )
        .unwrap();
        assert_debug_snapshot!(decode_file(&file).unwrap_err(), @r###"
        InvalidVersion(
            "foo",
        )
        "###);
    }

    #[test]
    fn converting_works() {
        let element: Element = r#"
            <FLARMDATA FlarmID="c0ffee">
              <NAME>John Doe</NAME>
              <AIRFIELD>EDKA</AIRFIELD>
              <TYPE>ASG 29</TYPE>
              <REG>D-KESH</REG>
              <COMPID>AS</COMPID>
              <FREQUENCY>123.500</FREQUENCY>
            </FLARMDATA>
        "#
        .parse()
        .unwrap();

        assert_debug_snapshot!(convert(&element).unwrap(), @r###"
        Record {
            flarm_id: "c0ffee",
            pilot_name: "John Doe",
            airfield: "EDKA",
            plane_type: "ASG 29",
            registration: "D-KESH",
            call_sign: "AS",
            frequency: "123.500",
        }
        "###);
    }

    #[test]
    fn converting_fails_for_missing_flarm_id() {
        let element: Element = r#"
            <FLARMDATA>
              <NAME></NAME>
              <AIRFIELD>000000</AIRFIELD>
              <TYPE>Paraglider</TYPE>
              <REG>000000</REG>
              <COMPID></COMPID>
              <FREQUENCY></FREQUENCY>
            </FLARMDATA>
        "#
        .parse()
        .unwrap();

        assert_debug_snapshot!(convert(&element).unwrap_err(), @"MissingFlarmId");
    }

    #[test]
    fn converting_fails_for_invalid_flarm_id() {
        let element: Element = r#"
            <FLARMDATA FlarmID="foo">
              <NAME></NAME>
              <AIRFIELD>000000</AIRFIELD>
              <TYPE>Paraglider</TYPE>
              <REG>000000</REG>
              <COMPID></COMPID>
              <FREQUENCY></FREQUENCY>
            </FLARMDATA>
        "#
        .parse()
        .unwrap();

        assert_debug_snapshot!(convert(&element).unwrap_err(), @r###"
        InvalidFlarmId(
            "foo",
        )
        "###);
    }
}
