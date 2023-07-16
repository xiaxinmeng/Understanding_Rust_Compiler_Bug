

$ rustc --target=x86_64-pc-solaris test.rs
error[E0461]: couldn't find crate `std` with expected target triple x86_64-pc-solaris
  |
  = note: the following crate versions were found:
          crate `std`, target triple x86_64-sun-solaris: /usr/lib/rustlib/x86_64-sun-solaris/lib/libstd-560fc60f1e090eba.rlib
          crate `std`, target triple x86_64-sun-solaris: /usr/lib/rustlib/x86_64-sun-solaris/lib/libstd-560fc60f1e090eba.so

error: aborting due to previous error
