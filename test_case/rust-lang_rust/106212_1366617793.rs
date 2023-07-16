plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9b889e53e78a186a861a8407c225f9d8e0d436f5 and a81054320b615da05fe0c3119fe339b51114f1c9
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

actual output differed from expected
--- tests/pass/box-custom-alloc.stderr
+++ <stderr output>
+error: Undefined Behavior: trying to retag from <TAG> for Unique permission at ALLOC[0x0], but that tag does not exist in the borrow stack for this location
+   |
+   |
+LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
+   | |
+   | |
+   | trying to retag from <TAG> for Unique permission at ALLOC[0x0], but that tag does not exist in the borrow stack for this location
+   | this error occurs as part of retag at ALLOC[0x0..0x1]
+   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
+   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
+   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
+help: <TAG> was created here, as the base tag for ALLOC
+   |
+   |
+LL |     let mut space = vec![MaybeUninit::new(0); 1];
+   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+help: <TAG> was later invalidated at offsets [0x0..0x1] by a Unique Box retag
+   |
+   |
+LL |     let boxed = Box::new_in([42u8; 1], &once_alloc);
+   = note: BACKTRACE (of the first span):
+   = note: BACKTRACE (of the first span):
+   = note: inside `std::ptr::drop_in_place::<[std::mem::MaybeUninit<u8>]> - shim(None)` at RUSTLIB/core/src/ptr/mod.rs:LL:CC
+   = note: inside `<std::vec::Vec<std::mem::MaybeUninit<u8>> as std::ops::Drop>::drop` at RUSTLIB/alloc/src/vec/mod.rs:LL:CC
+   = note: inside `std::ptr::drop_in_place::<std::vec::Vec<std::mem::MaybeUninit<u8>>> - shim(Some(std::vec::Vec<std::mem::MaybeUninit<u8>>))` at RUSTLIB/core/src/ptr/mod.rs:LL:CC
+  --> $DIR/box-custom-alloc.rs:LL:CC
+   |
+LL | }
+   | ^
---
+


There were 1 unmatched diagnostics that occurred outside the testfile and had no pattern
    Error: Undefined Behavior: trying to retag from <3064> for Unique permission at alloc1631[0x0], but that tag does not exist in the borrow stack for this location
full stderr:
full stderr:
error: Undefined Behavior: trying to retag from <3064> for Unique permission at alloc1631[0x0], but that tag does not exist in the borrow stack for this location
   |
   |
LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
   | |
   | |
   | trying to retag from <3064> for Unique permission at alloc1631[0x0], but that tag does not exist in the borrow stack for this location
   | this error occurs as part of retag at alloc1631[0x0..0x1]
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <3064> was created here, as the base tag for alloc1631
   |
   |
LL |     let mut space = vec![MaybeUninit::new(0); 1];
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: <3064> was later invalidated at offsets [0x0..0x1] by a Unique Box retag
   |
   |
LL |     let boxed = Box::new_in([42u8; 1], &once_alloc);
   = note: BACKTRACE (of the first span):
   = note: BACKTRACE (of the first span):
   = note: inside `std::ptr::drop_in_place::<[std::mem::MaybeUninit<u8>]> - shim(None)` at /checkout/library/core/src/ptr/mod.rs:490:1: 490:56
   = note: inside `<std::vec::Vec<std::mem::MaybeUninit<u8>> as std::ops::Drop>::drop` at /checkout/library/alloc/src/vec/mod.rs:3054:13: 3054:91
   = note: inside `std::ptr::drop_in_place::<std::vec::Vec<std::mem::MaybeUninit<u8>>> - shim(Some(std::vec::Vec<std::mem::MaybeUninit<u8>>))` at /checkout/library/core/src/ptr/mod.rs:490:1: 490:56
  --> tests/pass/box-custom-alloc.rs:56:1
   |
LL | }
   | ^
