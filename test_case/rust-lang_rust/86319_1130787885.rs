rust
use std::path::Path;

let path = Path::new("path/to/File Version 3.5.4 (Copy).tar.gz");
let mut exts = path.suffixes()

assert_eq!(path.file_prefix(), Some("File Version 3"));
assert_eq!(exts.next(), Some(".5"));
assert_eq!(exts.next(), Some(".4 (Copy)"));
assert_eq!(exts.next(), Some(".tar"));
assert_eq!(exts.next(), Some(".gz"));
assert_eq!(exts.next(), None);
