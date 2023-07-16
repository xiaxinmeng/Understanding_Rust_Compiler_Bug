rust
  for l in lib.libs {
    println!('cargo:rustc-link-lib={}', l);
  }
