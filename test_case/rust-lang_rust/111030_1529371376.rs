plain
..............................................i.........................................  7920/14888
.......................................................................i................  8008/14888
........................................................................................  8096/14888
........................................................................................  8184/14888
......................................................FF......F..F...F..................  8272/14888
................................................i.......................................  8448/14888
........................................................................................  8536/14888
....................................................................ii..................  8624/14888
........................................................................................  8712/14888
---
---- [ui] tests/ui/macros/assert-eq-macro-msg.rs stdout ----

error: error pattern 'panicked at 'assertion failed: `(left == right)`' not found!

error: error pattern 'right: `3`: 1 + 1 definitely should be 3'' not found!
error: multiple error patterns not found
status: exit status: 101
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-msg" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-msg/a"
stdout: none
thread 'main' panicked at fake-test-src-base/macros/assert-eq-macro-msg.rs:8:5:
thread 'main' panicked at fake-test-src-base/macros/assert-eq-macro-msg.rs:8:5:
assertion failed: `(1 + 1 == 3)`
 error: 1 + 1 definitely should be 3,
 right: `3`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------



---- [ui] tests/ui/macros/assert-eq-macro-panic.rs stdout ----

error: error pattern 'assertion failed: `(left == right)`' not found!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-panic" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-panic/a"
stdout: none
thread 'main' panicked at fake-test-src-base/macros/assert-eq-macro-panic.rs:8:5:
thread 'main' panicked at fake-test-src-base/macros/assert-eq-macro-panic.rs:8:5:
assertion failed: `(14 == 15)`
 right: `15`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------



---- [ui] tests/ui/macros/assert-matches-macro-msg.rs stdout ----

error: error pattern 'panicked at 'assertion failed: `(left matches right)`' not found!

error: error pattern 'right: `3`: 1 + 1 definitely should be 3'' not found!
error: multiple error patterns not found
status: exit status: 101
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-matches-macro-msg" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-matches-macro-msg/a"
stdout: none
thread 'main' panicked at fake-test-src-base/macros/assert-matches-macro-msg.rs:12:5:
thread 'main' panicked at fake-test-src-base/macros/assert-matches-macro-msg.rs:12:5:
assertion failed: `(& left matches & right)`
 error: 1 + 1 definitely should be 3,
 right: `3`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------



---- [ui] tests/ui/macros/assert-ne-macro-panic.rs stdout ----

error: error pattern 'assertion failed: `(left != right)`' not found!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-panic" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-panic/a"
stdout: none
thread 'main' panicked at fake-test-src-base/macros/assert-ne-macro-panic.rs:8:5:
thread 'main' panicked at fake-test-src-base/macros/assert-ne-macro-panic.rs:8:5:
assertion failed: `(14 != 14)`
 right: `14`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------



---- [ui] tests/ui/macros/assert-ne-macro-msg.rs stdout ----

error: error pattern 'panicked at 'assertion failed: `(left != right)`' not found!

error: error pattern 'right: `2`: 1 + 1 definitely should not be 2'' not found!
error: multiple error patterns not found
status: exit status: 101
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-msg" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-msg/a"
stdout: none
thread 'main' panicked at fake-test-src-base/macros/assert-ne-macro-msg.rs:8:5:
thread 'main' panicked at fake-test-src-base/macros/assert-ne-macro-msg.rs:8:5:
assertion failed: `(1 + 1 != 2)`
 error: 1 + 1 definitely should not be 2,
 right: `2`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------

