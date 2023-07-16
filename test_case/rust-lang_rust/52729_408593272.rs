
let mut p1 = PathBuf::from(format!("{}.tar.gz", "rustc-1.29.0-dev-src"));
p1.set_extension(""); // removes .gz
p1.set_extension(""); // removes .tar
