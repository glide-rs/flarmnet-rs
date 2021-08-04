use crate::lx::cipher::encrypt;
use crate::File;
use minidom::quick_xml;
use minidom::quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use minidom::quick_xml::Writer;
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
}

/// Encodes a FlarmNet file in LX format.
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
/// let result = flarmnet::lx::encode_file(&file);
/// assert!(result.is_ok());
/// ```
pub fn encode_file(file: &File) -> Result<Vec<u8>, EncodeError> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), 9u8, 0);

    writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;

    let version = format!("{:06x?}", file.version);
    writer.write_event(Event::Start(
        BytesStart::borrowed_name(b"FLARMNET")
            .with_attributes(vec![("Version".as_bytes(), version.as_bytes())]),
    ))?;
    for record in &file.records {
        writer.write_event(Event::Start(
            BytesStart::borrowed_name(b"FLARMDATA")
                .with_attributes(vec![("FlarmID".as_bytes(), record.flarm_id.as_bytes())]),
        ))?;

        writer.write_event(Event::Text(BytesText::from_plain_str("\n\t")))?;
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"NAME")))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(&record.pilot_name)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"NAME")))?;

        writer.write_event(Event::Text(BytesText::from_plain_str("\n\t")))?;
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"AIRFIELD")))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(&record.airfield)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"AIRFIELD")))?;

        writer.write_event(Event::Text(BytesText::from_plain_str("\n\t")))?;
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"TYPE")))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(&record.plane_type)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"TYPE")))?;

        writer.write_event(Event::Text(BytesText::from_plain_str("\n\t")))?;
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"REG")))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(&record.registration)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"REG")))?;

        writer.write_event(Event::Text(BytesText::from_plain_str("\n\t")))?;
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"COMPID")))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(&record.call_sign)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"COMPID")))?;

        writer.write_event(Event::Text(BytesText::from_plain_str("\n\t")))?;
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"FREQUENCY")))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(&record.frequency)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"FREQUENCY")))?;

        writer.write_event(Event::End(BytesEnd::borrowed(b"FLARMDATA")))?;
    }
    writer.write_event(Event::End(BytesEnd::borrowed(b"FLARMNET")))?;

    let xml = writer.into_inner().into_inner();

    Ok(encrypt(&xml))
}
