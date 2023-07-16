plain
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-apple-darwin target=x86_64-apple-darwin

failures:

---- [debuginfo-lldb] tests/debuginfo/box.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/box.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/box.lldb/box.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/box.lldb/box.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/box.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/box.lldb/a'
settings set auto-confirm true
version
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

breakpoint set --file 'box.rs' --line 34
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`box::main::hffbe036b935e8df9 + 270 at box.rs:34:5, address = 0x000000010000306e 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
run
run
TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 216, in <module>
    execute_command(command_interpreter, command)
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 78, in execute_command
    command_interpreter.HandleCommand(command, res)
  File "/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 3166, in HandleCommand
    return _lldb.SBCommandInterpreter_HandleCommand(self, *args)
KeyboardInterrupt
------------------------------------------



---- [debuginfo-lldb] tests/debuginfo/borrowed-struct.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/borrowed-struct.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/borrowed-struct.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/a'
settings set auto-confirm true
version
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

breakpoint set --file 'borrowed-struct.rs' --line 86
Breakpoint 1: where = a`borrowed_struct::main::h46b5418b8d5b3e31 + 245 at borrowed-struct.rs:86:5, address = 0x00000001000033f5 
DEBUG: breakpoint added, id = 1
run
TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 216, in <module>
    execute_command(command_interpreter, command)
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 78, in execute_command
    command_interpreter.HandleCommand(command, res)
  File "/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 3166, in HandleCommand
    return _lldb.SBCommandInterpreter_HandleCommand(self, *args)
KeyboardInterrupt
------------------------------------------



---- [debuginfo-lldb] tests/debuginfo/borrowed-basic.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-basic.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-basic.lldb/borrowed-basic.debugger.script"
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

breakpoint set --file 'borrowed-basic.rs' --line 158
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`borrowed_basic::main::hb88626abe4f7b94a + 298 at borrowed-basic.rs:158:5, address = 0x0000000100003cba 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
run
run
TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 216, in <module>
    execute_command(command_interpreter, command)
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 78, in execute_command
    command_interpreter.HandleCommand(command, res)
  File "/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 3166, in HandleCommand
    return _lldb.SBCommandInterpreter_HandleCommand(self, *args)
KeyboardInterrupt
------------------------------------------



---- [debuginfo-lldb] tests/debuginfo/borrowed-c-style-enum.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-c-style-enum.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-c-style-enum.lldb/borrowed-c-style-enum.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-c-style-enum.lldb/borrowed-c-style-enum.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-c-style-enum.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-c-style-enum.lldb/a'
settings set auto-confirm true
version
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

breakpoint set --file 'borrowed-c-style-enum.rs' --line 53
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`borrowed_c_style_enum::main::h25d1c13ddf8858ea + 44 at borrowed-c-style-enum.rs:53:5, address = 0x0000000100003b9c 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
run
run
TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 216, in <module>
    execute_command(command_interpreter, command)
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 78, in execute_command
    command_interpreter.HandleCommand(command, res)
  File "/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 3166, in HandleCommand
    return _lldb.SBCommandInterpreter_HandleCommand(self, *args)
KeyboardInterrupt
------------------------------------------



---- [debuginfo-lldb] tests/debuginfo/borrowed-unique-basic.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-unique-basic.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-unique-basic.lldb/borrowed-unique-basic.debugger.script"
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

breakpoint set --file 'borrowed-unique-basic.rs' --line 161
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`borrowed_unique_basic::main::h54fba867ae3d9746 + 2456 at borrowed-unique-basic.rs:161:5, address = 0x0000000100002f58 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
type format add -f decimal char
type format add -f decimal char
type format add -f decimal 'unsigned char'
run
TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 216, in <module>
    execute_command(command_interpreter, command)
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 78, in execute_command
    command_interpreter.HandleCommand(command, res)
  File "/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 3166, in HandleCommand
    return _lldb.SBCommandInterpreter_HandleCommand(self, *args)
KeyboardInterrupt
------------------------------------------



---- [debuginfo-lldb] tests/debuginfo/borrowed-tuple.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/borrowed-tuple.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/borrowed-tuple.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/a'
settings set auto-confirm true
version
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

breakpoint set --file 'borrowed-tuple.rs' --line 51
Breakpoint 1: where = a`borrowed_tuple::main::h2a63676140b98dc8 + 156 at borrowed-tuple.rs:51:5, address = 0x000000010000396c 
DEBUG: breakpoint added, id = 1
run
TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 216, in <module>
    execute_command(command_interpreter, command)
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 78, in execute_command
    command_interpreter.HandleCommand(command, res)
  File "/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 3166, in HandleCommand
    return _lldb.SBCommandInterpreter_HandleCommand(self, *args)
KeyboardInterrupt
------------------------------------------



---- [debuginfo-lldb] tests/debuginfo/basic-types.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1400
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
error: Error while running LLDB
status: signal: 2 (SIGINT)
command: PYTHONPATH="/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" PYTHONUNBUFFERED="1" "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/basic-types.lldb/basic-types.debugger.script"
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

breakpoint set --file 'basic-types.rs' --line 154
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`basic_types::main::h00ef3525e79207b1 + 130 at basic-types.rs:154:5, address = 0x0000000100003b72 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
run
run
TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 216, in <module>
    execute_command(command_interpreter, command)
  File "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py", line 78, in execute_command
    command_interpreter.HandleCommand(command, res)
  File "/Applications/Xcode_14.2.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 3166, in HandleCommand
    return _lldb.SBCommandInterpreter_HandleCommand(self, *args)
KeyboardInterrupt
------------------------------------------


