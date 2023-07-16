plain
.................................................................................................... 9400/11823
.................................................................................................... 9500/11823
.................................................................................................... 9600/11823
...........................................i......i................................................. 9700/11823
.........................................................................................iiiiiii..ii 9800/11823
.................................................................................................... 10000/11823
.................................................................................................... 10100/11823
.................................................................................................... 10200/11823
.................................................................................................... 10300/11823
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.12s

 finished in 0.183 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.983 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.452 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 26.25s
     Running unittests (build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-873a6d222f95b19d)

running 63 tests
....................F...........F..............................

---- html::highlight::tests::test_dos_backline stdout ----
---- html::highlight::tests::test_dos_backline stdout ----
thread 'html::highlight::tests::test_dos_backline' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9

---- html::highlight::tests::test_html_highlighting stdout ----
---- html::highlight::tests::test_html_highlighting stdout ----
thread 'html::highlight::tests::test_html_highlighting' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9

failures:
    html::highlight::tests::test_dos_backline
    html::highlight::tests::test_html_highlighting
    html::highlight::tests::test_html_highlighting

test result: FAILED. 61 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustdoc --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:25:43
