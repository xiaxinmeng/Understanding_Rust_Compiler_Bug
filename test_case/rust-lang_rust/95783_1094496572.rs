plain
diff of stdout:

1 
2 running 2 tests
- test $DIR/failed-doctest-output.rs - OtherStruct (line 22) ... FAILED
- test $DIR/failed-doctest-output.rs - SomeStruct (line 12) ... FAILED
+ test $DIR/failed-doctest-output.rs - OtherStruct (line 25) ... FAILED
+ test $DIR/failed-doctest-output.rs - SomeStruct (line 15) ... FAILED
6 failures:
7 


- ---- $DIR/failed-doctest-output.rs - OtherStruct (line 22) stdout ----
+ ---- $DIR/failed-doctest-output.rs - OtherStruct (line 25) stdout ----
9 error[E0425]: cannot find value `no` in this scope
-   --> $DIR/failed-doctest-output.rs:23:1
11    |
12 LL | no
13    | ^^ not found in this scope


16 
17 For more information about this error, try `rustc --explain E0425`.
18 Couldn't compile the test.
- ---- $DIR/failed-doctest-output.rs - SomeStruct (line 12) stdout ----
+ ---- $DIR/failed-doctest-output.rs - SomeStruct (line 15) stdout ----
20 Test executable failed (exit status: 101).
22 stdout:

32 
33 
33 
34 failures:
-     $DIR/failed-doctest-output.rs - OtherStruct (line 22)
-     $DIR/failed-doctest-output.rs - SomeStruct (line 12)
+     $DIR/failed-doctest-output.rs - OtherStruct (line 25)
+     $DIR/failed-doctest-output.rs - SomeStruct (line 15)
38 test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
39 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args failed-doctest-output.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "--test-args" "--test-threads=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
running 2 tests
test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 25) ... FAILED
test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 15) ... FAILED

---
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 15) stdout ----
Test executable failed (exit status: 101).
stdout:
stdout 1
stdout 2


stderr:
stderr 1
stderr 2
thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:7:1



failures:
