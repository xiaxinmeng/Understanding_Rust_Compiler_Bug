
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: crate0
- dep-node: index_hir(core[e6b8])', /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:577:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=1 -C debug-assertions=on -C overflow-checks=off -C incremental -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [hir_owner] HIR owner of `macros`
#1 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `core`
