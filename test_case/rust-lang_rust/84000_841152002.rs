plain
.................................................................................................... 9500/11866
.................................................................................................... 9600/11866
.....................................................................i......i....................... 9700/11866
.................................................................................................... 9800/11866
..............iiiiiii..iiiiii.i..................................................................... 9900/11866
.................................................................................................... 10100/11866
.................................................................................................... 10200/11866
.................................................................................................... 10300/11866
.................................................................................................... 10400/11866
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.12s

 finished in 0.193 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.69s

 finished in 3.768 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

running 1155 tests
i................................................................................................... 100/1155
.................................................................................................... 200/1155
............................iii..F...i......i...i.........i......................................... 300/1155
.............................................................................i....i................. 500/1155
...............................................i...............................ii................... 600/1155
.................................................................................................... 700/1155
.................................................................................................... 800/1155
---
---- src/ffi/c_str.rs - ffi::c_str::CStr::as_ptr (line 1267) stdout ----
error: unused import: `CStr`
 --> src/ffi/c_str.rs:1269:16
  |
5 | use std::ffi::{CStr, CString};
  |
note: the lint level is defined here
 --> src/ffi/c_str.rs:1265:9
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:50
