//! Torrc Reader types

use std::io::Read;
use std::path::Path;
use std::fs::File;

use types::Torrc;
use parser::parse;
use error::TorrcError;
use error::{from_io_err, from_parse_err};

pub fn from_stream<T: Read>(stream: &mut T) -> Result<Torrc, TorrcError> {
    let mut buf = String::new();

    match stream.read_to_string(&mut buf) {
        Ok(_) => from_str(&buf[..]),
        Err(e) => Err(from_io_err(e))
    }
}

pub fn from_file(path: &Path) -> Result<Torrc, TorrcError> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(from_io_err(e))
    };
    from_stream(&mut file)
}

pub fn from_str(input: &str) -> Result<Torrc, TorrcError> {
    parse(input).map_err(|e| from_parse_err(e))
}
