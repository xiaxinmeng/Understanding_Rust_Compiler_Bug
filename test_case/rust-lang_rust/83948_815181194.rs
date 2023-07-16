plain
.................................................................................................... 9400/11743
.................................................................................................... 9500/11743
.................................................................................i.......i.......... 9600/11743
.................................................................................................... 9700/11743
...........................iiiiiii..iiiiii.i........................................................ 9800/11743
.................................................................................................... 10000/11743
.................................................................................................... 10100/11743
.................................................................................................... 10200/11743
.................................................................................................... 10300/11743
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.108 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 12.002 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.37s

 finished in 2.446 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- src/builtin.rs - builtin::DEREF_NULLPTR (line 2976) stdout ----
error[E0745]: cannot take address of a temporary
 --> src/builtin.rs:2978:25
  |
4 |     core::ptr::addr_of!(std::ptr::null::<i32>())
  |                         ^^^^^^^^^^^^^^^^^^^^^^^ temporary value
error: aborting due to previous error

For more information about this error, try `rustc --explain E0745`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:45
