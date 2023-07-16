plain
........................................................................................ 11352/13147
........................................................................................ 11440/13147
........................................................................................ 11528/13147
.....................................................................................i.. 11616/13147
......i........i.....i..............................i............................F..F..F 11704/13147
F....................................................................................... 11792/13147
........................................................................................ 11968/13147
........................................................................................ 12056/13147
........................................................................................ 12144/13147
........................................................................................ 12232/13147
---

---- [ui] src/test/ui/hygiene/panic-location.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'capacity overflow', $SRC_DIR/alloc/src/collections/vec_deque/mod.rs:LL:COL
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
3 



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/panic-location.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a"
stdout: none
---

---- [ui] src/test/ui/issues/issue-87707.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'Here Once instance is poisoned.', $DIR/issue-87707.rs:13:24
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
3 thread 'main' panicked at 'Once instance has previously been poisoned', $DIR/issue-87707.rs:15:7


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87707/issue-87707.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87707/a"
stdout: none
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'Here Once instance is poisoned.', /checkout/src/test/ui/issues/issue-87707.rs:13:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
thread 'main' panicked at 'Once instance has previously been poisoned', /checkout/src/test/ui/issues/issue-87707.rs:15:7


---- [ui] src/test/ui/panics/location-detail-panic-no-column.rs stdout ----
diff of run.stderr:
diff of run.stderr:

1 thread 'main' panicked at 'column-redacted', $DIR/location-detail-panic-no-column.rs:7:0
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
3 



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/location-detail-panic-no-column.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a"
stdout: none
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'column-redacted', /checkout/src/test/ui/panics/location-detail-panic-no-column.rs:7:0
------------------------------------------


---- [ui] src/test/ui/panics/location-detail-panic-no-file.rs stdout ----
---- [ui] src/test/ui/panics/location-detail-panic-no-file.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'file-redacted', <redacted>:7:5
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
3 



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/location-detail-panic-no-file.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a"
stdout: none
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'file-redacted', <redacted>:7:5
------------------------------------------


---- [ui] src/test/ui/panics/location-detail-unwrap-no-file.rs stdout ----
---- [ui] src/test/ui/panics/location-detail-unwrap-no-file.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', <redacted>:8:9
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
3 



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/location-detail-unwrap-no-file.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a"
stdout: none
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', <redacted>:8:9
------------------------------------------


---- [ui] src/test/ui/panics/location-detail-panic-no-line.rs stdout ----
---- [ui] src/test/ui/panics/location-detail-panic-no-line.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'line-redacted', $DIR/location-detail-panic-no-line.rs:0:5
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
3 



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/location-detail-panic-no-line.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a"
stdout: none
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'line-redacted', /checkout/src/test/ui/panics/location-detail-panic-no-line.rs:0:5
------------------------------------------


---- [ui] src/test/ui/process/multi-panic.rs stdout ----
---- [ui] src/test/ui/process/multi-panic.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/multi-panic/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some("note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.")`,
 right: `Some("note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace")`', /checkout/src/test/ui/process/multi-panic.rs:12:5
------------------------------------------


---- [ui] src/test/ui/test-attrs/test-panic-abort-nocapture.rs stdout ----
---
9 testing321
10 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/test-panic-abort-nocapture.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/a" "--test-threads=1" "--nocapture"
running 4 tests
test it_fails ... about to fail
FAILED
test it_panics - should panic ... about to panic
---
ok

failures:

---- it_fails stdout ----
---- it_fails stderr ----

failures:
    it_fails

---
24 
25 failures:


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/test-panic-abort.run.stdout
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/a" "--test-threads=1"
running 5 tests
test it_exits ... FAILED
test it_fails ... FAILED
test it_panics - should panic ... ok
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
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `5`', /checkout/src/test/ui/test-attrs/test-panic-abort.rs:34:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
16 
17 failures:


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/test-thread-capture.run.stdout
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/a" "--test-threads=1"
running 2 tests
test thready_fail ... FAILED
test thready_pass ... ok

---
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
3 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-nocapture/test-thread-nocapture.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-nocapture/a" "--test-threads=1" "--nocapture"
running 2 tests
test thready_fail ... fee
fie
foe
