plain
......................................................................................i. 8360/13071
.ii..............................................................ii..................... 8448/13071
.......................................................................iiii............. 8536/13071
........................................................................................ 8624/13071
..................i.....................F.....F...........i............FF............... 8712/13071
........................................................................................ 8888/13071
...........................i............................................................ 8976/13071
........................................................................................ 9064/13071
.................................i...................................................... 9152/13071
---
5 note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.legacy/issue-47429-short-backtraces.legacy.run.stderr

error in revision `legacy`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.legacy/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/issue-47429-short-backtraces.rs:21:5
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/issue-47429-short-backtraces.rs:21:5
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
------------------------------------------


---- [ui] src/test/ui/panics/issue-47429-short-backtraces.rs#v0 stdout ----
diff of run.stderr:
1 thread 'main' panicked at 'explicit panic', $DIR/issue-47429-short-backtraces.rs:21:5
2 stack backtrace:
-    0: std::panicking::begin_panic::<&str>
-    1: issue_47429_short_backtraces::main
-    1: issue_47429_short_backtraces::main
5 note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.v0/issue-47429-short-backtraces.v0.run.stderr

error in revision `v0`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.v0/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/issue-47429-short-backtraces.rs:21:5
---
5 note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.legacy/runtime-switch.legacy.run.stderr

error in revision `legacy`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.legacy/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/runtime-switch.rs:24:5
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/runtime-switch.rs:24:5
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
------------------------------------------


---- [ui] src/test/ui/panics/runtime-switch.rs#v0 stdout ----
diff of run.stderr:
1 thread 'main' panicked at 'explicit panic', $DIR/runtime-switch.rs:24:5
2 stack backtrace:
-    0: std::panicking::begin_panic::<&str>
-    1: runtime_switch::main
-    1: runtime_switch::main
5 note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.v0/runtime-switch.v0.run.stderr

error in revision `v0`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.v0/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/runtime-switch.rs:24:5
---
---- [ui] src/test/ui/std-backtrace.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-backtrace/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: String::from_utf8_lossy(&p.stdout).contains(\"backtrace::main\")', /checkout/src/test/ui/std-backtrace.rs:34:5
------------------------------------------



