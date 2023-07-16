
[01:32:18] failures:
[01:32:18] 
[01:32:18] ---- [debuginfo-lldb] debuginfo/destructured-fn-argument.rs stdout ----
[01:32:18] 	NOTE: compiletest thinks it is using LLDB version 370
[01:32:18] 
[01:32:18] error: Error while running LLDB
[01:32:18] status: signal: 11
[01:32:18] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/destructured-fn-argument.stage2-i686-apple-darwin" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/destructured-fn-argument.debugger.script"
[01:32:18] stdout:
[01:32:18] ------------------------------------------
[01:32:18] LLDB batch-mode script
[01:32:18] ----------------------
[01:32:18] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/destructured-fn-argument.debugger.script'.
[01:32:18] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/destructured-fn-argument.stage2-i686-apple-darwin'.
[01:32:18] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:32:18] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/destructured-fn-argument.stage2-i686-apple-darwin'
[01:32:18] settings set auto-confirm true
[01:32:18] 
[01:32:18] version
[01:32:18] lldb-370.0.42 Swift-3.1 
[01:32:18] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:32:18] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:32:18] type category enable Rust
[01:32:18] 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 341
[01:32:18] Breakpoint 1: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::simple_tuple + 23 at destructured-fn-argument.rs:341, address = 0x000016a7 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 345
[01:32:18] Breakpoint 2: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::nested_tuple + 30 at destructured-fn-argument.rs:345, address = 0x000016de 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 349
[01:32:18] Breakpoint 3: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::destructure_only_first_level + 26 at destructured-fn-argument.rs:349, address = 0x0000170a 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 353
[01:32:18] Breakpoint 4: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::struct_as_tuple_element + 40 at destructured-fn-argument.rs:353, address = 0x00001748 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 357
[01:32:18] Breakpoint 5: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::struct_pattern + 24 at destructured-fn-argument.rs:357, address = 0x00001778 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 361
[01:32:18] Breakpoint 6: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::ignored_tuple_element + 20 at destructured-fn-argument.rs:361, address = 0x000017a4 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 365
[01:32:18] Breakpoint 7: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::ignored_struct_field + 15 at destructured-fn-argument.rs:365, address = 0x000017bf 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 369
[01:32:18] Breakpoint 8: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::one_struct_destructured_one_not + 40 at destructured-fn-argument.rs:369, address = 0x000017f8 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 373
[01:32:18] Breakpoint 9: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::different_order_of_struct_fields + 24 at destructured-fn-argument.rs:373, address = 0x00001828 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 379
[01:32:18] Breakpoint 10: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::complex_nesting + 78 at destructured-fn-argument.rs:379, address = 0x0000188e 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 383
[01:32:18] Breakpoint 11: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::managed_box + 26 at destructured-fn-argument.rs:383, address = 0x000018ba 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 387
[01:32:18] Breakpoint 12: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::borrowed_pointer + 26 at destructured-fn-argument.rs:387, address = 0x000018ea 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 391
[01:32:18] Breakpoint 13: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::contained_borrowed_pointer + 16 at destructured-fn-argument.rs:391, address = 0x00001910 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 395
[01:32:18] Breakpoint 14: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::unique_pointer + 30 at destructured-fn-argument.rs:395, address = 0x0000193e 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 399
[01:32:18] Breakpoint 15: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::ref_binding + 12 at destructured-fn-argument.rs:399, address = 0x0000198c 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 403
[01:32:18] Breakpoint 16: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::ref_binding_in_tuple + 24 at destructured-fn-argument.rs:403, address = 0x000019b8 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 407
[01:32:18] Breakpoint 17: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::ref_binding_in_struct + 15 at destructured-fn-argument.rs:407, address = 0x000019df 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 411
[01:32:18] Breakpoint 18: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::univariant_enum + 24 at destructured-fn-argument.rs:411, address = 0x00001a08 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 415
[01:32:18] Breakpoint 19: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::univariant_enum_with_ref_binding + 24 at destructured-fn-argument.rs:415, address = 0x00001a38 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 419
[01:32:18] Breakpoint 20: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::tuple_struct + 24 at destructured-fn-argument.rs:419, address = 0x00001a68 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 423

[01:32:18] Breakpoint 21: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::tuple_struct_with_ref_binding + 24 at destructured-fn-argument.rs:423, address = 0x00001a98 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 427
[01:32:18] Breakpoint 22: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::multiple_arguments + 32 at destructured-fn-argument.rs:427, address = 0x00001ad0 
[01:32:18] breakpoint set --file 'destructured-fn-argument.rs' --line 455
[01:32:18] Breakpoint 23: where = destructured-fn-argument.stage2-i686-apple-darwin`destructured_fn_argument::main::nested_function + 32 at destructured-fn-argument.rs:455, address = 0x000021a0 
[01:32:18] quit
[01:32:18] 
[01:32:18] 
[01:32:18] ------------------------------------------
[01:32:18] stderr:
[01:32:18] ------------------------------------------
[01:32:18] 
[01:32:18] ------------------------------------------
[01:32:18] 
[01:32:18] thread '[debuginfo-lldb] debuginfo/destructured-fn-argument.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2485:8
[01:32:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:32:18] 
[01:32:18] 
[01:32:18] failures:
[01:32:18]     [debuginfo-lldb] debuginfo/destructured-fn-argument.rs
[01:32:18] 
[01:32:18] test result: [31mFAILED(B[m. 97 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
[01:32:18] 
[01:32:18] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:329:21
