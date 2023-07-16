
let mut p2 = PathBuf::from("rustc-1.29.0-dev-src").with_extension("tar.gz");
p2.set_extension(""); // removes .gz
p2.set_extension(""); // removes .0-dev-src.tar !
