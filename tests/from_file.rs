extern crate torrc;

use std::path::Path;
use torrc::reader;

#[test]
fn torrc_from_file() {
    let res = reader::from_file(Path::new("tests/torrc.sample"));
    assert!(res.is_ok());

    let rc = res.unwrap();
    for (_, v) in rc.settings().iter() {
        for s in v.iter() {
            println!("k: {} v: {}", s.name(), s.value());
        }
    }
}

#[test]
fn torrc_lookup() {
    let res = reader::from_file(Path::new("tests/torrc.sample"));
    assert!(res.is_ok());

    let rc = res.unwrap();
    assert_eq!(rc.lookup(&"Log").unwrap().len(), 4);
}

