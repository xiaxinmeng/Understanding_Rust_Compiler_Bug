 rust
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;

fn main() {
    let mut f = OpenOptions::new().write(true).create(true).truncate(true).mode(0).open("/tmp/my_f").unwrap();
    f.write(b"test str");
    f.flush();
}
