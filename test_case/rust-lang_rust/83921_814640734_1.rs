
warning: struct is never constructed: `Packed2NestedPair`
 --> perses_node_priority_with_dfs_delta_reduced_mutant.rs:2:8
  |
2 | struct Packed2NestedPair;
  |        ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

error: internal compiler error: unrecognized representation hint
 --> perses_node_priority_with_dfs_delta_reduced_mutant.rs:1:8
  |
1 | #[repr(packed())]
  |        ^^^^^^^^
  |
  = note: delayed at compiler/rustc_attr/src/builtin.rs:940:32

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1018:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (c051c5ddd 2021-04-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type staticlib

query stack during panic:
end of query stack
