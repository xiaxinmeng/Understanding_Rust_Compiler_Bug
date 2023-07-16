 rust
let mut path = Path::new("/tmp/path");
println!("path: {}", path.display());
path.set_filename("foo");
path.push("bar");
println!("new path: {}", path.display());
println!("path exists: {}", path.exists());
