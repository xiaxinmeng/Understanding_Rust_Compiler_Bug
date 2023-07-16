
failures:

---- time::tests::macos_resolution_regression stdout ----
[library/std/src/time/tests.rs:21] t0 = Instant {
    t: 511866740696750,
}
[library/std/src/time/tests.rs:21] t1 = Instant {
    t: 511866740696751,
}
[library/std/src/time/tests.rs:21] d = 41ns
thread 'time::tests::macos_resolution_regression' panicked at 'assertion failed: `(left == right)`
  left: `Instant { t: 511866740696750 }`,
 right: `Instant { t: 511866740696751 }`', library/std/src/time/tests.rs:22:5


failures:
    time::tests::macos_resolution_regression
