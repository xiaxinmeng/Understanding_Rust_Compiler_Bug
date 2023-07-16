

---- [debuginfo-lldb] tests/debuginfo/cross-crate-spans.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$0 = { 0 = 17 1 = 17 }
status: exit status: 0
command: PYTHONPATH="/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/cross-crate-spans.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/cross-crate-spans.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/a'
settings set auto-confirm true

version
lldb-1400.0.38.17 Apple Swift version 5.7.2 (swiftlang-5.7.2.135.5 clang-1400.0.29.51) 
command script import /Users/runner/work/rust/rust/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&(mut )?str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&(mut )?\[.+\]$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::([a-z_]+::)+)OsString$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Vec<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)VecDeque<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)BTreeSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)BTreeMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::([a-z_]+::)+)HashMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::([a-z_]+::)+)HashSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Rc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)Arc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)Cell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)Ref<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)RefMut<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::([a-z_]+::)+)RefCell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^core::num::([a-z_]+::)*NonZero.+$' --category Rust
type category enable Rust

b cross_crate_spans.rs:14
DEBUG: breakpoint added, id = 1
Breakpoint 1: 2 locations. 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback

run
Process 72175 stopped * thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000039f3 a`cross_crate_spans::generic_function::h11cdb9dbf64cb115(val=17) at cross_crate_spans.rs:14:5 11 let result = (val.clone(), val.clone()); 12 let a_variable: u32 = 123456789; 13 let another_variable: f64 = 123456789.5; -> 14 zzz(); ^ 15 result 16 } 17 Target 0: (a) stopped. Process 72175 launched: '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/a' (x86_64) 
print result
error: Couldn't materialize: couldn't get the value of variable result: no location, value may have been optimized out
error: errored out in DoExecute, couldn't PrepareToExecuteJITExpression

print a_variable
(unsigned int) $0 = 123456789 
print another_variable
(double) $1 = 123456789.5 
continue
print result
error: Couldn't materialize: couldn't get the value of variable result: no location, value may have been optimized out
error: errored out in DoExecute, couldn't PrepareToExecuteJITExpression

print a_variable
(unsigned int) $2 = 123456789 
print another_variable
(double) $3 = 123456789.5 
continue
quit
------------------------------------------
stderr: none



failures:
    [debuginfo-lldb] tests/debuginfo/cross-crate-spans.rs
