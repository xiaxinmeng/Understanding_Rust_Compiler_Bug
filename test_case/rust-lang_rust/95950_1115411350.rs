plain

---- [ui] src/test/ui/issues/issue-87707.rs stdout ----
diff of run.stderr:

- thread 'main' panicked at 'Here Once instance is poisoned.', $DIR/issue-87707.rs:15:24
+ thread 'main' panicked at 'Here Once instance is poisoned.', $DIR/issue-87707.rs:14:24
2 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- thread 'main' panicked at 'Once instance has previously been poisoned', $DIR/issue-87707.rs:17:7
+ thread 'main' panicked at 'Once instance has previously been poisoned', $DIR/issue-87707.rs:16:7


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87707/issue-87707.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87707/a"
stdout: none
stdout: none
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
--- stderr -------------------------------
thread 'main' panicked at 'Here Once instance is poisoned.', /checkout/src/test/ui/issues/issue-87707.rs:14:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'Once instance has previously been poisoned', /checkout/src/test/ui/issues/issue-87707.rs:16:7



failures:
