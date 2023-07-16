
$ /builds/rustc-1.37.0/bin/rustc --target=x86_64-sun-solaris test.rs
$ /builds/rustc-1.37.0/bin/rustc --target=x86_64-pc-solaris test.rs
ulx-0 17:17 /builds/psumbera/FIREFOX-2/TMP: /builds/rustc-1.37.0/bin/rustc --target=x86_64-pc-solaris test.rs
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-pc-solaris` target may not be installed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
