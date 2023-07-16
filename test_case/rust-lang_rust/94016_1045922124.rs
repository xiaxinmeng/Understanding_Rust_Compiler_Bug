plain
..................i....i........................................i................i.............i.... 6800/12658
................................................i................................................... 6900/12658
...................................................i................................................ 7000/12658
.................................................................................................... 7100/12658
...............................FF.....F..FF..............................ii......................... 7200/12658
.................................................................................................... 7400/12658
..................................i................................................................. 7500/12658
.................................................................................................... 7600/12658
..........................................................................................ii........ 7700/12658
---
.................................................................................................... 11000/12658
.................................................................................................... 11100/12658
.................................................................................................... 11200/12658
...........................................................................iiiiiii.................. 11300/12658
..........i.........................F........F...................................................... 11400/12658
.................................................................................................... 11600/12658
.................................................................................................... 11700/12658
.................................................................................................... 11800/12658
.................................................................................i.................. 11900/12658
---
---- [ui] ui/macros/assert-eq-macro-msg.rs stdout ----

error: error pattern 'panicked at 'assertion failed: `(left == right)`' not found!

error: error pattern ' left: `2`' not found!

error: error pattern 'right: `3`: 1 + 1 definitely should be 3'' not found!
error: multiple error patterns not found
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-msg/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(upper_bounds == target)`
   Error: `1 + 1 definitely should be 3`,
   upper_bounds: `2`,
   target: `3`', /checkout/src/test/ui/macros/assert-eq-macro-msg.rs:8:5

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------



---- [ui] ui/macros/assert-eq-macro-panic.rs stdout ----

error: error pattern 'assertion failed: `(left == right)`' not found!

error: error pattern ' left: `14`' not found!

error: error pattern 'right: `15`' not found!
error: multiple error patterns not found
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-panic/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(upper_bounds == target)`
   upper_bounds: `14`,
   target: `15`', /checkout/src/test/ui/macros/assert-eq-macro-panic.rs:8:5

------------------------------------------



---- [ui] ui/macros/assert-ne-macro-msg.rs stdout ----

error: error pattern 'panicked at 'assertion failed: `(left != right)`' not found!

error: error pattern ' left: `2`' not found!

error: error pattern 'right: `2`: 1 + 1 definitely should not be 2'' not found!
error: multiple error patterns not found
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-msg/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(upper_bounds != target)`
   Error: `1 + 1 definitely should not be 2`,
   upper_bounds: `2`,
   target: `2`', /checkout/src/test/ui/macros/assert-ne-macro-msg.rs:8:5

------------------------------------------



---- [ui] ui/macros/assert-matches-macro-msg.rs stdout ----

error: error pattern 'panicked at 'assertion failed: `(left matches right)`' not found!

error: error pattern ' left: `2`' not found!

error: error pattern 'right: `3`: 1 + 1 definitely should be 3'' not found!
error: multiple error patterns not found
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-matches-macro-msg/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(upper_bounds matches target)`
   Error: `1 + 1 definitely should be 3`,
   upper_bounds: `2`,
   target: `3`', /checkout/src/test/ui/macros/assert-matches-macro-msg.rs:12:5

------------------------------------------



---- [ui] ui/macros/assert-ne-macro-panic.rs stdout ----

error: error pattern 'assertion failed: `(left != right)`' not found!

error: error pattern ' left: `14`' not found!

error: error pattern 'right: `14`' not found!
error: multiple error patterns not found
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-panic/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(upper_bounds != target)`
   upper_bounds: `14`,
   target: `14`', /checkout/src/test/ui/macros/assert-ne-macro-panic.rs:8:5

------------------------------------------



---- [ui] ui/test-attrs/test-panic-abort-nocapture.rs stdout ----
diff of run.stderr:

- thread 'main' panicked at 'assertion failed: `(left == right)`
-   left: `2`,
-  right: `4`', $DIR/test-panic-abort-nocapture.rs:33:5
+ thread 'main' panicked at 'assertion failed: `(upper_bounds == target)`
+    upper_bounds: `2`,
+    target: `4`', $DIR/test-panic-abort-nocapture.rs:33:5
- thread 'main' panicked at 'assertion failed: `(left == right)`
-   left: `2`,
-  right: `4`', $DIR/test-panic-abort-nocapture.rs:27:5
-  right: `4`', $DIR/test-panic-abort-nocapture.rs:27:5
+ thread 'main' panicked at 'assertion failed: `(upper_bounds == target)`
+    upper_bounds: `2`,
+    target: `4`', $DIR/test-panic-abort-nocapture.rs:27:5
9 testing321
10 



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/test-panic-abort-nocapture.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/a" "--test-threads=1" "--nocapture"
------------------------------------------

running 4 tests
test it_fails ... about to fail
---
ok

failures:

---- it_fails stdout ----
---- it_fails stderr ----

failures:
    it_fails


test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(upper_bounds == target)`
   upper_bounds: `2`,
   target: `4`', /checkout/src/test/ui/test-attrs/test-panic-abort-nocapture.rs:33:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'assertion failed: `(upper_bounds == target)`
   upper_bounds: `2`,
   target: `4`', /checkout/src/test/ui/test-attrs/test-panic-abort-nocapture.rs:27:5
testing321

------------------------------------------



---- [ui] ui/test-attrs/test-panic-abort.rs stdout ----
diff of run.stdout:

16 testing123
17 ---- it_fails stderr ----
- thread 'main' panicked at 'assertion failed: `(left == right)`
-   left: `2`,
-  right: `5`', $DIR/test-panic-abort.rs:34:5
-  right: `5`', $DIR/test-panic-abort.rs:34:5
+ thread 'main' panicked at 'assertion failed: `(upper_bounds == target)`
+    upper_bounds: `2`,
+    target: `5`', $DIR/test-panic-abort.rs:34:5
23 
24 



The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/test-panic-abort.run.stdout
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/a" "--test-threads=1"
------------------------------------------

running 5 tests
test it_exits ... FAILED
test it_exits ... FAILED
test it_fails ... FAILED
test it_panics - should panic ... ok
test it_works ... ok
test no_residual_environment ... ok

failures:

---- it_exits stdout ----
---- it_exits stderr ----
note: got unexpected return code 123
---- it_fails stdout ----
testing123
testing123
---- it_fails stderr ----
testing321
thread 'main' panicked at 'assertion failed: `(upper_bounds == target)`
   upper_bounds: `2`,
   target: `5`', /checkout/src/test/ui/test-attrs/test-panic-abort.rs:34:5


failures:
    it_exits
