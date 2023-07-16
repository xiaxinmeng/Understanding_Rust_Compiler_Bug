 rust
use std::io::fs::File;

#[no_mangle]
pub fn init()
{
    let _ = File::open(&Path::new("/dev/urandom")).unwrap();
}
