
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 21', /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.0/src/snapshot_vec.rs:199:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (7100b311d 2021-07-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options --crate-type lib

query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `Server<[closure@main.rs:14:46: 14:85]>`
#1 [typeck] type-checking `http`
end of query stack
