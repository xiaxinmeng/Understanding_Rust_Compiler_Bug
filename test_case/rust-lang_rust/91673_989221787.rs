rust
let path = Path::new(r"C:\path").join("D:foo");
// path is "D:foo"
let absolute = absolute(path);
// absolute is like "D:\something\foo"
// actual value depends on the `D` drive's current directory.
