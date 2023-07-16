

failures:

---- [debuginfo-lldb] debuginfo/vec-slices.rs stdout ----
    NOTE: compiletest thinks it is using LLDB version 310

error: line not found in debugger output: [...]$6 = &mut [64, 65]
status: exit code: 0
command: "/usr/bin/python2.7" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/etc/lldb_batchmode.py" "x86_64-apple-darwin/test/debuginfo-lldb/vec-slices.stage2-x86_64-apple-darwin" "x86_64-apple-darwin/test/debuginfo-lldb/vec-slices.debugger.script"
stdout:
------------------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is 'x86_64-apple-darwin/test/debuginfo-lldb/vec-slices.debugger.script'.
Target executable is 'x86_64-apple-darwin/test/debuginfo-lldb/vec-slices.stage2-x86_64-apple-darwin'.
Current working directory is '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/obj'
Creating a target for 'x86_64-apple-darwin/test/debuginfo-lldb/vec-slices.stage2-x86_64-apple-darwin'
settings set auto-confirm true

version
lldb-310.2.37 
command script import /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/./src/etc/lldb_rust_formatters.py
type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
type category enable Rust

breakpoint set --line 112
Breakpoint 1: where = vec-slices.stage2-x86_64-apple-darwin`vec_slices::main + 206 at vec-slices.rs:112, address = 0x0000000100000a4e 
run
Hit breakpoint 1.1: where = vec-slices.stage2-x86_64-apple-darwin`vec_slices::main + 206 at vec-slices.rs:112, address = 0x0000000100000a4e, resolved, hit count = 1 
Process 39175 launched: '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/obj/x86_64-apple-darwin/test/debuginfo-lldb/vec-slices.stage2-x86_64-apple-darwin' (x86_64) 
print empty
(&[i64]) $0 = &[] 
print singleton
(&[i64]) $1 = &[1] 
print multiple
(&[i64]) $2 = &[2, 3, 4, 5] 
print slice_of_slice
(&[i64]) $3 = &[3, 4] 
print padded_tuple
(&[(i32, i16)]) $4 = &[(6, 7), (8, 9)] 
print padded_struct
(&[vec_slices::AStruct]) $5 = &[AStruct { x: 10, y: 11, z: 12 }, AStruct { x: 13, y: 14, z: 15 }] 
print 'vec_slices::MUT_VECT_SLICE'
(int) $6 = 1279869765 
quit


------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[debuginfo-lldb] debuginfo/vec-slices.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/compiletest/runtest.rs:1505
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [debuginfo-lldb] debuginfo/vec-slices.rs

test result: FAILED. 90 passed; 1 failed; 10 ignored; 0 measured
