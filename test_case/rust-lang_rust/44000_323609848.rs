
[01:39:56] failures:
[01:39:56] 
[01:39:56] ---- [debuginfo-lldb] debuginfo-lldb/name-shadowing-and-scope-nesting.rs stdout ----
[01:39:56] 	NOTE: compiletest thinks it is using LLDB version 360
[01:39:56] 
[01:39:56] error: Error while running LLDB
[01:39:56] status: signal: 11
[01:39:56] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/name-shadowing-and-scope-nesting.debugger.script"
[01:39:56] stdout:
[01:39:56] ------------------------------------------
[01:39:56] LLDB batch-mode script
[01:39:56] ----------------------
[01:39:56] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/name-shadowing-and-scope-nesting.debugger.script'.
[01:39:56] Target executable is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin'.
[01:39:56] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:39:56] Creating a target for '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin'
[01:39:56] settings set auto-confirm true
[01:39:56] 
[01:39:56] version
[01:39:56] lldb-360.1.70 
[01:39:56] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:39:56] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:39:56] type category enable Rust
[01:39:56] 
[01:39:56] breakpoint set --file 'name-shadowing-and-scope-nesting.rs' --line 103
[01:39:56] Breakpoint 1: where = name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin`name_shadowing_and_scope_nesting::main + 16 at name-shadowing-and-scope-nesting.rs:103, address = 0x0000000100000df0 
[01:39:56] breakpoint set --file 'name-shadowing-and-scope-nesting.rs' --line 108
[01:39:56] Breakpoint 2: where = name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin`name_shadowing_and_scope_nesting::main + 33 at name-shadowing-and-scope-nesting.rs:108, address = 0x0000000100000e01 
[01:39:56] breakpoint set --file 'name-shadowing-and-scope-nesting.rs' --line 114
[01:39:56] Breakpoint 3: where = name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin`name_shadowing_and_scope_nesting::main + 63 at name-shadowing-and-scope-nesting.rs:114, address = 0x0000000100000e1f 
[01:39:56] breakpoint set --file 'name-shadowing-and-scope-nesting.rs' --line 121
[01:39:56] Breakpoint 4: where = name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin`name_shadowing_and_scope_nesting::main + 84 at name-shadowing-and-scope-nesting.rs:121, address = 0x0000000100000e34 
[01:39:56] breakpoint set --file 'name-shadowing-and-scope-nesting.rs' --line 126
[01:39:56] Breakpoint 5: where = name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin`name_shadowing_and_scope_nesting::main + 107 at name-shadowing-and-scope-nesting.rs:126, address = 0x0000000100000e4b 
[01:39:56] breakpoint set --file 'name-shadowing-and-scope-nesting.rs' --line 130
[01:39:56] Breakpoint 6: where = name-shadowing-and-scope-nesting.stage2-x86_64-apple-darwin`name_shadowing_and_scope_nesting::main + 117 at name-shadowing-and-scope-nesting.rs:130, address = 0x0000000100000e55 
[01:39:56] quit
[01:39:56] 
[01:39:56] 
[01:39:56] ------------------------------------------
[01:39:56] stderr:
[01:39:56] ------------------------------------------
[01:39:56] 
[01:39:56] ------------------------------------------
[01:39:56] 
[01:39:56] thread '[debuginfo-lldb] debuginfo-lldb/name-shadowing-and-scope-nesting.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2511:8
[01:39:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:39:56] 
[01:39:56] 
[01:39:56] failures:
[01:39:56]     [debuginfo-lldb] debuginfo-lldb/name-shadowing-and-scope-nesting.rs
