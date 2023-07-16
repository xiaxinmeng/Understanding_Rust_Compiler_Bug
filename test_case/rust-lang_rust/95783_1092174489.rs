plain
diff of stdout:

17 For more information about this error, try `rustc --explain E0425`.
18 Couldn't compile the test.
19 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 12) stdout ----
- Test executable failed (exit code 101).
+ Test executable failed (exit status: 101).
22 stdout:
23 stdout 1



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args failed-doctest-output.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "--test-args" "--test-threads=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
running 2 tests
test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 22) ... FAILED
test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 12) ... FAILED

---
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 12) stdout ----
Test executable failed (exit status: 101).
stdout:
stdout 1
stdout 2


stderr:
stderr 1
stderr 2
thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:7:1



failures:
