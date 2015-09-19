use std::path::Path;
use std::error::Error;
use std::str;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use nom::{not_line_ending};

pub fn parse_file(path: &Path) {
    // Open file
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file,
    };

    // Create our parser
    #[derive(Debug)]
    struct Line<'a> {
        timestamp: &'a str,
        username: &'a str,
        message: &'a str,
    }

    named!(parse_line<&[u8], Line>,
        chain!(
            timestamp: map_res!(
                take_until_and_consume!("\t"),
                str::from_utf8
            ) ~
            username: map_res!(
                take_until_and_consume!("\t"),
                str::from_utf8
            ) ~
            message: map_res!(
                not_line_ending,
                str::from_utf8
            ) ,

            || {
                Line {
                    timestamp: timestamp,
                    username: username,
                    message: message,
                }
            }
        )
    );

    // Iterate over lines
    let reader = BufReader::new(file);
    for l in reader.lines() {
        let line = match l {
            Err(why) => panic!("Couldn't get line: {}", Error::description(&why)),
            Ok(line) => line,
        };

        println!("{:?}", parse_line(line.as_bytes()));
    }
}
