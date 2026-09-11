use crate::Record;
use crate::lx::cipher;
use quick_xml::escape::{EscapeError, resolve_xml_entity};
use quick_xml::events::{BytesEnd, BytesRef, BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
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
/// let file = br#"=@ynm!wfstjpo>#2/1#!fodpejoh>#VUG.9#@?\u{b}=GMBSNOFU!Wfstjpo>#117gc1#?\u{b}=GMBSNEBUB!GmbsnJE>#111111#?\u{b}
/// =OBNF?Nvmmfs=0OBNF?\u{b}
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
pub fn decode_file(file: &[u8]) -> Result<DecodedFile, DecodeError> {
    let reader = cipher::Reader::new(file);
    let mut reader = Reader::from_reader(BufReader::new(reader));
    let mut buffer = Vec::new();

    let (version, empty) = loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) => {
                if element.name().as_ref() != "FLARMNET" {
                    validated_attribute(&element, None)?;
                    return Err(DecodeError::MissingElement("FLARMNET".to_string()));
                }
                break (read_version(&element)?, false);
            }
            Event::Empty(element) => {
                if element.name().as_ref() != "FLARMNET" {
                    validated_attribute(&element, None)?;
                    return Err(DecodeError::MissingElement("FLARMNET".to_string()));
                }
                break (read_version(&element)?, true);
            }
            Event::GeneralRef(reference) => validate_reference(&reference)?,
            Event::Eof => return Err(missing_end_tag("FLARMNET").into()),
            _ => buffer.clear(),
        }
    };

    let mut records = Vec::new();
    if !empty {
        loop {
            buffer.clear();
            match reader.read_event_into(&mut buffer)? {
                Event::Start(element) => {
                    let element = element.into_owned();
                    if element.name().as_ref() == "FLARMDATA" {
                        records.push(read_record(&mut reader, &mut buffer, &element)?);
                    } else {
                        validated_attribute(&element, None)?;
                        skip_element(&mut reader, &mut buffer, element.to_end().into_owned())?;
                    }
                }
                Event::Empty(element) => {
                    if element.name().as_ref() == "FLARMDATA" {
                        records.push(convert(
                            validated_attribute(&element, Some("FlarmID"))?,
                            RecordFields::default(),
                        ));
                    } else {
                        validated_attribute(&element, None)?;
                    }
                }
                Event::End(element) if element.name().as_ref() == "FLARMNET" => break,
                Event::GeneralRef(reference) => validate_reference(&reference)?,
                Event::Eof => return Err(missing_end_tag("FLARMNET").into()),
                _ => {}
            }
        }
    }

    Ok(DecodedFile { version, records })
}

fn read_version(element: &BytesStart<'_>) -> Result<u32, DecodeError> {
    let version =
        validated_attribute(element, Some("Version"))?.ok_or(DecodeError::MissingVersion)?;
    u32::from_str_radix(&version, 16).map_err(|_| DecodeError::InvalidVersion(version))
}

fn read_record<R: BufRead>(
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    element: &BytesStart<'_>,
) -> Result<Result<Record, DecodeError>, DecodeError> {
    let flarm_id = validated_attribute(element, Some("FlarmID"))?;
    let mut fields = RecordFields::default();

    loop {
        buffer.clear();
        match reader.read_event_into(buffer)? {
            Event::Start(element) => {
                validated_attribute(&element, None)?;
                let element = element.into_owned();
                if let Some(field) = field(&mut fields, element.name().as_ref()) {
                    let value = read_text(reader, buffer, element.to_end().into_owned())?;
                    field.get_or_insert(value);
                } else {
                    skip_element(reader, buffer, element.to_end().into_owned())?;
                }
            }
            Event::Empty(element) => {
                validated_attribute(&element, None)?;
                if let Some(value) = field(&mut fields, element.name().as_ref()) {
                    value.get_or_insert_with(String::new);
                }
            }
            Event::End(element) if element.name().as_ref() == "FLARMDATA" => break,
            Event::GeneralRef(reference) => validate_reference(&reference)?,
            Event::Eof => return Err(missing_end_tag("FLARMDATA").into()),
            _ => {}
        }
    }

    Ok(convert(flarm_id, fields))
}

#[derive(Default)]
struct RecordFields {
    pilot_name: Option<String>,
    airfield: Option<String>,
    plane_type: Option<String>,
    registration: Option<String>,
    call_sign: Option<String>,
    frequency: Option<String>,
}

fn field<'a>(fields: &'a mut RecordFields, name: &str) -> Option<&'a mut Option<String>> {
    match name {
        "NAME" => Some(&mut fields.pilot_name),
        "AIRFIELD" => Some(&mut fields.airfield),
        "TYPE" => Some(&mut fields.plane_type),
        "REG" => Some(&mut fields.registration),
        "COMPID" => Some(&mut fields.call_sign),
        "FREQUENCY" => Some(&mut fields.frequency),
        _ => None,
    }
}

