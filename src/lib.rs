#![crate_name = "torrc"]
#![crate_type = "lib"]

#[macro_use]
extern crate nom;

mod parser;

pub mod types;
pub mod error;
pub mod reader;
