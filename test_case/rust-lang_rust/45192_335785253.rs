
[01:46:37] ---- [debuginfo-lldb] debuginfo/by-value-non-immediate-argument.rs stdout ----
[01:46:37] 	NOTE: compiletest thinks it is using LLDB version 360
[01:46:37] 
[01:46:37] error: Error while running LLDB
[01:46:37] status: signal: 11
[01:46:37] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/by-value-non-immediate-argument.stage2-i686-apple-darwin" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/by-value-non-immediate-argument.debugger.script"
[01:46:37] stdout:
[01:46:37] ------------------------------------------
[01:46:37] LLDB batch-mode script
[01:46:37] ----------------------
[01:46:37] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/by-value-non-immediate-argument.debugger.script'.
[01:46:37] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/by-value-non-immediate-argument.stage2-i686-apple-darwin'.
[01:46:37] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:46:37] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/by-value-non-immediate-argument.stage2-i686-apple-darwin'
[01:46:37] settings set auto-confirm true
[01:46:37] 
[01:46:37] version
[01:46:37] lldb-360.1.70 
[01:46:37] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:46:37] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:46:37] type category enable Rust
[01:46:37] 
[01:46:37] breakpoint set --file 'by-value-non-immediate-argument.rs' --line 94
[01:46:37] Breakpoint 1: where = by-value-non-immediate-argument.stage2-i686-apple-darwin`by_value_non_immediate_argument::fun + 42 at by-value-non-immediate-argument.rs:94, address = 0x00001afa 
[01:46:37] breakpoint set --file 'by-value-non-immediate-argument.rs' --line 98
[01:46:37] Breakpoint 2: where = by-value-non-immediate-argument.stage2-i686-apple-darwin`by_value_non_immediate_argument::fun_fun + 72 at by-value-non-immediate-argument.rs:98, address = 0x00001b58 
[01:46:37] breakpoint set --file 'by-value-non-immediate-argument.rs' --line 102
[01:46:37] Breakpoint 3: where = by-value-non-immediate-argument.stage2-i686-apple-darwin`by_value_non_immediate_argument::tup + 79 at by-value-non-immediate-argument.rs:102, address = 0x00001bbf 
[01:46:37] breakpoint set --file 'by-value-non-immediate-argument.rs' --line 108
[01:46:37] Breakpoint 4: where = by-value-non-immediate-argument.stage2-i686-apple-darwin`by_value_non_immediate_argument::new_type + 79 at by-value-non-immediate-argument.rs:108, address = 0x00001c1f 
[01:46:37] breakpoint set --file 'by-value-non-immediate-argument.rs' --line 120
[01:46:37] Breakpoint 5: where = by-value-non-immediate-argument.stage2-i686-apple-darwin`by_value_non_immediate_argument::by_val_enum + 79 at by-value-non-immediate-argument.rs:120, address = 0x00001c7f 
[01:46:37] quit
[01:46:37] 
[01:46:37] 
[01:46:37] ------------------------------------------
[01:46:37] stderr:
[01:46:37] ------------------------------------------
[01:46:37] 
[01:46:37] ------------------------------------------
[01:46:37] 
[01:46:37] thread '[debuginfo-lldb] debuginfo/by-value-non-immediate-argument.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2425:8
[01:46:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:46:37] 
[01:46:37] 
[01:46:37] failures:
[01:46:37]     [debuginfo-lldb] debuginfo/by-value-non-immediate-argument.rs
[01:46:37] 
[01:46:37] test result: [31mFAILED(B[m. 97 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
[01:46:37] 
[01:46:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:323:21
