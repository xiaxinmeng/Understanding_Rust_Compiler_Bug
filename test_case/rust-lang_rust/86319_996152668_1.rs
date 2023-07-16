
use std::path::Path;

let path = Path::new("foo.tar.gz");
assert_eq!(path.file_prefix(), Some("foo"));
assert_eq!(path.file_suffix(), Some("tar.gz"));

assert_eq!(path.file_stem(), Some("foo.tar"));
assert_eq!(path.extension(), Some("gz"));
