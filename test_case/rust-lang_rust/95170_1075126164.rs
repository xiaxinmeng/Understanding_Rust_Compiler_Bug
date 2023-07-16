
cargo +beta build --bins && cargo +beta run check
    Finished dev [unoptimized] target(s) in 0.09s
    Finished dev [unoptimized] target(s) in 0.07s
     Running `target/debug/bootstrap check`
thread 'main' panicked at '

couldn't find required command: "python"

', src/bootstrap/sanity.rs:59:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
