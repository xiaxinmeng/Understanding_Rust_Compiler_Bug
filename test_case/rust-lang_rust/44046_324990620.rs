
[01:45:13] ---- [debuginfo-lldb] debuginfo-lldb/generic-method-on-generic-struct.rs stdout ----
[01:45:13] 	NOTE: compiletest thinks it is using LLDB version 360
[01:45:13] 
[01:45:13] error: Error while running LLDB
[01:45:13] status: signal: 11
[01:45:13] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct.stage2-x86_64-apple-darwin" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct.debugger.script"
[01:45:13] stdout:
[01:45:13] ------------------------------------------
[01:45:13] LLDB batch-mode script
[01:45:13] ----------------------
[01:45:13] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct.debugger.script'.
[01:45:13] Target executable is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct.stage2-x86_64-apple-darwin'.
[01:45:13] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:45:13] Creating a target for '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct.stage2-x86_64-apple-darwin'
[01:45:13] settings set auto-confirm true
[01:45:13] 
[01:45:13] version
[01:45:13] lldb-360.1.70 
[01:45:13] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:45:13] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:45:13] type category enable Rust
[01:45:13] 
[01:45:13] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 131
[01:45:13] Breakpoint 1: 2 locations. 
[01:45:13] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 136
[01:45:13] Breakpoint 2: 2 locations. 
[01:45:13] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 141
[01:45:13] Breakpoint 3: where = generic-method-on-generic-struct.stage2-x86_64-apple-darwin`generic_method_on_generic_struct::{{impl}}::self_owned<f64,f32> + 47 at generic-method-on-generic-struct.rs:141, address = 0x00000001000016ff 
[01:45:13] quit
[01:45:13] 
[01:45:13] 
[01:45:13] ------------------------------------------
[01:45:13] stderr:
[01:45:13] ------------------------------------------
[01:45:13] 
[01:45:13] ------------------------------------------
[01:45:13] 
[01:45:13] thread '[debuginfo-lldb] debuginfo-lldb/generic-method-on-generic-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2435:8
[01:45:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:45:13] 
[01:45:13] 
[01:45:13] failures:
[01:45:13]     [debuginfo-lldb] debuginfo-lldb/generic-method-on-generic-struct.rs
[01:45:13] 
[01:45:13] test result: [31mFAILED(B[m. 97 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
[01:45:13] 
[01:45:13] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:322:21
