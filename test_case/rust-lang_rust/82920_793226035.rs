
thread 'rustc' panicked at 'found unstable fingerprints for super_predicates_that_define_assoc_type(3b141dfec2cd95a0-b29c689b3e8b2251)', /rustc/234781afe33d3f339b002f85f948046d8476cfc9/compiler/rustc_query_system/src/query/plumbing.rs:586:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (234781afe 2021-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z incremental-verify-ich=yes -C embed-bitcode=no -C debuginfo=0 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [super_predicates_that_define_assoc_type] computing the super traits of `db::HirDatabase`
#1 [super_predicates_of] computing the super predicates of `db::HirDatabase`
end of query stack
