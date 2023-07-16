
error[E0425]: cannot find value `Family` in this scope
 --> reduced_mutant.rs:5:5
  |
5 |     Family as CollectionFamily
  |     ^^^^^^ not found in this scope

error[E0658]: generic associated types are unstable
 --> reduced_mutant.rs:2:5
  |
2 |     type Member<T>;
  |     ^^^^^^^^^^^^^^^
  |
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
  = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable

warning: trait objects without an explicit `dyn` are deprecated
 --> reduced_mutant.rs:5:15
  |
5 |     Family as CollectionFamily
  |               ^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn CollectionFamily`
  |
  = note: `#[warn(bare_trait_objects)]` on by default

error[E0601]: `main` function not found in crate `reduced_mutant`
 --> reduced_mutant.rs:1:1
  |
1 | / trait CollectionFamily {
2 | |     type Member<T>;
3 | | }
4 | | fn floatify() {
5 | |     Family as CollectionFamily
6 | | }
  | |_^ consider adding a `main` function to `reduced_mutant.rs`

error[E0191]: the value of the associated type `Member` (from trait `CollectionFamily`) must be specified
 --> reduced_mutant.rs:5:15
  |
2 |     type Member<T>;
  |     --------------- `Member` defined here
...
5 |     Family as CollectionFamily
  |               ^^^^^^^^^^^^^^^^ help: specify the associated type: `CollectionFamily<Member = Type>`

error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:529:17: type parameter `T/#1` (T/1) out of range when substituting, substs=[Self]

thread 'rustc' panicked at 'Box<Any>', /rustc/4f7612ac1499258025077f1fd05d2f429f9accfb/compiler/rustc_errors/src/lib.rs:888:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (4f7612ac1 2020-10-31) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [object_safety_violations] determine object safety of trait `CollectionFamily`
#1 [typeck] type-checking `floatify`
end of query stack
error: aborting due to 5 previous errors; 1 warning emitted

Some errors have detailed explanations: E0191, E0425, E0601, E0658.
For more information about an error, try `rustc --explain E0191`.
