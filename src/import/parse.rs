use std::path::Path;
use std::error::Error;
use std::str;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use nom::{not_line_ending};
use nom::IResult::{Done};

// A log line
#[derive(Debug)]
enum Line<'a> {
    UserMessage {
        timestamp: &'a str,
        username: &'a str,
        message: &'a str,
    },
    NetworkMessage {
        timestamp: &'a str,
        message: &'a str,
    },
    Join {
        timestamp: &'a str,
        username: &'a str,
        host: &'a str,
    },
    Leave {
        timestamp: &'a str,
        username: &'a str,
        host: &'a str,
        reason: &'a str,
    },
}

// Create our parser

// Parse a timestamp
named!(timestamp<&[u8], &str>,
    map_res!(
        take_until_and_consume!("\t"),
        str::from_utf8
    )
);

// Parse a line
named!(parse_line<&[u8], Line>,
    alt!(
        // Join
        chain!(
            timestamp: timestamp ~
            tag!("-->\t") ~
            username: map_res!(
                take_until_and_consume!(" "),
                str::from_utf8
            ) ~
            tag!("(") ~
            host: map_res!(
                take_until_and_consume!(")"),
                str::from_utf8
            ) ,

            || {
                Line::Join {
                    timestamp: timestamp,
                    username: username,
                    host: host,
                }
            }
        ) |

        // Leave
        chain!(
            timestamp: timestamp ~
            tag!("<--\t") ~
            username: map_res!(
                take_until_and_consume!(" "),
                str::from_utf8
            ) ~
            tag!("(") ~
            host: map_res!(
                take_until_and_consume!(")"),
                str::from_utf8
            ) ~
            take_until_and_consume!("(") ~
            reason: map_res!(
                take_until_and_consume!(")"),
                str::from_utf8
            ) ,

            || {
                Line::Leave {
                    timestamp: timestamp,
                    username: username,
                    host: host,
                    reason: reason,
                }
            }
        ) |

        // NetworkMessage
        chain!(
            timestamp: timestamp ~
            alt!(
                tag!("\u{2139} ") |
                tag!("--")   
            ) ~
            tag!("\t") ~
            message: map_res!(
                not_line_ending,
                str::from_utf8
            ) ,

            || {
                Line::NetworkMessage {
                    timestamp: timestamp,
                    message: message,
                }
            }
        ) |

        // UserMessage
        chain!(
            timestamp: timestamp ~
            username: map_res!(
                take_until_and_consume!("\t"),
                str::from_utf8
            ) ~
            message: map_res!(
                not_line_ending,
                str::from_utf8
            ) ,

            || {
                Line::UserMessage {
                    timestamp: timestamp,
                    username: username,
                    message: message,
                }
            }
        )
    )
);

// Parse a log file
pub fn parse_file(path: &Path) {
    // Open file
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file,
    };

    // Iterate over lines
    let reader = BufReader::new(file);
    for l in reader.lines() {
        let raw_line = match l {
            Err(why) => panic!("Couldn't get line: {}", Error::description(&why)),
            Ok(line) => line,
        };

        let line = parse_line(raw_line.as_bytes());
        match line {
            Done(_, parsed_line) => {
                println!("{:?}", parsed_line);
            }
            _ => println!("Can't parse line: {}", raw_line)
        }
    }
}
