plain
........................................................................................ 11880/13721
........................................................................................ 11968/13721
........................................................................................ 12056/13721
................................i.......i........i.....i.....................i.......... 12144/13721
F...............F...FFFFF............................................................... 12232/13721
........................................................................................ 12408/13721
........................................................................................ 12496/13721
........................................................................................ 12584/13721
........................................................................................ 12672/13721
---
- 

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-filter-multiple/test-filter-multiple.run.stdout
normalized run.stderr:
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13



The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-filter-multiple/test-filter-multiple.run.stderr
error: 2 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-filter-multiple/a" "--test-threads=1" "test1" "test2"
running 2 tests
test test1 ...
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
------------------------------------------


---- [ui] src/test/ui/test-attrs/test-type.rs stdout ----
---
+ test test_ok ... 
- 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-type/test-type.run.stdout
normalized run.stderr:
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13



The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-type/test-type.run.stderr
error: 2 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-type/a" "--test-threads=1"
running 3 tests
test test_no_run ... ignored, msg
test test_ok ...
------------------------------------------
------------------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
------------------------------------------


---- [ui] src/test/ui/test-attrs/test-passed.rs stdout ----
---
+ test it_works ... 
- 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-passed/test-passed.run.stdout
normalized run.stderr:
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13



The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-passed/test-passed.run.stderr
error: 2 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-passed/a" "--test-threads=1"
running 2 tests
test it_works ...
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
------------------------------------------


---- [ui] src/test/ui/test-attrs/test-thread-nocapture.rs stdout ----
---
+ test thready_fail ... 
- 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-nocapture/test-thread-nocapture.run.stdout
diff of run.stderr:
- thread 'main' panicked at 'explicit panic', $DIR/test-thread-nocapture.rs:32:5
- thread 'main' panicked at 'explicit panic', $DIR/test-thread-nocapture.rs:32:5
+ thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
3 


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-nocapture/test-thread-nocapture.run.stderr
error: 2 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-nocapture/a" "--test-threads=1" "--nocapture"
running 2 tests
test thready_fail ...
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
------------------------------------------


---- [ui] src/test/ui/test-attrs/test-thread-capture.rs stdout ----
---
+ test thready_fail ... 
- 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/test-thread-capture.run.stdout
normalized run.stderr:
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13



The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/test-thread-capture.run.stderr
error: 2 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/a" "--test-threads=1"
running 2 tests
test thready_fail ...
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
------------------------------------------


---- [ui] src/test/ui/test-attrs/test-panic-abort-nocapture.rs stdout ----
---
- ok
- 
- failures:
- 
- ---- it_fails stdout ----
- ---- it_fails stderr ----
- 
- failures:
-     it_fails
- 
- 
- test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
- 
+ test it_fails ... 
- 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/test-panic-abort-nocapture.run.stdout
diff of run.stderr:
- thread 'main' panicked at 'assertion failed: `(left == right)`
-   left: `2`,
-  right: `4`', $DIR/test-panic-abort-nocapture.rs:33:5
-  right: `4`', $DIR/test-panic-abort-nocapture.rs:33:5
+ thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
- thread 'main' panicked at 'assertion failed: `(left == right)`
-   left: `2`,
-  right: `4`', $DIR/test-panic-abort-nocapture.rs:27:5
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- testing321
10 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/test-panic-abort-nocapture.run.stderr
error: 2 errors occurred comparing run output.
error: 2 errors occurred comparing run output.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/a" "--test-threads=1" "--nocapture"
running 4 tests
test it_fails ...
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
------------------------------------------


---- [ui] src/test/ui/test-attrs/test-panic-abort.rs stdout ----
---
- test no_residual_environment ... ok
- 
- failures:
- 
- ---- it_exits stdout ----
- ---- it_exits stderr ----
- note: got unexpected return code 123
- ---- it_fails stdout ----
- testing123
- testing123
- ---- it_fails stderr ----
- thread 'main' panicked at 'assertion failed: `(left == right)`
-   left: `2`,
-  right: `5`', $DIR/test-panic-abort.rs:34:5
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
+ test it_exits ... 
- 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/test-panic-abort.run.stdout
normalized run.stderr:
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13



The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/test-panic-abort.run.stderr
error: 2 errors occurred comparing run output.
error: 2 errors occurred comparing run output.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/a" "--test-threads=1"
running 5 tests
test it_exits ...
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: join_handle.is_none()', library/test/src/lib.rs:332:13
------------------------------------------



