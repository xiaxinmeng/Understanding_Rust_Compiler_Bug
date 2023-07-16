
➜  rust git:(russell/70677-raw-str-panic) ✗ rustc -g main.rs    
thread 'rustc' panicked at 'begin <= end (2 <= 1) when slicing `r"`', /rustc/76b11980ad416c3ad6143504c2277757ecacf9b5/src/libcore/str/mod.rs:1920:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (76b11980a 2020-04-01) running on x86_64-unknown-linux-gnu
