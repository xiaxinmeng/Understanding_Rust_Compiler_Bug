plain
.................................................................................................... 9400/11760
.................................................................................................... 9500/11760
..............................................................................................i..... 9600/11760
.i.................................................................................................. 9700/11760
........................................iiiiiii..iiiiii.i........................................... 9800/11760
.................................................................................................... 10000/11760
.................................................................................................... 10100/11760
.................................................................................................... 10200/11760
.................................................................................................... 10300/11760
---
 finished in 0.428 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.183 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.373 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
 finished in 45.289 seconds
Set({"library/core"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/checkout/library/core)
error: use of deprecated struct `std::raw::TraitObject`: use pointer metadata APIs instead https://github.com/rust-lang/rust/issues/81513
    |
    |
109 |         let x: ::core::raw::TraitObject = transmute(a);
    |
    |
    = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated field `std::raw::TraitObject::data`: use pointer metadata APIs instead https://github.com/rust-lang/rust/issues/81513
    |
    |
110 |         assert!(*(x.data as *const isize) == 100);

error: aborting due to 2 previous errors

error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:01
