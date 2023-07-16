plain
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 222 tests
....................................i...................................................  88/222
......................................................2023-04-29T15:47:34.094435Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/tests/rustdoc-ui/issues/auxiliary/panic-handler.rs` source not found"
F.................F............... 176/222

failures:

---- [ui] tests/rustdoc-ui/issues/issue-107918.rs stdout ----
---- [ui] tests/rustdoc-ui/issues/issue-107918.rs stdout ----

error: aux-build `/checkout/tests/rustdoc-ui/issues/auxiliary/panic-handler.rs` source not found
thread '[ui] tests/rustdoc-ui/issues/issue-107918.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2287:9

---- [ui] tests/rustdoc-ui/issues/issue-81662-shortness.rs stdout ----
diff of stdout:


1 
2 running 1 test
- test $DIR/issues/issue-81662-shortness.rs - foo (line 6) ... FAILED
+ test $DIR/issue-81662-shortness.rs - foo (line 6) ... FAILED
5 failures:
6 


- ---- $DIR/issues/issue-81662-shortness.rs - foo (line 6) stdout ----
- $DIR/issues/issue-81662-shortness.rs:7:1: error[E0425]: cannot find function `foo` in this scope
+ ---- $DIR/issue-81662-shortness.rs - foo (line 6) stdout ----
+ $DIR/issue-81662-shortness.rs:7:1: error[E0425]: cannot find function `foo` in this scope
10 Couldn't compile the test.
11 

12 failures:
12 failures:
-     $DIR/issues/issue-81662-shortness.rs - foo (line 6)
+     $DIR/issue-81662-shortness.rs - foo (line 6)
15 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
16 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issues/issue-81662-shortness/issue-81662-shortness.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-81662-shortness.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issues/issue-81662-shortness.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issues/issue-81662-shortness" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issues/issue-81662-shortness/auxiliary" "--test" "--error-format=short"
running 1 test
test /checkout/tests/rustdoc-ui/issues/issue-81662-shortness.rs - foo (line 6) ... FAILED

failures:
