plain
.................................................................................................... 9400/11776
.................................................................................................... 9500/11776
.................................................................................................... 9600/11776
....i......i........................................................................................ 9700/11776
..................................................iiiiiii..iiiiii.i................................. 9800/11776
.................................................................................................... 10000/11776
.................................................................................................... 10100/11776
.................................................................................................... 10200/11776
.................................................................................................... 10300/11776
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.167 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.57s

 finished in 2.644 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"library/std"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 2.785 seconds
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (/checkout/library/std)
error: cannot link together two panic runtimes: panic_unwind and panic_abort

error: the linked panic runtime `panic_abort` is not compiled with this crate's panic strategy `unwind`

error: the crate `panic_abort` is compiled with the panic strategy `abort` which is incompatible with this crate's strategy of `unwind`

error: cannot link together two panic runtimes: panic_unwind and panic_abort

error: the linked panic runtime `panic_abort` is not compiled with this crate's panic strategy `unwind`

error: the crate `panic_abort` is compiled with the panic strategy `abort` which is incompatible with this crate's strategy of `unwind`
error: aborting due to 3 previous errors

error: could not compile `std`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: aborting due to 3 previous errors

error: cannot link together two panic runtimes: panic_unwind and panic_abort

error: the linked panic runtime `panic_abort` is not compiled with this crate's panic strategy `unwind`

error: the crate `panic_abort` is compiled with the panic strategy `abort` which is incompatible with this crate's strategy of `unwind`
error: aborting due to 3 previous errors


error: cannot link together two panic runtimes: panic_unwind and panic_abort

error: the linked panic runtime `panic_abort` is not compiled with this crate's panic strategy `unwind`

error: the crate `panic_abort` is compiled with the panic strategy `abort` which is incompatible with this crate's strategy of `unwind`
error: aborting due to 3 previous errors


error: cannot link together two panic runtimes: panic_unwind and panic_abort

error: the linked panic runtime `panic_abort` is not compiled with this crate's panic strategy `unwind`

error: the crate `panic_abort` is compiled with the panic strategy `abort` which is incompatible with this crate's strategy of `unwind`
error: aborting due to 3 previous errors

error: build failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:01
