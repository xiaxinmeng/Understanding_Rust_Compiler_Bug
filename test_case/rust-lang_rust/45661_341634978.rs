
[00:57:34] ---- [debuginfo-gdb] debuginfo/vec.rs stdout ----
[00:57:34] 	NOTE: compiletest thinks it is using GDB without native rust support
[00:57:34] NOTE: compiletest thinks it is using GDB version 7011001
[00:57:34] thread '[debuginfo-gdb] debuginfo/vec.rs' panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [47, 99, 104, 101, 99, 107, 111, 117, 116, 47, 111, 98, 106, 47, 98, 117, 105, 108, 100, 47, 120, 56, 54, 95, 54, 52, 45, 117, 110, 107, 110, 111, 119, 110, 45, 108, 105, 110, 117, 120, 45, 103, 110, 117, 47, 116, 101, 115, 116, 47, 100, 101, 98, 117, 103, 105, 110, 102, 111, 47, 118, 101, 99, 46, 100, 101, 98, 117, 103, 103, 101, 114, 46, 115, 99, 114, 105, 112, 116, 58, 55, 58, 32, 69, 114, 114, 111, 114, 32, 105, 110, 32, 115, 111, 117, 114, 99, 101, 100, 32, 99, 111, 109, 109, 97, 110, 100, 32, 102, 105, 108, 101, 58, 10, 248, 236, 46, 249, 67, 127, 10], error: Utf8Error { valid_up_to: 114, error_len: Some(1) } }', /checkout/src/libcore/result.rs:906:4
[00:57:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:34] 
[00:57:34] 
[00:57:34] failures:
[00:57:34]     [debuginfo-gdb] debuginfo/vec.rs
[00:57:34] 
[00:57:34] test result: [31mFAILED(B[m. 104 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
