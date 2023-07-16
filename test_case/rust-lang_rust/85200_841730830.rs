plain
.................................................................................................... 9500/11870
.................................................................................................... 9600/11870
.........................................................................i......i................... 9700/11870
.................................................................................................... 9800/11870
..................iiiiiii..iiiiii.i................................................................. 9900/11870
.................................................................................................... 10100/11870
.................................................................................................... 10200/11870
.................................................................................................... 10300/11870
.................................................................................................... 10400/11870
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.186 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.81s

 finished in 3.878 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
 finished in 3.169 seconds
Set({"library/std"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (/checkout/library/std)
error: field is never read: `flushed`
    |
472 |     pub flushed: bool,
    |     ^^^^^^^^^^^^^^^^^
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `std`


To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:20
