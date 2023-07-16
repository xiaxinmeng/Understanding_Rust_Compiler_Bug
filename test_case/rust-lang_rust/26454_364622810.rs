rust
use std::fs;

use std::os::ext::fs::MetadataExt;

fn main() {
    fs::metadata("/usr").unwrap().mtime()
}
