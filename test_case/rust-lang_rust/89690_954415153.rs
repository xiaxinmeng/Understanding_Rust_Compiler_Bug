
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing }, value: Binder(TraitPredicate(<C as B<{&_: &'static A}>>, polarity:Positive), []) } }
- dep-node: evaluate_obligation(b655c273fc552c69-c4da40ff6c91c2de)', /rustc/c390d69a615f095208ac94841f3310268521b2ee/compiler/rustc_query_system/src/dep_graph/graph.rs:236:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (c390d69a6 2021-10-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `C: B<{&_: &'static A}>`
#1 [typeck] type-checking `ice`
end of query stack
error: could not compile `rust`
