plain
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 200 tests
.........................................................i.............................. 88/200
...................................................................FF.......F........... 176/200
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [ui] tests/rustdoc-ui/remap-path-prefix-failed-doctest-output.rs stdout ----

1 
2 running 1 test
- test remapped_path/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10) ... FAILED
- test remapped_path/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10) ... FAILED
+ test $DIR/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10) ... FAILED
5 failures:
6 


- ---- remapped_path/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10) stdout ----
+ ---- $DIR/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10) stdout ----
8 Test executable failed (exit status: 101).
10 stderr:


- thread 'main' panicked at 'oh no', remapped_path/remap-path-prefix-failed-doctest-output.rs:3:1
+ thread 'main' panicked at 'oh no', $DIR/remap-path-prefix-failed-doctest-output.rs:3:1
13 
14 

15 
15 
16 failures:
-     remapped_path/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10)
+     $DIR/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10)
19 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
20 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/remap-path-prefix-failed-doctest-output/remap-path-prefix-failed-doctest-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args remap-path-prefix-failed-doctest-output.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/remap-path-prefix-failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/remap-path-prefix-failed-doctest-output" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/remap-path-prefix-failed-doctest-output/auxiliary" "--test" "-Z" "unstable-options" "--remap-path-prefix=tests/rustdoc-ui=remapped_path" "--test-args" "--test-threads=1"
running 1 test
test /checkout/tests/rustdoc-ui/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10) ... FAILED

failures:
failures:

---- /checkout/tests/rustdoc-ui/remap-path-prefix-failed-doctest-output.rs - SomeStruct (line 10) stdout ----
Test executable failed (exit status: 101).

stderr:
thread 'main' panicked at 'oh no', /checkout/tests/rustdoc-ui/remap-path-prefix-failed-doctest-output.rs:3:1



failures:
---
diff of stdout:

1 
2 running 1 test
- test remapped_path/remap-path-prefix-invalid-doctest.rs - SomeStruct (line 10) ... FAILED
+ test $DIR/remap-path-prefix-invalid-doctest.rs - SomeStruct (line 10) ... FAILED
5 failures:
6 


- ---- remapped_path/remap-path-prefix-invalid-doctest.rs - SomeStruct (line 10) stdout ----
+ ---- $DIR/remap-path-prefix-invalid-doctest.rs - SomeStruct (line 10) stdout ----
8 error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `is`
-   --> remapped_path/remap-path-prefix-invalid-doctest.rs:11:6
10    |
11 LL | this is not real code
12    |      ^^ expected one of 8 possible tokens

---
To only update this specific test, also pass `--test-args remap-path-prefix-invalid-doctest.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/remap-path-prefix-invalid-doctest.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/remap-path-prefix-invalid-doctest" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/remap-path-prefix-invalid-doctest/auxiliary" "--test" "-Z" "unstable-options" "--remap-path-prefix=tests/rustdoc-ui=remapped_path" "--test-args" "--test-threads=1"
running 1 test
test /checkout/tests/rustdoc-ui/remap-path-prefix-invalid-doctest.rs - SomeStruct (line 10) ... FAILED

failures:
failures:

---- /checkout/tests/rustdoc-ui/remap-path-prefix-invalid-doctest.rs - SomeStruct (line 10) stdout ----
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `is`
   |
LL | this is not real code
   |      ^^ expected one of 8 possible tokens

---
diff of stdout:

1 
2 running 1 test
- test remapped_path/remap-path-prefix-passed-doctest-output.rs - SomeStruct (line 11) ... ok
+ test $DIR/remap-path-prefix-passed-doctest-output.rs - SomeStruct (line 11) ... ok
5 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
6 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/remap-path-prefix-passed-doctest-output/remap-path-prefix-passed-doctest-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args remap-path-prefix-passed-doctest-output.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/remap-path-prefix-passed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/remap-path-prefix-passed-doctest-output" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/remap-path-prefix-passed-doctest-output/auxiliary" "--test" "-Z" "unstable-options" "--remap-path-prefix=tests/rustdoc-ui=remapped_path" "--test-args" "--test-threads=1"
running 1 test
test /checkout/tests/rustdoc-ui/remap-path-prefix-passed-doctest-output.rs - SomeStruct (line 11) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
