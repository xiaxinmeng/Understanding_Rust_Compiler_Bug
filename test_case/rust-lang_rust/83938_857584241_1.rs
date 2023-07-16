
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:577:17: const parameter `M/#1` (Const { ty: usize, val: Param(M/#1) }/1) out of range when substituting substs=[Const { ty: usize, val: Param(N1/#0) }]
 --> src/main.rs:5:34
  |
5 | pub fn foo<const N1: usize>() -> Bar<N1> { loop {} }
  |                                  ^^^^^^^

thread 'rustc' panicked at 'Box<dyn Any>', /media/ellen-nyan/Nyoomies/rust/compiler/rustc_errors/src/lib.rs:953:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [fn_sig] computing function signature of `foo`
#1 [collect_mod_item_types] collecting item types in top-level module
end of query stack
error: aborting due to previous error

error: could not compile `playground-cgd-subst-ice`
