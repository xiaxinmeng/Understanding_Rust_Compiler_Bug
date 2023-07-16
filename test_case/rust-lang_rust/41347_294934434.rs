rust
use std::fs::File;
use std::str;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {

use std::error::Error;
use std::path::Path;

let path = Path::new("/var/log/vmkernel.log");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`i

    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {} {:?}", display,
                                                   why.description(),
                                                   why.cause(),),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
