use flarmnet::lx::decode_file;
use insta::assert_debug_snapshot;

#[test]
fn it_works() {
    let fixture = include_bytes!("fixtures/lx.fln");
    assert_debug_snapshot!(decode_file(fixture));
}
