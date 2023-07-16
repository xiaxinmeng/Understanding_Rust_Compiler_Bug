rust
use std::fs;
use std::os::linux::fs::MetadataExt; // <--------------- use this -------------

use std::io;
fn f() -> io::Result<()> {
    let meta = fs::metadata("/dev/null")?;
    println!("{}", meta.st_dev());
    Ok(())
}

fn main() {
    println!("{:?}", f());
}
