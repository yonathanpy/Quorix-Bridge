use std::fs::{OpenOptions, File};
use std::io::{Write, Read};

pub fn save_ledger(data: &str) {
    let mut file = OpenOptions::new().append(true).create(true).open("ledger.db").unwrap();
    writeln!(file, "{}", data).unwrap();
}

pub fn read_ledger() -> String {
    let mut file = File::open("ledger.db").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
