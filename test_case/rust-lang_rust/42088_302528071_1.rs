\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/bind-by-move-with-guard.rs","byte_start":668,"byte_end":672,"line_start":20,"line_end":20,"column_start":25,"column_end":29,"is_primary":true,"text":[{"text":"        Some(z) if z == true => { dispose(z); },","highlight_start":25,"highlight_end":29}],"label":"expected struct `std::sync::Arc`, found bool","suggested_replacement":null,"expansion":null}],"children":[{"message":"expected type `std::sync::Arc<bool>`\n   found type `bool`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":null}
[00:54:23] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:315
[00:54:23] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":null}
[00:54:23] 
[00:54:23] ------------------------------------------
[00:54:23] 
[00:54:23] thread '[run-pass] run-pass/bind-by-move-with-guard.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2472
[00:54:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:23] 
[00:54:23] 
[00:54:23] failures:
[00:54:23]     [run-pass] run-pass/bind-by-move-with-guard.rs
[00:54:23] 
[00:54:23] test result: FAILED. 2686 passed; 1 failed; 5 ignored; 0 measured