fn read_text<R: BufRead>(
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    end: BytesEnd<'static>,
) -> Result<String, quick_xml::Error> {
    let mut value = String::new();

    loop {
        buffer.clear();
        match reader.read_event_into(buffer)? {
            Event::Text(text) => value.push_str(&text.xml_content(XmlVersion::Explicit1_0)),
            Event::CData(text) => value.push_str(&text.xml_content(XmlVersion::Explicit1_0)),
            Event::GeneralRef(reference) => {
                if let Some(character) = reference.resolve_char_ref()? {
                    value.push(character);
                } else if let Some(entity) = resolve_xml_entity(reference.as_ref()) {
                    value.push_str(entity);
                } else {
                    return Err(EscapeError::UnrecognizedEntity(
                        0..reference.len(),
                        reference.to_string(),
                    )
                    .into());
                }
            }
            Event::Start(element) => {
                validated_attribute(&element, None)?;
                let element = element.into_owned();
                skip_element(reader, buffer, element.to_end().into_owned())?;
            }
            Event::Empty(element) => {
                validated_attribute(&element, None)?;
            }
            Event::End(element) if element.name() == end.name() => break,
            Event::Eof => return Err(missing_end_tag(end.name().as_ref())),
            _ => {}
        }
    }

    Ok(value)
}

fn skip_element<R: BufRead>(
    reader: &mut Reader<R>,
    buffer: &mut Vec<u8>,
    end: BytesEnd<'static>,
) -> Result<(), quick_xml::Error> {
    let mut depth = 0;
    loop {
        buffer.clear();
        match reader.read_event_into(buffer)? {
            Event::Start(element) => {
                validated_attribute(&element, None)?;
                depth += 1;
            }
            Event::Empty(element) => {
                validated_attribute(&element, None)?;
            }
            Event::End(element) if depth == 0 && element.name() == end.name() => break,
            Event::End(_) => depth -= 1,
            Event::GeneralRef(reference) => validate_reference(&reference)?,
            Event::Eof => return Err(missing_end_tag(end.name().as_ref())),
            _ => {}
        }
    }
    Ok(())
}

fn validated_attribute(
    element: &BytesStart<'_>,
    name: Option<&str>,
) -> Result<Option<String>, quick_xml::Error> {
    let mut value = None;
    for attribute in element.attributes() {
        let attribute = attribute?;
        let normalized = attribute.normalized_value(XmlVersion::Explicit1_0)?;
        if Some(attribute.key.as_ref()) == name {
            value = Some(normalized.into_owned());
        }
    }
    Ok(value)
}

fn validate_reference(reference: &BytesRef<'_>) -> Result<(), quick_xml::Error> {
    if reference.resolve_char_ref()?.is_none() && resolve_xml_entity(reference.as_ref()).is_none() {
        return Err(
            EscapeError::UnrecognizedEntity(0..reference.len(), reference.to_string()).into(),
        );
    }
    Ok(())
}

fn convert(flarm_id: Option<String>, fields: RecordFields) -> Result<Record, DecodeError> {
    let flarm_id = flarm_id.ok_or(DecodeError::MissingFlarmId)?;
    if u32::from_str_radix(&flarm_id, 16).is_err() {
        return Err(DecodeError::InvalidFlarmId(flarm_id));
    }

    Ok(Record {
        flarm_id,
        pilot_name: fields.pilot_name.unwrap_or_default(),
        airfield: fields.airfield.unwrap_or_default(),
        plane_type: fields.plane_type.unwrap_or_default(),
        registration: fields.registration.unwrap_or_default(),
        call_sign: fields.call_sign.unwrap_or_default(),
        frequency: fields.frequency.unwrap_or_default(),
    })
}

fn missing_end_tag(name: &str) -> quick_xml::Error {
    quick_xml::errors::IllFormedError::MissingEndTag(name.to_string()).into()
}

#[cfg(test)]
mod tests {
    use crate::Record;
    use crate::lx::cipher::Writer;
    use crate::lx::decode::{DecodeError, decode_file};
    use insta::assert_debug_snapshot;
    use std::io::copy;

    fn encrypt(mut s: &[u8]) -> Vec<u8> {
        let vec = Vec::with_capacity(s.len());
        let mut writer = Writer::new(vec);
        copy(&mut s, &mut writer).unwrap();
        writer.into_inner()
    }

