plain
 - Checking "/checkout/src/librustdoc/html/static/css/themes/dark.css"... OK
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 222 tests
............................FFFFFFFFiFF.FF.F...........FFF.F.FF.F.F.....................  88/222
......................................................2023-04-29T10:37:05.263283Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/tests/rustdoc-ui/issues/auxiliary/panic-handler.rs` source not found"
F.............F.F........F........ 176/222

failures:

---- [ui] tests/rustdoc-ui/doctest/check-cfg-test.rs stdout ----
---- [ui] tests/rustdoc-ui/doctest/check-cfg-test.rs stdout ----
diff of stdout:

1 
2 running 1 test
- test $DIR/doctest/check-cfg-test.rs - Foo (line 8) ... ok
+ test $DIR/check-cfg-test.rs - Foo (line 8) ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 


---
To only update this specific test, also pass `--test-args doctest/check-cfg-test.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/check-cfg-test.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/check-cfg-test" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/check-cfg-test/auxiliary" "--test" "--nocapture" "--check-cfg=values(feature,\"test\")" "-Z" "unstable-options"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/check-cfg-test.rs - Foo (line 8) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
---
warning: 1 warning emitted
------------------------------------------


---- [ui] tests/rustdoc-ui/doctest/display-output.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/display-output.rs - foo (line 9) ... ok
+ test $DIR/display-output.rs - foo (line 9) ... ok
5 successes:
6 


- ---- $DIR/doctest/display-output.rs - foo (line 9) stdout ----
+ ---- $DIR/display-output.rs - foo (line 9) stdout ----
8 warning: unused variable: `x`
-   --> $DIR/doctest/display-output.rs:11:5
10    |
11 LL | let x = 12;
12    |     ^ help: if this is intentional, prefix it with an underscore: `_x`

---
21 warning: unused variable: `x`
-   --> $DIR/doctest/display-output.rs:13:8
+   --> $DIR/display-output.rs:13:8
23    |
24 LL | fn foo(x: &dyn std::fmt::Display) {}
25    |        ^ help: if this is intentional, prefix it with an underscore: `_x`
26 
27 warning: function `foo` is never used
-   --> $DIR/doctest/display-output.rs:13:4
+   --> $DIR/display-output.rs:13:4
+   --> $DIR/display-output.rs:13:4
29    |
30 LL | fn foo(x: &dyn std::fmt::Display) {}

37 
38 
39 successes:
39 successes:
-     $DIR/doctest/display-output.rs - foo (line 9)
+     $DIR/display-output.rs - foo (line 9)
42 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
43 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/display-output/display-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/display-output.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/display-output.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/display-output" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/display-output/auxiliary" "--edition=2018" "--test" "--test-args=--show-output"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/display-output.rs - foo (line 9) ... ok

successes:
successes:

---- /checkout/tests/rustdoc-ui/doctest/display-output.rs - foo (line 9) stdout ----
warning: unused variable: `x`
  --> /checkout/tests/rustdoc-ui/doctest/display-output.rs:11:5
LL | let x = 12;
   |     ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
note: the lint level is defined here
---

warning: unused variable: `x`
  --> /checkout/tests/rustdoc-ui/doctest/display-output.rs:13:8
   |
LL | fn foo(x: &dyn std::fmt::Display) {}
   |        ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: function `foo` is never used
  --> /checkout/tests/rustdoc-ui/doctest/display-output.rs:13:4
   |
   |
LL | fn foo(x: &dyn std::fmt::Display) {}
   |
   = note: `#[warn(dead_code)]` implied by `#[warn(unused)]`

warning: 3 warnings emitted
---
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/doc-comment-multi-line-attr.rs stdout ----

