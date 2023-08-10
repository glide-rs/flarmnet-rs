use flarmnet::lx::{decode_file, encode_file};
use insta::assert_snapshot;

#[test]
fn it_works() {
    let fixture = include_bytes!("fixtures/lx.fln");
    let decoded = decode_file(fixture).unwrap();
    let file = flarmnet::File {
        version: decoded.version,
        records: decoded
            .records
            .into_iter()
            .filter_map(|it| it.ok())
            .collect(),
    };
    let encoded = encode_file(&file).unwrap();
    assert_snapshot!(String::from_utf8_lossy(&encoded));
}
