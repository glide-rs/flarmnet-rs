use flarmnet::lx::{decode_file, encode_file};

#[test]
fn it_works() {
    let fixture = include_str!("fixtures/lx.fln");
    let decoded = decode_file(fixture).unwrap();
    let file = flarmnet::File {
        version: decoded.version,
        records: decoded
            .records
            .into_iter()
            .filter_map(|it| it.ok())
            .collect(),
    };
    assert_eq!(encode_file(&file).unwrap(), fixture);
}