1 
2 running 1 test
- test $DIR/doctest/doc-comment-multi-line-attr.rs - (line 7) ... ok
---
To only update this specific test, also pass `--test-args doctest/doc-comment-multi-line-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/doc-comment-multi-line-attr.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-comment-multi-line-attr" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-comment-multi-line-attr/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/doc-comment-multi-line-attr.rs - (line 7) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/doc-comment-multi-line-cfg-attr.rs stdout ----

1 
2 running 1 test
- test $DIR/doctest/doc-comment-multi-line-cfg-attr.rs - Bar (line 6) ... ok
---
To only update this specific test, also pass `--test-args doctest/doc-comment-multi-line-cfg-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/doc-comment-multi-line-cfg-attr.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-comment-multi-line-cfg-attr" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-comment-multi-line-cfg-attr/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/doc-comment-multi-line-cfg-attr.rs - Bar (line 6) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/doc-test-doctest-feature.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/doc-test-doctest-feature.rs - Foo (line 9) ... ok
+ test $DIR/doc-test-doctest-feature.rs - Foo (line 9) ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-test-doctest-feature/doc-test-doctest-feature.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/doc-test-doctest-feature.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/doc-test-doctest-feature.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-test-doctest-feature" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-test-doctest-feature/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/doc-test-doctest-feature.rs - Foo (line 9) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/cfg-test.rs stdout ----

1 
2 running 2 tests
- test $DIR/doctest/cfg-test.rs - Bar (line 27) ... ok
- test $DIR/doctest/cfg-test.rs - Bar (line 27) ... ok
- test $DIR/doctest/cfg-test.rs - Foo (line 19) ... ok
+ test $DIR/cfg-test.rs - Bar (line 27) ... ok
+ test $DIR/cfg-test.rs - Foo (line 19) ... ok
6 test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
7 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/cfg-test/cfg-test.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/cfg-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/cfg-test.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/cfg-test" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/cfg-test/auxiliary" "--test" "--test-args" "--test-threads=1"
running 2 tests
test /checkout/tests/rustdoc-ui/doctest/cfg-test.rs - Bar (line 27) ... ok
test /checkout/tests/rustdoc-ui/doctest/cfg-test.rs - Foo (line 19) ... ok


test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.36s
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/doc-test-rustdoc-feature.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/doc-test-rustdoc-feature.rs - Foo (line 10) ... ok
+ test $DIR/doc-test-rustdoc-feature.rs - Foo (line 10) ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-test-rustdoc-feature/doc-test-rustdoc-feature.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/doc-test-rustdoc-feature.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/doc-test-rustdoc-feature.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-test-rustdoc-feature" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doc-test-rustdoc-feature/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/doc-test-rustdoc-feature.rs - Foo (line 10) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/doctest-multiline-crate-attribute.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/doctest-multiline-crate-attribute.rs - f (line 6) ... ok
+ test $DIR/doctest-multiline-crate-attribute.rs - f (line 6) ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doctest-multiline-crate-attribute/doctest-multiline-crate-attribute.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/doctest-multiline-crate-attribute.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/doctest-multiline-crate-attribute.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doctest-multiline-crate-attribute" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doctest-multiline-crate-attribute/auxiliary" "--test" "--test-args=--test-threads=1"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/doctest-multiline-crate-attribute.rs - f (line 6) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/failed-doctest-extra-semicolon-on-item.rs - m (line 11) ... FAILED
+ test $DIR/failed-doctest-extra-semicolon-on-item.rs - m (line 11) ... FAILED
5 failures:
6 


- ---- $DIR/doctest/failed-doctest-extra-semicolon-on-item.rs - m (line 11) stdout ----
+ ---- $DIR/failed-doctest-extra-semicolon-on-item.rs - m (line 11) stdout ----
8 error: expected item, found `;`
-   --> $DIR/doctest/failed-doctest-extra-semicolon-on-item.rs:12:12
10    |
10    |
11 LL | struct S {}; // unexpected semicolon after struct def

