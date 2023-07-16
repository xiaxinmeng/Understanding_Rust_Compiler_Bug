
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::path::PathBuf;

lazy_static! {
    static ref BINARIES: HashMap<String, Option<PathBuf>> = [].iter().cloned().collect();
}

fn main() {
    let _ = match BINARIES.iter().find(|(_, &path)| path.is_none()) {
        None => Ok(()),
        Some((ref name, _)) => Err(name.to_string()),
    };
}
