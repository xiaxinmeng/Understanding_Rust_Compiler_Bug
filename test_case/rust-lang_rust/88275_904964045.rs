
$ RUSTFLAGS=-Cforce-unwind-tables RUST_BACKTRACE=1 cargo run --release 
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/testcase`
thread 'main' panicked at 'foo', src/main.rs:6:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
   1: std::panicking::begin_panic_fmt
             at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:457:5
   2: do_rust_panic
   3: foo
   4: testcase::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