18 Couldn't compile the test.
19 
20 failures:
20 failures:
-     $DIR/doctest/failed-doctest-extra-semicolon-on-item.rs - m (line 11)
+     $DIR/failed-doctest-extra-semicolon-on-item.rs - m (line 11)
23 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
24 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item/failed-doctest-extra-semicolon-on-item.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/failed-doctest-extra-semicolon-on-item.rs`
error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item/auxiliary" "--test"
running 1 test
running 1 test
test /checkout/tests/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item.rs - m (line 11) ... FAILED
failures:


---- /checkout/tests/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item.rs - m (line 11) stdout ----
error: expected item, found `;`
  --> /checkout/tests/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item.rs:12:12
   |
LL | struct S {}; // unexpected semicolon after struct def
   |
   |
   = help: braced struct declarations are not followed by a semicolon
error: aborting due to previous error

Couldn't compile the test.


failures:
    /checkout/tests/rustdoc-ui/doctest/failed-doctest-extra-semicolon-on-item.rs - m (line 11)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
------------------------------------------
stderr: none



---- [ui] tests/rustdoc-ui/doctest/failed-doctest-missing-codes.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/failed-doctest-missing-codes.rs - Foo (line 9) - compile fail ... FAILED
+ test $DIR/failed-doctest-missing-codes.rs - Foo (line 9) - compile fail ... FAILED
5 failures:
6 


- ---- $DIR/doctest/failed-doctest-missing-codes.rs - Foo (line 9) stdout ----
+ ---- $DIR/failed-doctest-missing-codes.rs - Foo (line 9) stdout ----
-   --> $DIR/doctest/failed-doctest-missing-codes.rs:10:13
+   --> $DIR/failed-doctest-missing-codes.rs:10:13
10    |
11 LL | let x: () = 5i32;
11 LL | let x: () = 5i32;
12    |        --   ^^^^ expected `()`, found `i32`

19 Some expected error codes were not found: ["E0004"]
21 failures:
21 failures:
-     $DIR/doctest/failed-doctest-missing-codes.rs - Foo (line 9)
+     $DIR/failed-doctest-missing-codes.rs - Foo (line 9)
24 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
25 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-missing-codes/failed-doctest-missing-codes.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/failed-doctest-missing-codes.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/failed-doctest-missing-codes.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-missing-codes" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-missing-codes/auxiliary" "--test"
running 1 test
running 1 test
test /checkout/tests/rustdoc-ui/doctest/failed-doctest-missing-codes.rs - Foo (line 9) - compile fail ... FAILED
failures:


---- /checkout/tests/rustdoc-ui/doctest/failed-doctest-missing-codes.rs - Foo (line 9) stdout ----
  --> /checkout/tests/rustdoc-ui/doctest/failed-doctest-missing-codes.rs:10:13
   |
LL | let x: () = 5i32;
   |        --   ^^^^ expected `()`, found `i32`
   |        --   ^^^^ expected `()`, found `i32`
   |        |
   |        expected due to this

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Some expected error codes were not found: ["E0004"]
failures:
failures:
    /checkout/tests/rustdoc-ui/doctest/failed-doctest-missing-codes.rs - Foo (line 9)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
------------------------------------------
stderr: none



---- [ui] tests/rustdoc-ui/doctest/failed-doctest-compile-fail.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/failed-doctest-compile-fail.rs - Foo (line 9) - compile fail ... FAILED
+ test $DIR/failed-doctest-compile-fail.rs - Foo (line 9) - compile fail ... FAILED
5 failures:
6 


- ---- $DIR/doctest/failed-doctest-compile-fail.rs - Foo (line 9) stdout ----
+ ---- $DIR/failed-doctest-compile-fail.rs - Foo (line 9) stdout ----
8 Test compiled successfully, but it's marked `compile_fail`.
10 failures:


-     $DIR/doctest/failed-doctest-compile-fail.rs - Foo (line 9)
+     $DIR/failed-doctest-compile-fail.rs - Foo (line 9)
13 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
14 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-compile-fail/failed-doctest-compile-fail.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/failed-doctest-compile-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/failed-doctest-compile-fail.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-compile-fail" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-compile-fail/auxiliary" "--test"
running 1 test
running 1 test
test /checkout/tests/rustdoc-ui/doctest/failed-doctest-compile-fail.rs - Foo (line 9) - compile fail ... FAILED
failures:


---- /checkout/tests/rustdoc-ui/doctest/failed-doctest-compile-fail.rs - Foo (line 9) stdout ----
Test compiled successfully, but it's marked `compile_fail`.
failures:
failures:
    /checkout/tests/rustdoc-ui/doctest/failed-doctest-compile-fail.rs - Foo (line 9)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
------------------------------------------
stderr: none



---- [ui] tests/rustdoc-ui/doctest/nocapture-fail.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/nocapture-fail.rs - Foo (line 7) - compile fail ... ok
+ test $DIR/nocapture-fail.rs - Foo (line 7) - compile fail ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 


---
To only update this specific test, also pass `--test-args doctest/nocapture-fail.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/nocapture-fail.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/nocapture-fail" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/nocapture-fail/auxiliary" "--test" "-Zunstable-options" "--nocapture"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/nocapture-fail.rs - Foo (line 7) - compile fail ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
---
LL | |     Input: 123
LL | | }
   | |_^
   |
