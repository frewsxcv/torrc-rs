//! Internal parser types

use std::collections::HashMap;

pub type SettingsList = HashMap<String, Setting>;

/// The top-level `Torrc` type
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Torrc {
    settings: SettingsList
}

/// The `Setting` type
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Setting {
    /// Setting name
    pub name: String,
    /// Setting value
    pub value: String
}

impl Torrc {
    /// Create a new `Torrc` to hold the `SettingsList`
    pub fn new(s: SettingsList) -> Torrc {
        Torrc { settings: s }
    }

    pub fn settings(&self) -> &SettingsList {
        &self.settings
    }
}

impl Setting {
    /// Create a new `Setting`
    pub fn new(n: String, v: String) -> Setting {
        Setting { name: n, value: v }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}
