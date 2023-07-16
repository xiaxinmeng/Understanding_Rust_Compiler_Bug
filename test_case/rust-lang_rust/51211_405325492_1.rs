
warning: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
  --> foo.rs:7:17
   |
7  |             use myself::f;
   |                 ^^^^^^^^^ help: use `crate`: `crate::myself::f`
...
18 |     foo!();
   |     ------- in this macro invocation
   |
note: lint level defined here
  --> foo.rs:2:9
   |
2  | #![warn(rust_2018_compatibility)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^
   = note: #[warn(absolute_paths_not_starting_with_crate)] implied by #[warn(rust_2018_compatibility)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue TBD

warning: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
  --> foo.rs:7:17
   |
7  |             use myself::f;
   |                 ^^^^^^^^^ help: use `crate`: `crate::myself::f`
...
19 |     foo!();
   |     ------- in this macro invocation
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue TBD
