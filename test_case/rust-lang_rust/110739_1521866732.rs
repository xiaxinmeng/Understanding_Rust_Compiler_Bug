
╰ ➤ RUST_BACKTRACE=1 cargo +stage1 r > /dev/full
thread 'main' panicked at 'failed printing to stdout: No space left on device (os error 28)', library/std/src/io/stdio.rs:1019:9
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: std::io::stdio::_print
   3: scratch::main
             at ./src/main.rs:2:5
   4: core::ops::function::FnOnce::call_once
             at /home/ben/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
