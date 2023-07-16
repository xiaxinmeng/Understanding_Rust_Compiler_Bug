plain
.................................................................................................... 9300/11530
.................................................................................................... 9400/11530
...............................................................................i......i............. 9500/11530
.................................................................................................... 9600/11530
..................iiiiiii..iiiiii.i................................................................. 9700/11530
.................................................................................................... 9900/11530
.................................................................................................... 10000/11530
.................................................................................................... 10100/11530
.................................................................................................... 10200/11530
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.060 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.219 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.26s

 finished in 2.326 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 500/557
..........................................FFF............
failures:

---- src/vec/mod.rs - vec::[T; N]::try_from (line 2705) stdout ----
error: the item `TryInto` is imported redundantly
   |
4  | use std::convert::TryInto;
   |     ^^^^^^^^^^^^^^^^^^^^^
   | 
   | 
  ::: /checkout/library/std/src/prelude/v1.rs:26:35
   |
26 | pub use crate::convert::{TryFrom, TryInto};
   |                                   ------- the item `TryInto` is already defined here
note: the lint level is defined here
  --> src/vec/mod.rs:2704:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error: aborting due to previous error

Couldn't compile the test.
---- src/vec/mod.rs - vec::[T; N]::try_from (line 2713) stdout ----
error: the item `TryInto` is imported redundantly
   |
4  | use std::convert::TryInto;
   |     ^^^^^^^^^^^^^^^^^^^^^
   | 
   | 
  ::: /checkout/library/std/src/prelude/v1.rs:26:35
   |
26 | pub use crate::convert::{TryFrom, TryInto};
   |                                   ------- the item `TryInto` is already defined here
note: the lint level is defined here
  --> src/vec/mod.rs:2712:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error: aborting due to previous error

Couldn't compile the test.
---- src/vec/mod.rs - vec::[T; N]::try_from (line 2698) stdout ----
error: the item `TryInto` is imported redundantly
   |
4  | use std::convert::TryInto;
   |     ^^^^^^^^^^^^^^^^^^^^^
   | 
   | 
  ::: /checkout/library/std/src/prelude/v1.rs:26:35
   |
26 | pub use crate::convert::{TryFrom, TryInto};
   |                                   ------- the item `TryInto` is already defined here
note: the lint level is defined here
  --> src/vec/mod.rs:2697:9
   |
2  | #![deny(warnings)]
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:42
