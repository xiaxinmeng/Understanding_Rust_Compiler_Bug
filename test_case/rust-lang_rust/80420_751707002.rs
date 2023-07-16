plain
.................................................................................................... 9000/11195
.................................................................................................... 9100/11195
.....................................................................................i......i....... 9200/11195
.................................................................................................... 9300/11195
........................iiiiii..iiiiii.i............................................................ 9400/11195
.................................................................................................... 9600/11195
.................................................................................................... 9700/11195
.................................................................................................... 9800/11195
.................................................................................................... 9900/11195
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.062 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.394 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
running 5 tests
FFF..
failures:

---- symbols::tests::check_dup_symbol_and_keyword stdout ----
thread 'symbols::tests::check_dup_symbol_and_keyword' panicked at 'duplicate key `"splat"`', /cargo/registry/src/github.com-1ecc6299db9ec823/phf_codegen-0.8.0/src/lib.rs:203:17
---- symbols::tests::check_dup_keywords stdout ----
---- symbols::tests::check_dup_keywords stdout ----
thread 'symbols::tests::check_dup_keywords' panicked at 'duplicate key `"crate"`', /cargo/registry/src/github.com-1ecc6299db9ec823/phf_codegen-0.8.0/src/lib.rs:203:17
---- symbols::tests::check_dup_symbol stdout ----
---- symbols::tests::check_dup_symbol stdout ----
thread 'symbols::tests::check_dup_symbol' panicked at 'duplicate key `"splat"`', /cargo/registry/src/github.com-1ecc6299db9ec823/phf_codegen-0.8.0/src/lib.rs:203:17


failures:
    symbols::tests::check_dup_keywords
    symbols::tests::check_dup_keywords
    symbols::tests::check_dup_symbol
    symbols::tests::check_dup_symbol_and_keyword

test result: FAILED. 2 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.82s

error: test failed, to rerun pass '-p rustc_macros --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_macros" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:26:08
