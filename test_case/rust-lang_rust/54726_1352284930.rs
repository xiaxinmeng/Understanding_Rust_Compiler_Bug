
   Compiling test-exe v0.1.0 (/src/qix-/repro-no_std-custom-inner-attr/test-exe)
error[E0658]: `#[prelude_import]` is for use by rustc only
 --> test-exe/src/main.rs:1:1
  |
1 | / #![feature(custom_inner_attributes)]
2 | | #![::custom_attr::add_no_std]
3 | |
4 | | fn main() {}
  | |____________^
  |
  = help: add `#![feature(prelude_import)]` to the crate attributes to enable

warning: unused import: `#![feature(custom_inner_attributes)]
         #![::custom_attr::add_no_std]

         fn main() {}`
 --> test-exe/src/main.rs:1:1
  |
1 | / #![feature(custom_inner_attributes)]
2 | | #![::custom_attr::add_no_std]
3 | |
4 | | fn main() {}
  | |____________^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused `#[macro_use]` import
 --> test-exe/src/main.rs:1:1
  |
1 | / #![feature(custom_inner_attributes)]
2 | | #![::custom_attr::add_no_std]
3 | |
4 | | fn main() {}
  | |____________^

For more information about this error, try `rustc --explain E0658`.
warning: `test-exe` (bin "test-exe") generated 2 warnings
error: could not compile `test-exe` due to previous error; 2 warnings emitted
