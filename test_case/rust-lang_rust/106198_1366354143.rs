plain
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-apple-darwin target=x86_64-apple-darwin

failures:

---- [debuginfo-lldb] src/test/debuginfo/basic-types.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.0.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/basic-types.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/basic-types.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a'
settings set auto-confirm true
version
version
lldb-1400.0.30.3 Apple Swift version 5.7 (swiftlang-5.7.0.127.4 clang-1400.0.29.50) 
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

breakpoint set --file 'basic-types.rs' --line 154
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`basic_types::main::h00ef3525e79207b1 + 130 at basic-types.rs:154:5, address = 0x0000000100003b72 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
run
run
Process 80149 stopped * thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100003b72 a`basic_types::main::h00ef3525e79207b1 at basic-types.rs:154:5 151 let f32: f32 = 2.5; 152 let f64: f64 = 3.5; 153 let s: &str = "Hello, World!"; -> 154 _zzz(); // #break ^ 155 } 156 157 fn _zzz() { Target 0: (a) stopped. Process 80149 launched: '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a' (x86_64) 
print b
(bool) $0 = false 
print i
(long) $1 = -1 
print i8
(char) $2 = 'D' 
print i16
(short) $3 = -16 
print i32
(int) $4 = -32 
print i64
(long) $5 = -64 
print u
(unsigned long) $6 = 1 
print u8
(unsigned char) $7 = 'd' 
print u16
(unsigned short) $8 = 16 
print u32
(unsigned int) $9 = 32 
print u64
(unsigned long) $10 = 64 
print f32
(float) $11 = 2.5 
print f64
(double) $12 = 3.5 


TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 224, in <module>
    debugger.Terminate()
  File "/Applications/Xcode_14.0.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 4201, in Terminate
    return _lldb.SBDebugger_Terminate()
KeyboardInterrupt
------------------------------------------



---- [debuginfo-lldb] src/test/debuginfo/borrowed-unique-basic.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.0.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-unique-basic.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-unique-basic.lldb/borrowed-unique-basic.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-unique-basic.lldb/borrowed-unique-basic.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-unique-basic.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-unique-basic.lldb/a'
settings set auto-confirm true
version
version
lldb-1400.0.30.3 Apple Swift version 5.7 (swiftlang-5.7.0.127.4 clang-1400.0.29.50) 
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

breakpoint set --file 'borrowed-unique-basic.rs' --line 161
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`borrowed_unique_basic::main::h54fba867ae3d9746 + 2459 at borrowed-unique-basic.rs:161:5, address = 0x0000000100002f1b 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
type format add -f decimal char
type format add -f decimal char
type format add -f decimal 'unsigned char'
run
Process 80148 stopped * thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100002f1b a`borrowed_unique_basic::main::h54fba867ae3d9746 at borrowed-unique-basic.rs:161:5 158 let f64_box: Box<f64> = Box::new(3.5); 159 let f64_ref: &f64 = &*f64_box; 160 -> 161 zzz(); // #break ^ 162 } 163 164 fn zzz() {()} Target 0: (a) stopped. Process 80148 launched: '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-unique-basic.lldb/a' (x86_64) 
print *bool_ref
(bool) $0 = true 
print *int_ref
(long) $1 = -1 
print *i8_ref
(char) $2 = 68 
print *i16_ref
(short) $3 = -16 
print *i32_ref
(int) $4 = -32 
print *i64_ref
(long) $5 = -64 
print *uint_ref
(unsigned long) $6 = 1 
print *u8_ref
(unsigned char) $7 = 100 
print *u16_ref
(unsigned short) $8 = 16 
print *u32_ref
(unsigned int) $9 = 32 
print *u64_ref
(unsigned long) $10 = 64 
print *f32_ref
(float) $11 = 2.5 
print *f64_ref
(double) $12 = 3.5 


TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 224, in <module>
    debugger.Terminate()
  File "/Applications/Xcode_14.0.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 4201, in Terminate
    return _lldb.SBDebugger_Terminate()
KeyboardInterrupt
------------------------------------------



---- [debuginfo-lldb] src/test/debuginfo/borrowed-basic.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.0.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-basic.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-basic.lldb/borrowed-basic.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-basic.lldb/borrowed-basic.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-basic.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-basic.lldb/a'
settings set auto-confirm true
version
version
lldb-1400.0.30.3 Apple Swift version 5.7 (swiftlang-5.7.0.127.4 clang-1400.0.29.50) 
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

breakpoint set --file 'borrowed-basic.rs' --line 158
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`borrowed_basic::main::hb88626abe4f7b94a + 298 at borrowed-basic.rs:158:5, address = 0x0000000100003cba 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
run
run
Process 80147 stopped * thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100003cba a`borrowed_basic::main::hb88626abe4f7b94a at borrowed-basic.rs:158:5 155 let f64_val: f64 = 3.5; 156 let f64_ref: &f64 = &f64_val; 157 -> 158 zzz(); // #break ^ 159 } 160 161 fn zzz() {()} Target 0: (a) stopped. Process 80147 launched: '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-basic.lldb/a' (x86_64) 
print *bool_ref
(bool) $0 = true 
print *int_ref
(long) $1 = -1 
print *i8_ref
(char) $2 = 'D' 
print *i16_ref
(short) $3 = -16 
print *i32_ref
(int) $4 = -32 
print *i64_ref
(long) $5 = -64 
print *uint_ref
(unsigned long) $6 = 1 
print *u8_ref
(unsigned char) $7 = 'd' 
print *u16_ref
(unsigned short) $8 = 16 
print *u32_ref
(unsigned int) $9 = 32 
print *u64_ref
(unsigned long) $10 = 64 
print *f32_ref
(float) $11 = 2.5 
print *f64_ref
(double) $12 = 3.5 


TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 224, in <module>
    debugger.Terminate()
  File "/Applications/Xcode_14.0.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 4201, in Terminate
    return _lldb.SBDebugger_Terminate()
KeyboardInterrupt
------------------------------------------


