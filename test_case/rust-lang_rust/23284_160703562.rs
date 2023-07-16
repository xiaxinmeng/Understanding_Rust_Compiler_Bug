 rust
let path = Path::new("/might/be/something/a");
assert_eq!(path.relative_from(Path::new("/might/be")), Path::new("something/a"));