help: you might have forgotten to add the struct literal inside the block
   |
LL ~ fn foo() { SomeStruct {
LL ~ } }
   |

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] tests/rustdoc-ui/doctest/failed-doctest-should-panic.rs stdout ----

1 
2 running 1 test
2 running 1 test
- test $DIR/doctest/failed-doctest-should-panic.rs - Foo (line 9) ... FAILED
+ test $DIR/failed-doctest-should-panic.rs - Foo (line 9) ... FAILED
5 failures:
6 


- ---- $DIR/doctest/failed-doctest-should-panic.rs - Foo (line 9) stdout ----
+ ---- $DIR/failed-doctest-should-panic.rs - Foo (line 9) stdout ----
8 Test executable succeeded, but it's marked `should_panic`.
10 failures:


-     $DIR/doctest/failed-doctest-should-panic.rs - Foo (line 9)
+     $DIR/failed-doctest-should-panic.rs - Foo (line 9)
13 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
14 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-should-panic/failed-doctest-should-panic.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/failed-doctest-should-panic.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/failed-doctest-should-panic.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-should-panic" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/failed-doctest-should-panic/auxiliary" "--test"
running 1 test
running 1 test
test /checkout/tests/rustdoc-ui/doctest/failed-doctest-should-panic.rs - Foo (line 9) ... FAILED
failures:


---- /checkout/tests/rustdoc-ui/doctest/failed-doctest-should-panic.rs - Foo (line 9) stdout ----
Test executable succeeded, but it's marked `should_panic`.
failures:
failures:
    /checkout/tests/rustdoc-ui/doctest/failed-doctest-should-panic.rs - Foo (line 9)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
------------------------------------------
stderr: none



---- [ui] tests/rustdoc-ui/doctest/no-run-flag.rs stdout ----
diff of stdout:

1 
2 running 7 tests
- test $DIR/doctest/no-run-flag.rs - f (line 11) - compile ... ok
- test $DIR/doctest/no-run-flag.rs - f (line 14) ... ignored
- test $DIR/doctest/no-run-flag.rs - f (line 17) - compile ... ok
- test $DIR/doctest/no-run-flag.rs - f (line 23) - compile fail ... ok
- test $DIR/doctest/no-run-flag.rs - f (line 28) - compile ... ok
- test $DIR/doctest/no-run-flag.rs - f (line 32) - compile ... ok
- test $DIR/doctest/no-run-flag.rs - f (line 8) - compile ... ok
+ test $DIR/no-run-flag.rs - f (line 11) - compile ... ok
+ test $DIR/no-run-flag.rs - f (line 14) ... ignored
+ test $DIR/no-run-flag.rs - f (line 17) - compile ... ok
+ test $DIR/no-run-flag.rs - f (line 23) - compile fail ... ok
+ test $DIR/no-run-flag.rs - f (line 28) - compile ... ok
+ test $DIR/no-run-flag.rs - f (line 32) - compile ... ok
+ test $DIR/no-run-flag.rs - f (line 8) - compile ... ok
11 test result: ok. 6 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in $TIME
12 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/no-run-flag/no-run-flag.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/no-run-flag.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/no-run-flag.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/no-run-flag" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/no-run-flag/auxiliary" "-Z" "unstable-options" "--test" "--no-run" "--test-args=--test-threads=1"
running 7 tests
test /checkout/tests/rustdoc-ui/doctest/no-run-flag.rs - f (line 11) - compile ... ok
test /checkout/tests/rustdoc-ui/doctest/no-run-flag.rs - f (line 14) ... ignored
test /checkout/tests/rustdoc-ui/doctest/no-run-flag.rs - f (line 17) - compile ... ok
---
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/nocapture.rs stdout ----

