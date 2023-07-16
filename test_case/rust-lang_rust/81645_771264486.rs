plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.069 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.687 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii......iiii....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.28s

 finished in 2.355 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 200/2831
..ii................................................................................................ 300/2831
...................................................i................................................ 400/2831
.................................................................................................... 500/2831
........................i..i...F..............iiii.................................................. 600/2831
.................................................................................................... 800/2831
.................................................................................................... 900/2831
.................................................................................................... 1000/2831
.................................................................................................... 1100/2831
---
---- src/lib.rs - panic (line 41) stdout ----
error: panic message is not a string literal
 --> src/lib.rs:45:8
  |
7 | panic!(4); // panic with the value of 4 to be collected elsewhere
  |
note: the lint level is defined here
 --> src/lib.rs:39:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(non_fmt_panic)]` implied by `#[deny(warnings)]`
  = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
  |
7 | panic!("{}", 4); // panic with the value of 4 to be collected elsewhere
  |        ^^^^^
help: or use std::panic::panic_any instead
  |
7 | std::panic::panic_any(4); // panic with the value of 4 to be collected elsewhere

error: aborting due to previous error

Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:47
