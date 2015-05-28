//! The torrc parser module

//use std::str;
use std::str::from_utf8;
//use std::str::FromStr;

use types::{Setting,SettingsList,Torrc};

//use nom::{alphanumeric,eof,line_ending,multispace,not_line_ending};
use nom::{alphanumeric,eof,not_line_ending,space};
//use nom::{IResult};
use nom::IResult::*;

pub type ParseError = u32;

pub fn parse(input: &str) -> Result<Torrc, ParseError> {
    match torrc(&input.as_bytes()[..]) {
        Done(_, rc) => Ok(rc),
        _ => Err(0)
    }
}

named!(torrc<&[u8], Torrc>,
    chain!(
        rc: alt!(map_res!(settings_list,
                          |s| -> Result<Torrc, ()> { Ok(Torrc::new(s)) }) |
                 map_res!(blanks,
                          |_| -> Result<Torrc, ()> { Ok(Torrc::new(SettingsList::new())) })) ~
        eof,
        || { rc }));

named!(settings_list<&[u8], SettingsList>,
    map_res!(settings_list_parts,
             |s: Vec<Setting>| -> Result<SettingsList, ()> {
                 let mut res = SettingsList::new();
                 for setting in s.into_iter() {
                     res.insert(setting.name.clone(), setting);
                 }
                 Ok(res) }));

named!(settings_list_parts<&[u8], Vec<Setting> >, many1!(setting));

named!(setting<&[u8], Setting>,
    chain!(
        blanks? ~
        n: setting_name ~
        space ~
        v: setting_value ~
        blanks?,
        || { Setting::new(n, v) }));

named!(setting_name<&[u8], String>,
       chain!(
           h: map_res!(alphanumeric, from_utf8),
           || { h.to_string() }));

named!(setting_value<&[u8], String>,
       chain!(
           h: map_res!(take_until_either!("#\n"), from_utf8),
           || { h.to_string() }));

named!(blanks,
    chain!(
        many0!(alt!(comment | eol | eof)),
        || { &b""[..] }));

named!(comment,
    chain!(
        tag!("#") ~
        not_line_ending? ~
        alt!(eol | eof),
        || { &b""[..] }));

named!(eol,
    alt!(tag!("\r\n") | tag!("\n") | tag!("\u{2028}") | tag!("\u{2029}")));



/*

// the top-level torrc parser
// torrc -> settings_list
named!(torrc<&[u8], Torrc>,
    chain!(
        rc: alt!(map_res!(settings_list,
                          |s| -> Result<Torrc, ()> { Ok(Torrc::new(s)) }) |
                 map_res!(blanks,
                          |_| -> Result<Torrc, ()> { Ok(Torrc::new(SettingsList::new())) })) ~
        eof,
        || { rc }));

// settings list
named!(settings_list<&[u8], SettingsList>,
    map_res!(settings_list_parts,
             |s: Vec<Setting>| -> Result<SettingsList, ()> {
                 let mut res = SettingsList::new();
                 for setting in s.into_iter() {
                     res.insert(setting.name.clone(), setting);
                 }
                 Ok(res) }));

// settings list parts
named!(settings_list_parts<&[u8], Vec<Setting> >, many1!(setting));

// setting
named!(setting<&[u8], Setting>,
    chain!(
        multispace? ~
        name: setting_name ~
        multispace? ~
        tag!("=") ~
        multispace? ~
        v: setting_value ~
        blanks?,
        || { Setting::new(name, v) }));

// setting name
// name -> [a-zA-Z0-9]*
named!(setting_name<&[u8], String>,
    map_res!(alphanumeric, from_utf8));

// setting value
named!(setting_value<&[u8], String>,
    map_res!(take_until_either!("\n#"), from_utf8));


// parser for stuff to ignore
named!(blanks,
    chain!(
        many0!(alt!(comment | eol | eof)),
        || { &b""[..] }));

// comment parser
// comment -> #any.*$
named!(comment,
    chain!(
        tag!("#") ~
        not_line_ending? ~
        alt!(eol | eof),
        || { &b""[..] }));

// eol parser
named!(eol,
    alt!(tag!("\r\n") | tag!("\n") | tag!("\u{2028}") | tag!("\u{2029}")));
*/