1 
2 running 1 test
3 hello!
3 hello!
- test $DIR/doctest/nocapture.rs - Foo (line 6) ... ok
+ test $DIR/nocapture.rs - Foo (line 6) ... ok
6 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
7 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/nocapture/nocapture.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/nocapture.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/nocapture.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/nocapture" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/nocapture/auxiliary" "--test" "-Zunstable-options" "--nocapture"
running 1 test
hello!
test /checkout/tests/rustdoc-ui/doctest/nocapture.rs - Foo (line 6) ... ok

---
diff of stdout:

1 
2 running 1 test
- test $DIR/doctest/unparseable-doc-test.rs - foo (line 7) ... FAILED
+ test $DIR/unparseable-doc-test.rs - foo (line 7) ... FAILED
5 failures:
6 


- ---- $DIR/doctest/unparseable-doc-test.rs - foo (line 7) stdout ----
+ ---- $DIR/unparseable-doc-test.rs - foo (line 7) stdout ----
8 error[E0765]: unterminated double quote string
-   --> $DIR/doctest/unparseable-doc-test.rs:9:1
+   --> $DIR/unparseable-doc-test.rs:9:1
10    |
11 LL | "unterminated

17 Couldn't compile the test.
18 
19 failures:
19 failures:
-     $DIR/doctest/unparseable-doc-test.rs - foo (line 7)
+     $DIR/unparseable-doc-test.rs - foo (line 7)
22 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
23 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/unparseable-doc-test/unparseable-doc-test.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/unparseable-doc-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command:  RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/unparseable-doc-test.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/unparseable-doc-test" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/unparseable-doc-test/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/unparseable-doc-test.rs - foo (line 7) ... FAILED

failures:
failures:

---- /checkout/tests/rustdoc-ui/doctest/unparseable-doc-test.rs - foo (line 7) stdout ----
error[E0765]: unterminated double quote string
  --> /checkout/tests/rustdoc-ui/doctest/unparseable-doc-test.rs:9:1
   |
LL | "unterminated

error: aborting due to previous error

For more information about this error, try `rustc --explain E0765`.
---
------------------------------------------
stderr: none


---- [ui] tests/rustdoc-ui/doctest/doctest-output.rs stdout ----

1 
2 running 3 tests
- test $DIR/doctest/doctest-output.rs - (line 8) ... ok
- test $DIR/doctest/doctest-output.rs - (line 8) ... ok
- test $DIR/doctest/doctest-output.rs - ExpandedStruct (line 24) ... ok
- test $DIR/doctest/doctest-output.rs - foo::bar (line 18) ... ok
+ test $DIR/doctest-output.rs - (line 8) ... ok
+ test $DIR/doctest-output.rs - ExpandedStruct (line 24) ... ok
+ test $DIR/doctest-output.rs - foo::bar (line 18) ... ok
7 test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
8 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doctest-output/doctest-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/doctest-output.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/doctest-output.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doctest-output" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/doctest-output/auxiliary" "--edition=2018" "--test" "--test-args=--test-threads=1"
running 3 tests
test /checkout/tests/rustdoc-ui/doctest/doctest-output.rs - (line 8) ... ok
test /checkout/tests/rustdoc-ui/doctest/doctest-output.rs - ExpandedStruct (line 24) ... ok
test /checkout/tests/rustdoc-ui/doctest/doctest-output.rs - foo::bar (line 18) ... ok
---
diff of stdout:

