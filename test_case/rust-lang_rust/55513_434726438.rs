
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let path = Path::new("herp.derp");
    
    if let Some(OsStr::new("derp")) = path.extension() {}
}
