rust
use std::path::Path;

let path = Path::new("path/to/File Version 3.5.4 (Copy).tar.gz");
let mut parts = path.file_parts()

assert_eq!(parts.next(), Some("File Version 3"));
assert_eq!(parts.next(), Some(".5"));
assert_eq!(parts.next(), Some(".4 (Copy)"));
assert_eq!(parts.next(), Some(".tar"));
assert_eq!(parts.next(), Some(".gz"));
assert_eq!(parts.next(), None);
