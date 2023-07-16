console
error[E0283]: type annotations needed
  --> src/librustc_session/filesearch.rs:93:42
   |
93 |         p.push(find_libdir(self.sysroot).as_ref());
   |
   |
   = note: cannot satisfy `std::borrow::Cow<'_, str>: std::convert::AsRef<_>`

error[E0283]: type annotations needed
   --> src/librustc_session/filesearch.rs:102:52
    |
    |
102 |     let mut p = PathBuf::from(find_libdir(sysroot).as_ref());
    |
    |
    = note: cannot satisfy `std::borrow::Cow<'_, str>: std::convert::AsRef<_>`
