plain
.................................................................................................... 9000/11185
.................................................................................................... 9100/11185
............................................................................i......i................ 9200/11185
.................................................................................................... 9300/11185
...............iiiiii..iiiiii.i..................................................................... 9400/11185
.................................................................................................... 9600/11185
.................................................................................................... 9700/11185
.................................................................................................... 9800/11185
.................................................................................................... 9900/11185
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.066 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i...ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.06s

 finished in 2.123 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- src/io/mod.rs - io::read_to_string (line 957) stdout ----
error[E0433]: failed to resolve: use of undeclared crate or module `io`
 --> src/io/mod.rs:959:17
  |
4 |     let stdin = io::read_to_string(io::stdin());
  |                 ^^ use of undeclared crate or module `io`
error[E0433]: failed to resolve: use of undeclared crate or module `io`
 --> src/io/mod.rs:959:36
  |
  |
4 |     let stdin = io::read_to_string(io::stdin());
  |                                    ^^ use of undeclared crate or module `io`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:12
