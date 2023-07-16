console
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/panicking_test-85130fa46b54f758

running 1 test
thread 'test test ... test' panicked at 'explicit panic', src/main.rs:5:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

fatal runtime error: failed to initiate panic, error 5

$ echo $?
0
