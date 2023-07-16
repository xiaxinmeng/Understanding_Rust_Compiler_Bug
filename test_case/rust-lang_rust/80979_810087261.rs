plain
.................................................................................................... 9400/11720
.................................................................................................... 9500/11720
..............................................................i......i.............................. 9600/11720
.................................................................................................... 9700/11720
........iiiiiii...i.iiiiii.......................................................................... 9800/11720
.................................................................................................... 10000/11720
.................................................................................................... 10100/11720
.................................................................................................... 10200/11720
.................................................................................................... 10300/11720
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.104 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
 finished in 2.366 seconds
---
---- src/path.rs - path::Path::exists (line 2485) stdout ----
error[E0308]: mismatched types
  --> src/path.rs:2492:24
   |
10 |     Err(err) => return Err(err),
   |                        ^^^^^^^^ expected `()`, found enum `Result`
   = note: expected unit type `()`
                   found enum `Result<_, std::io::Error>`

error: aborting due to previous error
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:51
