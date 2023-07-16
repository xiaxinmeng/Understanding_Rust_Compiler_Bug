rust
  let lib = pkg_config::Config::new().cargo_metadata(false).probe("lua53").unwrap();

  let mut gcc = gcc::Config::new();
  gcc.file("src/c.c");
  for p in lib.include_paths {
    gcc.include(p);
  }
  gcc.compile("libc-sys.a");

  for l in lib.libs {
    println!("cargo:rustc-link-lib={}", l);
  }
