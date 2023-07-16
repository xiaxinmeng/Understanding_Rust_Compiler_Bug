
     Running `build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup`
thread 'main' panicked at 'failed to determine underlying rustc version of Miri: CommandError { stdout: "", stderr: "/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-tools-bin/miri: error while loading shared libraries: libLLVM-12-rust-1.54.0-nightly.so: cannot open shared object file: No such file or directory\n" }', src/tools/miri/cargo-miri/bin.rs:190:38
stack backtrace:
   0: rust_begin_unwind
             at ./library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at ./library/core/src/panicking.rs:92:14
   2: core::result::unwrap_failed
             at ./library/core/src/result.rs:1355:5
   3: core::result::Result<T,E>::expect
             at ./library/core/src/result.rs:997:23
   4: cargo_miri::version_info
             at ./src/tools/miri/cargo-miri/bin.rs:190:5
   5: cargo_miri::setup
             at ./src/tools/miri/cargo-miri/bin.rs:399:16
   6: cargo_miri::phase_cargo_miri
             at ./src/tools/miri/cargo-miri/bin.rs:477:5
   7: cargo_miri::main
             at ./src/tools/miri/cargo-miri/bin.rs:965:25
   8: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.


command did not execute successfully: "/home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--manifest-path" "/home/r/src/rust/rustc.2/src/tools/miri/cargo-miri/Cargo.toml" "--" "miri" "setup"
expected success, got: exit code: 101
