rust
let home = PathBuf::from("/home");
let user = Path::new("user");
assert_eq!(Path::new("/home/user/documents"), home / user / "documents");
