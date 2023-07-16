
$ ./configure --disable-optimize
...
$ export target=x86_64-unknown-linux-gnu
$ make RUSTFLAGS=--opt-level=1 $target/stage0/lib/rustlib/$target/lib/stamp.{std,native}
...
$ ./$target/stage0/bin/rustc foo.rs --opt-level=1
$ ./foo
task '<main>' failed at 'assertion failed: !contains_nul(path.container_as_bytes())', /home/alex/code/rust2/src/libstd/path/mod.rs:161                                                                                                  
