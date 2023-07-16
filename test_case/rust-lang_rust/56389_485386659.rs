console
> # d6f513ec7d1e83c8689f94fb48686dd11f1d1c80 is just before #58702
> echo '#![no_implicit_prelude] fn main() { ::std::assert_eq!(1, 1) }' | rustc +d6f513ec7d1e83c8689f94fb48686dd11f1d1c80 --edition 2018 -

> echo '#![no_implicit_prelude] fn main() { ::std::assert_eq!(1, 1) }' | rustc +d6f513ec7d1e83c8689f94fb48686dd11f1d1c80 --edition 2015 -

> # 5d20ff4d2718c820632b38c1e49d4de648a9810b is the merge of #58702
> echo '#![no_implicit_prelude] fn main() { ::std::assert_eq!(1, 1) }' | rustc +5d20ff4d2718c820632b38c1e49d4de648a9810b --edition 2015 -
error: cannot find macro `panic!` in this scope
 --> <anon>:1:37
  |
1 | #![no_implicit_prelude] fn main() { ::std::assert_eq!(1, 1) }
  |                                     ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: have you added the `#[macro_use]` on the module/import?
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to previous error

> echo '#![no_implicit_prelude] fn main() { ::std::assert_eq!(1, 1) }' | rustc +5d20ff4d2718c820632b38c1e49d4de648a9810b --edition 2018 -
error: cannot find macro `panic!` in this scope
 --> <anon>:1:37
  |
1 | #![no_implicit_prelude] fn main() { ::std::assert_eq!(1, 1) }
  |                                     ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: have you added the `#[macro_use]` on the module/import?
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to previous error
