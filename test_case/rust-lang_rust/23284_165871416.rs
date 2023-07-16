 rust
let path = Path::new("/home/user/code");
let home = env::home_dir().unwrap();
path.relative_from(home);
path.strip_prefix(home);
path.trim(home);
