
$ ./bin/rustdoc -V && RUST_BACKTRACE=1 ./bin/rustdoc ../test.rs 
rustdoc 1.10.0-dev (fa5597900 2016-05-17)
../test.rs:7:10: 7:14 error: cannot move out of borrowed content [E0507]
../test.rs:7     Some(self.0)
                      ^~~~
error: Compilation failed, aborting rustdoc