1 
2 running 1 test
- test $DIR/doctest/test-no_std.rs - f (line 10) ... ok
+ test $DIR/test-no_std.rs - f (line 10) ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/test-no_std/test-no_std.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/test-no_std.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/test-no_std.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/test-no_std" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/test-no_std/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/test-no_std.rs - f (line 10) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s
---
diff of stdout:

1 
2 running 1 test
- test $DIR/doctest/run-directory.rs - foo (line 19) ... ok
+ test $DIR/run-directory.rs - foo (line 19) ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/run-directory.incorrect/run-directory.incorrect.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/run-directory.rs`

error in revision `incorrect`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/run-directory.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "incorrect" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/run-directory.incorrect" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/run-directory.incorrect/auxiliary" "--test" "--test-run-directory=/checkout/tests/rustdoc-ui/coverage"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/run-directory.rs - foo (line 19) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s
---
diff of stdout:

1 
2 running 1 test
- test $DIR/doctest/run-directory.rs - foo (line 10) ... ok
+ test $DIR/run-directory.rs - foo (line 10) ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/run-directory.correct/run-directory.correct.stdout
To only update this specific test, also pass `--test-args doctest/run-directory.rs`


error in revision `correct`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/run-directory.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "correct" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/run-directory.correct" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/run-directory.correct/auxiliary" "--test" "--test-run-directory=/checkout/tests/rustdoc-ui"
running 1 test
test /checkout/tests/rustdoc-ui/doctest/run-directory.rs - foo (line 10) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
---
diff of stdout:

1 
2 running 5 tests
- test $DIR/doctest/test-type.rs - f (line 12) ... ignored
- test $DIR/doctest/test-type.rs - f (line 15) - compile ... ok
- test $DIR/doctest/test-type.rs - f (line 21) - compile fail ... ok
- test $DIR/doctest/test-type.rs - f (line 6) ... ok
- test $DIR/doctest/test-type.rs - f (line 9) ... ok
+ test $DIR/test-type.rs - f (line 12) ... ignored
+ test $DIR/test-type.rs - f (line 15) - compile ... ok
+ test $DIR/test-type.rs - f (line 21) - compile fail ... ok
+ test $DIR/test-type.rs - f (line 6) ... ok
+ test $DIR/test-type.rs - f (line 9) ... ok
9 test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in $TIME
10 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/test-type/test-type.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doctest/test-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doctest/test-type.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/test-type" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doctest/test-type/auxiliary" "--test" "--test-args=--test-threads=1"
running 5 tests
test /checkout/tests/rustdoc-ui/doctest/test-type.rs - f (line 12) ... ignored
test /checkout/tests/rustdoc-ui/doctest/test-type.rs - f (line 15) - compile ... ok
test /checkout/tests/rustdoc-ui/doctest/test-type.rs - f (line 21) - compile fail ... ok
---


---- [ui] tests/rustdoc-ui/issues/issue-107918.rs stdout ----

error: aux-build `/checkout/tests/rustdoc-ui/issues/auxiliary/panic-handler.rs` source not found
thread '[ui] tests/rustdoc-ui/issues/issue-107918.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2287:9

---- [ui] tests/rustdoc-ui/issues/issue-80992.rs stdout ----
diff of stdout:

---
To only update this specific test, also pass `--test-args issues/issue-80992.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issues/issue-80992.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issues/issue-80992" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issues/issue-80992/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/issues/issue-80992.rs - test (line 7) - compile fail ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
---
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
---
To only update this specific test, also pass `--test-args issues/issue-91134.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issues/issue-91134.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issues/issue-91134" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issues/issue-91134/auxiliary" "--test" "--crate-name=empty_fn" "--extern=empty_fn" "--test-args=--test-threads=1" "--edition=2021"
running 1 test
test /checkout/tests/rustdoc-ui/issues/issue-91134.rs - Something (line 10) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
