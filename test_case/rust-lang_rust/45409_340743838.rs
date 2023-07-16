
[01:34:24] ---- [debuginfo-lldb] debuginfo/associated-types.rs stdout ----
[01:34:24] 	NOTE: compiletest thinks it is using LLDB version 370
[01:34:24] 
[01:34:24] error: Error while running LLDB
[01:34:24] status: signal: 11
[01:34:24] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types.stage2-x86_64-apple-darwin" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types.debugger.script"
[01:34:24] stdout:
[01:34:24] ------------------------------------------
[01:34:24] LLDB batch-mode script
[01:34:24] ----------------------
[01:34:24] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types.debugger.script'.
[01:34:24] Target executable is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types.stage2-x86_64-apple-darwin'.
[01:34:24] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:34:24] Creating a target for '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types.stage2-x86_64-apple-darwin'
[01:34:24] settings set auto-confirm true
[01:34:24] 
[01:34:24] version
[01:34:24] lldb-370.0.42 Swift-3.1 
[01:34:24] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:34:24] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:34:24] type category enable Rust
[01:34:24] 
[01:34:24] breakpoint set --file 'associated-types.rs' --line 110
[01:34:24] Breakpoint 1: where = associated-types.stage2-x86_64-apple-darwin`associated_types::assoc_struct<i32> + 4 at associated-types.rs:110, address = 0x0000000100000764 
[01:34:24] breakpoint set --file 'associated-types.rs' --line 117
[01:34:24] Breakpoint 2: where = associated-types.stage2-x86_64-apple-darwin`associated_types::assoc_local<i32> + 74 at associated-types.rs:117, address = 0x00000001000007fa 
[01:34:24] breakpoint set --file 'associated-types.rs' --line 121
[01:34:24] Breakpoint 3: where = associated-types.stage2-x86_64-apple-darwin`associated_types::assoc_arg<i32> + 12 at associated-types.rs:121, address = 0x000000010000084c 
[01:34:24] breakpoint set --file 'associated-types.rs' --line 129
[01:34:24] Breakpoint 4: where = associated-types.stage2-x86_64-apple-darwin`associated_types::assoc_tuple<i32> + 4 at associated-types.rs:129, address = 0x00000001000008d4 
[01:34:24] breakpoint set --file 'associated-types.rs' --line 136
[01:34:24] Breakpoint 5: where = associated-types.stage2-x86_64-apple-darwin`associated_types::assoc_enum<i32> + 102 at associated-types.rs:136, address = 0x0000000100000976 
[01:34:24] breakpoint set --file 'associated-types.rs' --line 139
[01:34:24] Breakpoint 6: where = associated-types.stage2-x86_64-apple-darwin`associated_types::assoc_enum<i32> + 143 at associated-types.rs:139, address = 0x000000010000099f 
[01:34:24] quit
[01:34:24] 
[01:34:24] 
[01:34:24] ------------------------------------------
[01:34:24] stderr:
[01:34:24] ------------------------------------------
[01:34:24] 
[01:34:24] ------------------------------------------
[01:34:24] 
[01:34:24] thread '[debuginfo-lldb] debuginfo/associated-types.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2485:8
[01:34:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:34:24] 
[01:34:24] 
[01:34:24] failures:
[01:34:24]     [debuginfo-lldb] debuginfo/associated-types.rs
[01:34:24] 
[01:34:24] test result: [31mFAILED(B[m. 97 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
