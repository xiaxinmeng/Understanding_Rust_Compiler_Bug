
thread 'rustc' panicked at 'index out of bounds: the len is 221944 but the index is 221944', /rustc/cb75ad5db02783e8b0222fee363c5f63f7e2cf5b\compiler\rustc_query_system\src\dep_graph\gr
aph.rs:477:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0 (cb75ad5db 2021-02-10) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

