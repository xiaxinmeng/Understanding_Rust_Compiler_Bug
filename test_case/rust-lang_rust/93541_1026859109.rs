plain
test [ui] ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] ui/panics/runtime-switch.rs#v0 stdout ----
diff of run.stderr:
2 stack backtrace:
3    0: std::panicking::begin_panic::<&str>
4    1: runtime_switch::main
+    2: <fn() as core::ops::function::FnOnce<()>>::call_once
+    2: <fn() as core::ops::function::FnOnce<()>>::call_once
5 note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.v0/runtime-switch.v0.run.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error in revision `v0`: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.v0/a"
------------------------------------------

------------------------------------------
stderr:
---

------------------------------------------


---- [ui] ui/panics/runtime-switch.rs#legacy stdout ----
diff of run.stderr:
2 stack backtrace:
3    0: std::panicking::begin_panic
4    1: runtime_switch::main
+    2: core::ops::function::FnOnce::call_once
+    2: core::ops::function::FnOnce::call_once
5 note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.legacy/runtime-switch.legacy.run.stderr

error in revision `legacy`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.legacy/a"
stdout:
------------------------------------------

