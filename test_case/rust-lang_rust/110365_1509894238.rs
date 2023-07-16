plain
   Compiling rustc-build-sysroot v0.4.1
    Finished release [optimized] target(s) in 2.81s
    Finished release [optimized] target(s) in 0.16s
     Running `obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup --target x86_64-unknown-linux-gnu`
thread 'main' panicked at 'failed to determine underlying rustc version of Miri: CouldNotExecuteCommand(Os { code: 2, kind: NotFound, message: "No such file or directory" })', src/tools/miri/cargo-miri/src/phases.rs:91:10
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   2: core::result::unwrap_failed
   3: cargo_miri::phases::phase_cargo_miri::<std::env::Args>
   4: cargo_miri::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
