
[01:45:44] ---- [debuginfo-lldb] debuginfo-lldb/static-method-on-struct-and-enum.rs stdout ----
[01:45:44] 	NOTE: compiletest thinks it is using LLDB version 360
[01:45:44] 
[01:45:44] error: Error while running LLDB
[01:45:44] status: signal: 11
[01:45:44] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/static-method-on-struct-and-enum.stage2-i686-apple-darwin" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/static-method-on-struct-and-enum.debugger.script"
[01:45:44] stdout:
[01:45:44] ------------------------------------------
[01:45:44] LLDB batch-mode script
[01:45:44] ----------------------
[01:45:44] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/static-method-on-struct-and-enum.debugger.script'.
[01:45:44] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/static-method-on-struct-and-enum.stage2-i686-apple-darwin'.
[01:45:44] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:45:44] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/static-method-on-struct-and-enum.stage2-i686-apple-darwin'
[01:45:44] settings set auto-confirm true
[01:45:44] 
[01:45:44] version
[01:45:44] lldb-360.1.70 
[01:45:44] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:45:44] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:45:44] type category enable Rust
[01:45:44] 
[01:45:44] breakpoint set --file 'static-method-on-struct-and-enum.rs' --line 66
[01:45:44] Breakpoint 1: where = static-method-on-struct-and-enum.stage2-i686-apple-darwin`static_method_on_struct_and_enum::{{impl}}::static_method + 36 at static-method-on-struct-and-enum.rs:66, address = 0x00001d14 
[01:45:44] breakpoint set --file 'static-method-on-struct-and-enum.rs' --line 80
[01:45:44] Breakpoint 2: where = static-method-on-struct-and-enum.stage2-i686-apple-darwin`static_method_on_struct_and_enum::{{impl}}::static_method + 50 at static-method-on-struct-and-enum.rs:80, address = 0x00001d82 
[01:45:44] quit
[01:45:44] 
[01:45:44] 
[01:45:44] ------------------------------------------
[01:45:44] stderr:
[01:45:44] ------------------------------------------
[01:45:44] 
[01:45:44] ------------------------------------------
[01:45:44] 
[01:45:44] thread '[debuginfo-lldb] debuginfo-lldb/static-method-on-struct-and-enum.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2435:8
[01:45:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:45:44] 
[01:45:44] 
[01:45:44] failures:
[01:45:44]     [debuginfo-lldb] debuginfo-lldb/static-method-on-struct-and-enum.rs
[01:45:44] 
[01:45:44] test result: [31mFAILED(B[m. 97 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
[01:45:44] 
[01:45:44] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:323:21
