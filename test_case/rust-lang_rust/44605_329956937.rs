
[01:34:20] failures:
[01:34:20] 
[01:34:20] ---- [debuginfo-lldb] debuginfo-lldb/lexical-scope-in-unique-closure.rs stdout ----
[01:34:20] 	NOTE: compiletest thinks it is using LLDB version 360
[01:34:20] 
[01:34:20] error: Error while running LLDB
[01:34:20] status: signal: 11
[01:34:20] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/lexical-scope-in-unique-closure.stage2-i686-apple-darwin" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/lexical-scope-in-unique-closure.debugger.script"
[01:34:20] stdout:
[01:34:20] ------------------------------------------
[01:34:20] LLDB batch-mode script
[01:34:20] ----------------------
[01:34:20] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/lexical-scope-in-unique-closure.debugger.script'.
[01:34:20] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/lexical-scope-in-unique-closure.stage2-i686-apple-darwin'.
[01:34:20] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:34:20] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/lexical-scope-in-unique-closure.stage2-i686-apple-darwin'
[01:34:20] settings set auto-confirm true
[01:34:20] 
[01:34:20] version
[01:34:20] lldb-360.1.70 
[01:34:20] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:34:20] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:34:20] type category enable Rust
[01:34:20] 
[01:34:20] breakpoint set --file 'lexical-scope-in-unique-closure.rs' --line 80
[01:34:20] Breakpoint 1: where = lexical-scope-in-unique-closure.stage2-i686-apple-darwin`lexical_scope_in_unique_closure::main + 10 at lexical-scope-in-unique-closure.rs:80, address = 0x00001dea 
[01:34:20] breakpoint set --file 'lexical-scope-in-unique-closure.rs' --line 84
[01:34:20] Breakpoint 2: where = lexical-scope-in-unique-closure.stage2-i686-apple-darwin`lexical_scope_in_unique_closure::main::{{closure}} + 30 at lexical-scope-in-unique-closure.rs:84, address = 0x00001e4e 
[01:34:20] breakpoint set --file 'lexical-scope-in-unique-closure.rs' --line 89
[01:34:20] Breakpoint 3: where = lexical-scope-in-unique-closure.stage2-i686-apple-darwin`lexical_scope_in_unique_closure::main::{{closure}} + 59 at lexical-scope-in-unique-closure.rs:89, address = 0x00001e6b 
[01:34:20] breakpoint set --file 'lexical-scope-in-unique-closure.rs' --line 94
[01:34:20] Breakpoint 4: where = lexical-scope-in-unique-closure.stage2-i686-apple-darwin`lexical_scope_in_unique_closure::main::{{closure}} + 73 at lexical-scope-in-unique-closure.rs:94, address = 0x00001e79 
[01:34:20] breakpoint set --file 'lexical-scope-in-unique-closure.rs' --line 98
[01:34:20] Breakpoint 5: where = lexical-scope-in-unique-closure.stage2-i686-apple-darwin`lexical_scope_in_unique_closure::main + 20 at lexical-scope-in-unique-closure.rs:98, address = 0x00001df4 
[01:34:20] breakpoint set --file 'lexical-scope-in-unique-closure.rs' --line 103
[01:34:20] Breakpoint 6: where = lexical-scope-in-unique-closure.stage2-i686-apple-darwin`lexical_scope_in_unique_closure::main + 55 at lexical-scope-in-unique-closure.rs:103, address = 0x00001e17 
[01:34:20] quit
[01:34:20] 
[01:34:20] 
[01:34:20] ------------------------------------------
[01:34:20] stderr:
[01:34:20] ------------------------------------------
[01:34:20] 
[01:34:20] ------------------------------------------
[01:34:20] 
[01:34:20] thread '[debuginfo-lldb] debuginfo-lldb/lexical-scope-in-unique-closure.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2435:8
[01:34:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:34:20] 
[01:34:20] 
[01:34:20] failures:
[01:34:20]     [debuginfo-lldb] debuginfo-lldb/lexical-scope-in-unique-closure.rs
[01:34:20] 
[01:34:20] test result: [31mFAILED(B[m. 97 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
