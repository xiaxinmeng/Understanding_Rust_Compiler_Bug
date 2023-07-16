
   Compiling brotli v2.1.0 (file:///$DIR/brotli)
error: couldn't read src/bin/testdata/random_then_unicode: No such file or directory (os error 2)
   --> src/bin/integration_tests.rs:335:46
    |
335 | static RANDOM_THEN_UNICODE : &'static [u8] = include_bytes!("testdata/random_then_unicode");
    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
