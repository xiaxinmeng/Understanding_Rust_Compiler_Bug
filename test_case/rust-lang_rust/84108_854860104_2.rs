
error: free static item without body
 --> mutant.rs:1:1
  |
1 | static FOO: (PartialEq<Item>, u32);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
  |                                   |
  |                                   help: provide a definition for the static: `= <expr>;`

error[E0412]: cannot find type `Item` in this scope
 --> mutant.rs:1:24
  |
1 | static FOO: (PartialEq<Item>, u32);
  |                        ^^^^ not found in this scope

warning: trait objects without an explicit `dyn` are deprecated
 --> mutant.rs:1:14
  |
1 | static FOO: (PartialEq<Item>, u32);
  |              ^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PartialEq<Item>`
  |
  = note: `#[warn(bare_trait_objects)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
  = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>

error[E0601]: `main` function not found in crate `mutant`
 --> mutant.rs:1:1
  |
1 | static FOO: (PartialEq<Item>, u32);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ consider adding a `main` function to `mutant.rs`

error: internal compiler error: compiler/rustc_middle/src/ty/layout.rs:370:17: univariant: field #2 of `(dyn PartialEq<[type error]>, u32)` comes after unsized field

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:1007:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (cc77ba46f 2021-06-03) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [layout_raw] computing layout of `(dyn std::cmp::PartialEq<[type error]>, u32)`
#1 [check_mod_item_types] checking item types in top-level module
end of query stack
error: aborting due to 4 previous errors; 1 warning emitted

Some errors have detailed explanations: E0412, E0601.
For more information about an error, try `rustc --explain E0412`.
