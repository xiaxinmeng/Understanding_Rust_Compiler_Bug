
[00:45:59] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:323:21
[00:45:59] ------------------------------------------
[00:45:59] error[E0004]: non-exhaustive patterns: `Initializing` not covered
[00:45:59]   --> /checkout/src/test/run-pass/tls-init-on-init.rs:40:19
[00:45:59]    |
[00:45:59] 40 |             match FOO.state() {
[00:45:59]    |                   ^^^^^^^^^^^ pattern `Initializing` not covered
[00:45:59] 
