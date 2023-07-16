
$ # This is what it would look like
$ rustc --crate-type=rlib --out-dir=target/ -o foobar test.rs --emit=[everything] && ls target/
foobar.o foobar.s foobar.ll foobar.bc libfoobar.rlib
