use flarmnet::decode_file;
use insta::assert_debug_snapshot;

#[test]
fn it_works() {
    let fixture = include_str!("fixtures/data.fln");
    assert_debug_snapshot!(decode_file(fixture));
}
