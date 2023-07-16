
; bat ./tt.rs --style="plain"
macro_rules! a { () => { A<'a,> }; }
; rustfmt +nightly-2022-01-18 ./tt.rs
thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1188:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
