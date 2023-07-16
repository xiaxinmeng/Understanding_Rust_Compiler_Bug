plain
.................................................................................................... 9000/11189
.................................................................................................... 9100/11189
................................................................................i......i............ 9200/11189
.................................................................................................... 9300/11189
...................iiiiii..iiiiii.i................................................................. 9400/11189
.................................................................................................... 9600/11189
.................................................................................................... 9700/11189
...................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
................................................. 9800/11189
---
 finished in 0.475 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.075 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i...i..ii...........iiii.........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.41s

 finished in 2.487 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
running 51 tests
......................F............................
failures:

---- doctest::tests::make_test_named_wrapper stdout ----
thread 'doctest::tests::make_test_named_wrapper' panicked at 'assertion failed: `(left == right)`
  left: `("#![allow(unused)]\nfn main() { #[allow(non_snake_case)] fn _doctest_main__some_unique_name() {\nassert_eq!(2+2, 4);\n}; _doctest_main__some_unique_name() }", 2)`,
 right: `("#![allow(unused)]\nfn main() { #[allow(non_snake_case)] fn _doctest_main__some_unique_name() {\nassert_eq!(2+2, 4);\n}; _doctest_main_some_unique_name() }", 2)`', src/librustdoc/doctest/tests.rs:313:5


failures:
    doctest::tests::make_test_named_wrapper
    doctest::tests::make_test_named_wrapper

test result: FAILED. 50 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustdoc --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:49
