plain
- hello, world!
+ hello


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/runtime/stdout-during-shutdown/stdout-during-shutdown.run.stdout
normalized run.stderr:
thread '<unnamed>' panicked at 'cannot access a Thread Local Storage value during or after destruction: AccessError', library/std/src/thread/local.rs:418:26
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
fatal runtime error: failed to initiate panic, error 5


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/runtime/stdout-during-shutdown/stdout-during-shutdown.run.stderr
error: 2 errors occurred comparing run output.
error: 2 errors occurred comparing run output.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/runtime/stdout-during-shutdown/a"
hello
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread '<unnamed>' panicked at 'cannot access a Thread Local Storage value during or after destruction: AccessError', library/std/src/thread/local.rs:418:26
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
fatal runtime error: failed to initiate panic, error 5



failures:
