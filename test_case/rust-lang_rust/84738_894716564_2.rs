
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 17 but the index is 17', compiler\rustc_metadata\src\creader.rs:146:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-dev running on x86_64-pc-windows-gnu

query stack during panic:
#0 [crate_name] fetching what a crate is named
end of query stack

------------------------------------------
