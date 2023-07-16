
---- [debuginfo-lldb] debuginfo/pretty-std-collections.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1200
NOTE: compiletest thinks it is using LLDB without native rust support

error: Error while running LLDB
status: exit code: 1
command: "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/pretty-std-collections.debugger.script"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: need to add support for DW_TAG_base_type '()' encoded with DW_ATE = 0x7, bit_size = 0
clangclang: CommandLine Error: Option 'h' registered more than once!
: CommandLine Error: Option 'LLVM ERROR: inconsistency in registered CommandLine options
h' registered more than once!
LLVM ERROR: inconsistency in registered CommandLine options

------------------------------------------



failures:
    [debuginfo-lldb] debuginfo/pretty-std-collections.rs

test result: FAILED. 89 passed; 1 failed; 26 ignored; 0 measured; 0 filtered out