    #[test]
    fn decoding_fails_for_empty_file() {
        let file = b"";
        assert_debug_snapshot!(decode_file(file).unwrap_err(), @r###"
        Xml(
            IllFormed(
                MissingEndTag(
                    "FLARMNET",
                ),
            ),
        )
        "###);
    }

    #[test]
    fn decoding_fails_for_invalid_file() {
        let file = b"foo";
        assert_debug_snapshot!(decode_file(file).unwrap_err(), @r###"
        Xml(
            IllFormed(
                MissingEndTag(
                    "FLARMNET",
                ),
            ),
        )
        "###);
    }

    #[test]
    fn decoding_fails_for_missing_root_element() {
        let file = encrypt(
            br#"<?xml version="1.0" encoding="UTF-8"?>
                <FOO>
                </FOO>"#,
        );
        assert_debug_snapshot!(decode_file(&file).unwrap_err(), @r###"
        MissingElement(
            "FLARMNET",
        )
        "###);
    }

    #[test]
    fn decoding_fails_for_missing_file_version() {
        let file = encrypt(
            br#"<?xml version="1.0" encoding="UTF-8"?>
                <FLARMNET>
                </FLARMNET>"#,
        );
        assert_debug_snapshot!(decode_file(&file).unwrap_err(), @"MissingVersion");
    }

    #[test]
    fn decoding_fails_for_invalid_file_version() {
        let file = encrypt(
            br#"<?xml version="1.0" encoding="UTF-8"?>
                <FLARMNET Version="foo">
                </FLARMNET>"#,
        );
        assert_debug_snapshot!(decode_file(&file).unwrap_err(), @r###"
        InvalidVersion(
            "foo",
        )
        "###);
    }

    #[test]
    fn decoding_preserves_supported_text_forms() {
        let file = encrypt(
            br#"<FLARMNET Version="012345">
                <FLARMDATA FlarmID="c0ffee">
                    <NAME>Jane &amp; John</NAME>
                    <AIRFIELD><![CDATA[ED&KA]]></AIRFIELD>
                    <TYPE>ASG &#x32;9</TYPE>
                    <REG/>
                    <FREQUENCY></FREQUENCY>
                    <IGNORED>ignored</IGNORED>
                    <WRAPPER><COMPID>nested</COMPID></WRAPPER>
                </FLARMDATA>
            </FLARMNET>"#,
        );

        let decoded = decode_file(&file).unwrap();
        let record = decoded.records.into_iter().next().unwrap().unwrap();

        assert_eq!(
            record,
            Record {
                flarm_id: "c0ffee".to_string(),
                pilot_name: "Jane & John".to_string(),
                airfield: "ED&KA".to_string(),
                plane_type: "ASG 29".to_string(),
                registration: String::new(),
                call_sign: String::new(),
                frequency: String::new(),
            }
        );
    }

    #[test]
    fn decoding_preserves_record_errors() {
        let file = encrypt(
            br#"<FLARMNET Version="012345">
                <FLARMDATA FlarmID="c0ffee"/>
                <FLARMDATA/>
                <FLARMDATA FlarmID="invalid"/>
                <FLARMDATA FlarmID="f00baa"/>
            </FLARMNET>"#,
        );

        let decoded = decode_file(&file).unwrap();

        let [
            Ok(first),
            Err(DecodeError::MissingFlarmId),
            Err(DecodeError::InvalidFlarmId(id)),
            Ok(last),
        ] = decoded.records.as_slice()
        else {
            panic!("unexpected records: {:?}", decoded.records);
        };
        assert_eq!(first.flarm_id, "c0ffee");
        assert_eq!(id, "invalid");
        assert_eq!(last.flarm_id, "f00baa");
    }

    #[test]
    fn decoding_rejects_malformed_xml() {
        let file = encrypt(
            br#"<FLARMNET Version="012345">
                <FLARMDATA FlarmID="c0ffee">
            </FLARMNET>"#,
        );

        assert!(matches!(decode_file(&file), Err(DecodeError::Xml(_))));
    }

    #[test]
    fn decoding_rejects_malformed_ignored_content() {
        for xml in [
            r#"<FLARMNET Version="012345"><IGNORED>&unknown;</IGNORED></FLARMNET>"#,
            r#"<FLARMNET Version="012345"><IGNORED value="&unknown;"/></FLARMNET>"#,
        ] {
            assert!(matches!(
                decode_file(&encrypt(xml.as_bytes())),
                Err(DecodeError::Xml(_))
            ));
        }
    }

    #[test]
    fn decoding_works() {
        let file = encrypt(
            br#"<FLARMNET Version="012345">
                <FLARMDATA FlarmID="c0ffee">
                    <NAME>John Doe</NAME>
                    <AIRFIELD>EDKA</AIRFIELD>
                    <TYPE>ASG 29</TYPE>
                    <REG>D-KESH</REG>
                    <COMPID>AS</COMPID>
                    <FREQUENCY>123.500</FREQUENCY>
                </FLARMDATA>
            </FLARMNET>"#,
        );

        let decoded = decode_file(&file).unwrap();
        assert_debug_snapshot!(decoded.records[0].as_ref().unwrap(), @r###"
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
}
