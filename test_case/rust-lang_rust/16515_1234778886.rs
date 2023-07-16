
error: Undefined Behavior: trying to retag from <3260> for Unique permission at alloc1702[0x0], but that tag does not exist in the borrow stack for this location
  --> test.rs:6:16
   |
6  | pub fn use_foo(foo: Foo) {
   |                ^^^
   |                |
   |                trying to retag from <3260> for Unique permission at alloc1702[0x0], but that tag does not exist in the borrow stack for this location
   |                this error occurs as part of FnEntry retag at alloc1702[0x0..0x4]
   |
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <3260> was created by a Unique retag at offsets [0x0..0x4]
  --> test.rs:17:17
   |
17 |         use_foo(std::mem::transmute(pair));
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^
help: <3260> was later invalidated at offsets [0x0..0x4] by a Unique retag
  --> test.rs:17:17
   |
17 |         use_foo(std::mem::transmute(pair));
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: BACKTRACE:
   = note: inside `use_foo` at test.rs:6:16
note: inside `main` at test.rs:17:9
  --> test.rs:17:9
   |
17 |         use_foo(std::mem::transmute(pair));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
