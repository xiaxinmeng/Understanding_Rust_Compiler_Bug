
use std::io::prelude::*;
use std::fs::File;
fn main() {
    let mut f = try!(File::create("foo.txt"));
    try!(f.write_all(b"Hello, world!"));
    try!(f.sync_data());
}

   = note: expected type `()`
   = note:    found type `std::result::Result<_, _>`
   = note: this error originates in a macro outside of the current crate

error: aborting due to 3 previous errors
