
   Compiling playground v0.0.1 (/playground)
warning: the feature `const_generics` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information
  = help: consider using `min_const_generics` instead, which is more stable and complete

warning: the feature `const_evaluatable_checked` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:2:12
  |
2 | #![feature(const_evaluatable_checked)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

error[E0599]: no method named `push` found for struct `SmallVec<_, { D * 2 }>` in the current scope
  --> src/lib.rs:23:19
   |
23 |         node.keys.push(k);
   |                   ^^^^ method not found in `SmallVec<_, { D * 2 }>`
...
29 | struct SmallVec<T, const D: usize> {
   | ---------------------------------- method `push` not found for this

error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0599`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
