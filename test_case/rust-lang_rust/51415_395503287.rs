
use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let junk: HashMap<String, Option<PathBuf>> = HashMap::new();
    let _ = match junk.iter().find(|(_, &path)| path.is_none()) {
        None => Ok(()),
        Some((ref name, _)) => Err(name.to_string()),
    };
}
