

[01:54:46] ---- [run-make] run-make-fulldeps/rustdoc-error-lines stdout ----
[01:54:46] 
[01:54:46] error: make failed
[01:54:46] status: exit code: 2
[01:54:46] command: "make"
[01:54:46] stdout:
[01:54:46] ------------------------------------------
[01:54:46] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/rustdoc-error-lines'
[01:54:46] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --test input.rs > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output || true
[01:54:46] "/checkout/src/etc/cat-and-grep.sh" 'input.rs:7:15' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output
[01:54:46] [[[ begin stdout ]]]
[01:54:46] [90m
[01:54:46] running 1 test
[01:54:46] test input.rs - foo (line 6) ... FAILED
[01:54:46] 
[01:54:46] failures:
[01:54:46] 
[01:54:46] ---- input.rs - foo (line 6) stdout ----
[01:54:46] error[E0308]: mismatched types
[01:54:46]  --> input.rs:8:15
[01:54:46]   |
[01:54:46] 4 | let x: char = 1;
[01:54:46]   |               ^ expected char, found u8
[01:54:46] 
[01:54:46] error: aborting due to previous error
[01:54:46] 
[01:54:46] For more information about this error, try `rustc --explain E0308`.
[01:54:46] thread 'input.rs - foo (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:54:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:54:46] 
[01:54:46] 
[01:54:46] failures:
[01:54:46]     input.rs - foo (line 6)
[01:54:46] 
[01:54:46] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:54:46] 
[01:54:46] [0m
[01:54:46] [[[ end stdout ]]]
[01:54:46] [1;31mError: cannot match: input.rs:7:15[0m
[01:54:46] Makefile:7: recipe for target 'all' failed
