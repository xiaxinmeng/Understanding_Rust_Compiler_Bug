
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `1`', compiler/rustc_middle/src/mir/mod.rs:551:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `consts`
#1 [analysis] running analysis passes on this crate
end of query stack
