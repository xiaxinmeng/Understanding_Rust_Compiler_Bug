`
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(regex[ea4b]::vector::avx2::{impl#0}::new)', /rustc/e2267046859c9ceb932abc983561d53a117089f6/compiler/rustc_query_system/src/query/plumbing.rs:566:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (e22670468 2020-12-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z incremental-verify-ich=yes -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `vector::avx2::AVX2VectorBuilder::new`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `regex`
