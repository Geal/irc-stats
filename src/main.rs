#[macro_use]
extern crate nom;

use std::path::Path;

mod import;
use import::parse::parse_file;

fn main() {
    // Path to the log file
    let path = Path::new("test_data/ipfs.log");

    // Parse the log file
    parse_file(path);
}

