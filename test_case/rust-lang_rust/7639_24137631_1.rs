
$ make
rustc  --test test/runner.rs -o run-tests
test/runner.rs:16:2: 16:12 error: unresolved name `Graph::new`.
test/runner.rs:16   Graph::new();
                    ^~~~~~~~~~
error: aborting due to previous error
make: *** [run-tests] Error 101
