plain
test [ui] tests/ui/transmutability/primitives/numbers.rs ... ok

failures:

---- [ui] tests/ui/async-await/future-sizes/future-as-arg.rs stdout ----
error: test run failed!
error: test run failed!
status: signal: 6 (SIGABRT) (core dumped)
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/future-as-arg" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/future-as-arg/a"
stdout: none
--- stderr -------------------------------
thread 'main' has overflowed its stack
fatal runtime error: stack overflow



failures:
