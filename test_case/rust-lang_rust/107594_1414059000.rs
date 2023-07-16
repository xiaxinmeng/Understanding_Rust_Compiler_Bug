rust
let p = PathBuf::from(r"meh\c:\foo");
let mut c = p.components();
c.next();
let subpath = c.as_path();
assert_ne!(subpath.is_absolute(), false); // BOOM
