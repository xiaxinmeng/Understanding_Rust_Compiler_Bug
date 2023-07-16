
$ cargo run --bin quill -- -svp stdlib/core run
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/quill -svp stdlib/core run`
Apr 19 13:56:30.151  INFO quill: initialised logging with verbosity level TRACE
   Compiling quill_parser v0.1.0 (/home/sky/code/quill/src/quill_parser)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(Fingerprint(17272853890222528742, 4579056754543839504))`,
 right: `Some(Fingerprint(16699371946333824810, 9762301140808967693))`: found unstable fingerprints for predicates_of(core[ec89]::iter::traits::collect::IntoIterator): GenericPredicates { parent: None, predicates: [(Binder(TraitPredicate(<Self as std::iter::IntoIterator>), []), /home/sky/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:202:1: 202:23 (#0))] }', /rustc/392ba2ba1a7d6c542d2459fb8133bebf62a4a423/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (392ba2ba1 2021-04-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [predicates_of] computing predicates of `std::iter::IntoIterator`
#1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
end of query stack
error: could not compile `quill_parser`

To learn more, run the command again with --verbose.
