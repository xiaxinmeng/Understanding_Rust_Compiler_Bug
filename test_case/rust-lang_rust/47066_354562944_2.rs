
$ rustc +nightly /dev/null -C debuginfo=1 -o d.dylib --crate-type proc-macro && md5 d.dylib
MD5 (d.dylib) = 210b2835a8ac8f9d86ba809f5b0c35d6
$ rustc +nightly /dev/null -C debuginfo=1 -o d.dylib --crate-type proc-macro && md5 d.dylib
MD5 (d.dylib) = 971fc21184955eac740d207e38a47b2f
