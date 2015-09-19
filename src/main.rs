use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("test_data/ipfs.log");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Collect all lines into a vector
    let reader = BufReader::new(file);

    for l in reader.lines() {
        println!("{}", l.unwrap());
    }
}
