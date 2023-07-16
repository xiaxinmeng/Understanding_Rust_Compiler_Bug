
     Running `build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup`
thread 'main' panicked at 'failed to determine underlying rustc version of Miri: UnexpectedVersionFormat', src/tools/miri/cargo-miri/bin.rs:132:38
stack backtrace:
   0: rust_begin_unwind
             at ./library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at ./library/core/src/panicking.rs:92:14
   2: core::option::expect_none_failed
             at ./library/core/src/option.rs:1266:5
   3: core::result::Result<T,E>::expect
             at ./library/core/src/result.rs:929:23
   4: cargo_miri::version_info
             at ./src/tools/miri/cargo-miri/bin.rs:132:5
   5: cargo_miri::setup
             at ./src/tools/miri/cargo-miri/bin.rs:325:16
   6: cargo_miri::phase_cargo_miri
             at ./src/tools/miri/cargo-miri/bin.rs:418:5
   7: cargo_miri::main
             at ./src/tools/miri/cargo-miri/bin.rs:757:25
   8: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
