extern crate torrc;

use std::path::Path;
use torrc::reader;

#[test]
fn torrc_from_file() {
    let res = reader::from_file(Path::new("tests/torrc.sample"));
    assert!(res.is_ok());

    let rc = res.unwrap();
    println!("len: {}", rc.settings().len());
}

