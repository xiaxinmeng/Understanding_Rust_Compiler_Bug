rust
let mut exts = Path::new("foo.tar.gz").extensions().unwrap();
assert_eq!(exts.next(), Some(OsStr::new("gz")));
assert_eq!(exts.next(), Some(OsStr::new("tar")));
assert_eq!(exts.next(), None);
assert_eq!(exts.visited(), Some(OsStr::new("tar.gz"));
assert_eq!(exts.remaining(), "foo");
