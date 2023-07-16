shell
$ cargo run
   Compiling strings v0.1.0 (file:///Users/felipenoris/Documents/src/learnrust/strings)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/strings`
Segmentation fault: 11
$ cargo +77a8ed98fc66821a410b0c4696ba99b07b713467 run
   Compiling strings v0.1.0 (file:///Users/felipenoris/Documents/src/learnrust/strings)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/strings`
thread 'main' panicked at 'attempt to add with overflow', src/main.rs:5:20
note: Run with `RUST_BACKTRACE=1` for a backtrace.
