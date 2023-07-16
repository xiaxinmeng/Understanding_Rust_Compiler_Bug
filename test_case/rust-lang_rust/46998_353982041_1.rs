
error: linking with `cc` failed: exit code: 1
 --> deflate/build.rs:2:5
  |
2 |     println!("cargo:rustc-link-lib=asdfasdf");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
= note: /usr/bin/ld: cannot find library "asdfasdf"
= note: searched for library in the following paths:
     /usr/bin/
     /usr/local/bin
     /usr/sbin
= note: collect2: error: ld returned 1 exit status
