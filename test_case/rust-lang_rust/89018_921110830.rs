
thread 'rustc' panicked at 'Failed to recover key for crate_hash(7b325c0bcf1c8f7e-78e92a3d885642c8) with hash 7b325c0bcf1c8f7e-78e92a3d885642c8', compiler/rustc_query_impl/src/lib.rs:54:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0 (c8dfcfe04 2021-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `storing_lists_vectors`
