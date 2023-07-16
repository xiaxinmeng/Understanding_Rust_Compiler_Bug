
error: internal compiler error: broken MIR in DefId(2:2112 ~ core[44b8]::ops[0]::function[0]::FnOnce[0]::call_once[0]) (after AddCallGuards in phase Const) at bb0[0]:
encountered non-callable type Self in `Call` terminator
   --> /home/nathan/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ops/function.rs:232:5
    |
232 |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
