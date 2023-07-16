plain
##[section]Starting: macOS x86_64-apple
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 60'
Agent machine name: 'Mac-1561'
Current agent version: '2.168.2'
##[group]Operating System
Mac OS X
19E287
19E287
##[endgroup]
##[group]Virtual Environment
Environment: macos-10.15
Version: 20200514.1
Included Software: https://github.com/actions/virtual-environments/blob/master/images/macos/macos-10.15-Readme.md
##[endgroup]
Agent running as: 'runner'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /Users/runner/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /Users/runner/runners/2.168.2/work/_temp/fbcefb09-72ca-4159-bb1d-26f60f352944.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72357/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72357/merge:refs/remotes/pull/72357/merge
---
   Compiling rustc_feature v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_hir)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_query_system v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_parse v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_parse)
   Compiling rustc_hir_pretty v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_hir_pretty)
   Compiling rustc_ast_lowering v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_ast_lowering)
---
.......................................................................i............................ 1800/10200
.................................................................................................... 1900/10200
..........................................................................................i..i...... 2000/10200
.................................................................................................... 2100/10200
................................................................................iiiii............... 2200/10200
.................................................................................................... 2400/10200
.................................................................................................... 2500/10200
.................................................................................................... 2600/10200
.................................................................................................... 2700/10200
---
............i....................................................................................... 5200/10200
.................................................................................................... 5300/10200
...........................................i........................................................ 5400/10200
.................................................................................................... 5500/10200
.............................................ii.ii........i...i..................................... 5600/10200
...............................................................................................i.... 5800/10200
.................................................................................................... 5900/10200
...............................................ii.....................................i............. 6000/10200
.................................................................................................... 6100/10200
.................................................................................................... 6100/10200
...............i.................................................................................... 6200/10200
.................................................................................................... 6300/10200
........ii...i..ii...........i...................................................................... 6400/10200
.................................................................................................... 6600/10200
.................................................................................................... 6700/10200
.................................................................................................... 6700/10200
.........................................i..ii...................................................... 6800/10200
..................................i.......................................................ii..i..... 7000/10200
...............................................................................................i.... 7100/10200
.................................................................................................... 7200/10200
.................................................................................................... 7300/10200
---
.................................................................................................... 8100/10200
.................................................................................................... 8200/10200
.................................................................................................... 8300/10200
..................i................................................................................. 8400/10200
..........................................................................iiii...i.................. 8500/10200
.................................................................................................... 8700/10200
.................................................................................................... 8800/10200
.................................................................................................... 8900/10200
.................................................................................................... 9000/10200
---
 finished in 12.278
Check compiletest suite=codegen mode=codegen (x86_64-apple-darwin -> x86_64-apple-darwin)

running 190 tests
i.......................i.i..........i........i......................i...................ii.....i... 100/190
.........i...i....ii..iiiiiiii...iiiii.........................i..................i.......

 finished in 5.966
Check compiletest suite=codegen-units mode=codegen-units (x86_64-apple-darwin -> x86_64-apple-darwin)

---
 finished in 2.212
Check compiletest suite=assembly mode=assembly (x86_64-apple-darwin -> x86_64-apple-darwin)

running 20 tests
........iiiiiiiii...

 finished in 0.572
Check compiletest suite=incremental mode=incremental (x86_64-apple-darwin -> x86_64-apple-darwin)

---
 finished in 22.685
Check compiletest suite=debuginfo mode=debuginfo (x86_64-apple-darwin -> x86_64-apple-darwin)

running 116 tests
i.......i.....i.F.........iFF..i..ii..i..i..ii.i..F..........i.i.ii.FF.....iFF...i.Fii.....i.F..iF.i 100/116
F...Fii.i..FFFFF

---- [debuginfo-lldb] debuginfo/c-style-enum-in-composite.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$1  = { 0 = { 0 = 1 1 = OneThousand } 1 = 2 }
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/c-style-enum-in-composite.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`c_style_enum_in_composite::main::h215c7f6b7ead7eea + 206 at c-style-enum-in-composite.rs:155:5, address = 0x0000000100000b0e, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/c-style-enum-in-composite.debugger.script'.
Target executable is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a'.
Current working directory is '/Users/runner/runners/2.168.2/work/1/s'
Creating a target for '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a'
settings set auto-confirm true
version
version
lldb-1103.0.22.8 Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51) 
command script import /Users/runner/runners/2.168.2/work/1/s/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::(\w+::)+)OsString$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Vec<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)VecDeque<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Rc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Arc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Cell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Ref<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefMut<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefCell<.+>$' --category Rust


breakpoint set --file 'c-style-enum-in-composite.rs' --line 155
Breakpoint 1: where = a`c_style_enum_in_composite::main::h215c7f6b7ead7eea + 206 at c-style-enum-in-composite.rs:155:5, address = 0x0000000100000b0e 
run
Process 62500 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000b0e a`c_style_enum_in_composite::main::h215c7f6b7ead7eea at c-style-enum-in-composite.rs:155:5 152 153 let struct_with_drop = (StructWithDrop { a: OneHundred, b: Vienna }, 9_i64); 154 -> 155 zzz(); // #break ^ 156 } 157 158 fn zzz() { () } Target 0: (a) stopped. Process 62500 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a' (x86_64) 
print tuple_interior_padding
((i16, c_style_enum_in_composite::AnEnum)) $0 = { 0 = 0 1 = OneHundred } 
print tuple_padding_at_end
(((u64, c_style_enum_in_composite::AnEnum), u64)) $1 = { 0 = { 0 = 1 1 = OneThousand } 1 = 2 } 
print tuple_different_enums
((c_style_enum_in_composite::AnEnum, c_style_enum_in_composite::AnotherEnum, c_style_enum_in_composite::AnEnum, c_style_enum_in_composite::AnotherEnum)) $2 = { 0 = OneThousand 1 = MountainView 2 = OneMillion 3 = Vienna } 
print padded_struct
(c_style_enum_in_composite::PaddedStruct) $3 = { a = 3 b = OneMillion c = 4 d = Toronto e = 5 } 
print packed_struct
(c_style_enum_in_composite::PackedStruct) $4 = { a = 6 b = OneHundred c = 7 d = Vienna e = 8 } 
print non_padded_struct
(c_style_enum_in_composite::NonPaddedStruct) $5 = { a = OneMillion b = MountainView c = OneThousand d = Toronto } 
print struct_with_drop
((c_style_enum_in_composite::StructWithDrop, i64)) $6 = { 0 = { a = OneHundred b = Vienna } 1 = 9 } 


------------------------------------------
stderr:
stderr:
------------------------------------------
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:148: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  watchdog_start_time = clock()
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:152: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  while clock() < watchdog_max_time:
------------------------------------------


---- [debuginfo-lldb] debuginfo/empty-string.rs stdout ----
---- [debuginfo-lldb] debuginfo/empty-string.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]empty_string = "" { vec = size=0 }
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/empty-string.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`empty_string::main::h16dc11aef3d9e163 + 36 at empty-string.rs:33:5, address = 0x0000000100001034, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/empty-string.debugger.script'.
Target executable is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a'.
Current working directory is '/Users/runner/runners/2.168.2/work/1/s'
Creating a target for '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a'
settings set auto-confirm true
version
version
lldb-1103.0.22.8 Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51) 
command script import /Users/runner/runners/2.168.2/work/1/s/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::(\w+::)+)OsString$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Vec<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)VecDeque<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Rc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Arc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Cell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Ref<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefMut<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefCell<.+>$' --category Rust


breakpoint set --file 'empty-string.rs' --line 33
Breakpoint 1: where = a`empty_string::main::h16dc11aef3d9e163 + 36 at empty-string.rs:33:5, address = 0x0000000100001034 
run
Process 62717 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001034 a`empty_string::main::h16dc11aef3d9e163 at empty-string.rs:33:5 30 31 let empty_str = ""; 32 -> 33 zzz(); // #break ^ 34 } 35 36 fn zzz() {} Target 0: (a) stopped. Process 62717 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a' (x86_64) 
fr v empty_string
(alloc::string::String) empty_string = { vec = {} } 
fr v empty_str
(&str) empty_str = "" { data_ptr = 0x00000001000017d0 "" length = 0 } 


------------------------------------------
stderr:
stderr:
------------------------------------------
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:148: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  watchdog_start_time = clock()
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:152: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  while clock() < watchdog_max_time:
------------------------------------------


---- [debuginfo-lldb] debuginfo/enum-thinlto.rs stdout ----
---- [debuginfo-lldb] debuginfo/enum-thinlto.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: (enum_thinlto::ABC) $0 = ABC { }
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/enum-thinlto.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`enum_thinlto::f::h427be9c8ef855fd8 + 12 at enum-thinlto.rs:42:5, address = 0x000000010000162c, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/enum-thinlto.debugger.script'.
Target executable is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a'.
Current working directory is '/Users/runner/runners/2.168.2/work/1/s'
Creating a target for '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a'
settings set auto-confirm true
version
version
lldb-1103.0.22.8 Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51) 
command script import /Users/runner/runners/2.168.2/work/1/s/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::(\w+::)+)OsString$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Vec<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)VecDeque<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Rc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Arc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Cell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Ref<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefMut<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefCell<.+>$' --category Rust

breakpoint set --file 'enum-thinlto.rs' --line 40
breakpoint set --file 'enum-thinlto.rs' --line 40
Breakpoint 1: where = a`enum_thinlto::f::h427be9c8ef855fd8 + 12 at enum-thinlto.rs:42:5, address = 0x000000010000162c 
run
Process 62739 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x000000010000162c a`enum_thinlto::f::h427be9c8ef855fd8(abc=0x00007ffeefbfd338) at enum-thinlto.rs:42:5 39 fn f(abc: &ABC) { 40 zzz(); // #break 41 -> 42 println!("{:?}", abc); ^ 43 } 44 45 fn zzz() {()} Target 0: (a) stopped. Process 62739 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a' (x86_64) 
print *abc
(enum_thinlto::ABC) $0 = 


------------------------------------------
stderr:
stderr:
------------------------------------------
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:148: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  watchdog_start_time = clock()
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:152: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  while clock() < watchdog_max_time:
------------------------------------------


---- [debuginfo-lldb] debuginfo/issue-22656.rs stdout ----
---- [debuginfo-lldb] debuginfo/issue-22656.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$0 = size=3 { [0] = 1 [1] = 2 [2] = 3 }
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/issue-22656.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`issue_22656::main::h34c4c6bd35d83e23 + 78 at issue-22656.rs:46:5, address = 0x00000001000020ce, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/issue-22656.debugger.script'.
Target executable is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a'.
Current working directory is '/Users/runner/runners/2.168.2/work/1/s'
Creating a target for '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a'
settings set auto-confirm true
version
version
lldb-1103.0.22.8 Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51) 
command script import /Users/runner/runners/2.168.2/work/1/s/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::(\w+::)+)OsString$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Vec<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)VecDeque<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Rc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Arc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Cell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Ref<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefMut<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefCell<.+>$' --category Rust

breakpoint set --file 'issue-22656.rs' --line 46
breakpoint set --file 'issue-22656.rs' --line 46
Breakpoint 1: where = a`issue_22656::main::h34c4c6bd35d83e23 + 78 at issue-22656.rs:46:5, address = 0x00000001000020ce 
run
Process 63074 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000020ce a`issue_22656::main::h34c4c6bd35d83e23 at issue-22656.rs:46:5 43 w: 456 44 }; 45 -> 46 zzz(); // #break ^ 47 } 48 49 fn zzz() { () } Target 0: (a) stopped. Process 63074 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a' (x86_64) 
print v
(alloc::vec::Vec<int>) $0 = { [0] = 1 [1] = 2 [2] = 3 } 
print zs
(issue_22656::StructWithZeroSizedField) $1 = { x = y = 123 z = w = 456 } 


------------------------------------------
stderr:
stderr:
------------------------------------------
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:148: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  watchdog_start_time = clock()
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:152: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  while clock() < watchdog_max_time:
------------------------------------------


---- [debuginfo-lldb] debuginfo/method-on-trait.rs stdout ----
---- [debuginfo-lldb] debuginfo/method-on-trait.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$0 = Struct { x: 100 }
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-trait.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-trait.lldb/method-on-trait.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_by_ref::h540d69ae28ebadf2 + 32 at method-on-trait.rs:143:9, address = 0x0000000100001310, resolved, hit count = 1 

Hit breakpoint 2.1: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_by_val::h4ad246b05578f827 + 32 at method-on-trait.rs:148:9, address = 0x00000001000013a0, resolved, hit count = 1 

Hit breakpoint 1.1: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_by_ref::h540d69ae28ebadf2 + 32 at method-on-trait.rs:143:9, address = 0x0000000100001310, resolved, hit count = 2 

Hit breakpoint 2.1: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_by_val::h4ad246b05578f827 + 32 at method-on-trait.rs:148:9, address = 0x00000001000013a0, resolved, hit count = 2 

Hit breakpoint 3.1: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_owned::hf36434a330f9681f + 28 at method-on-trait.rs:153:9, address = 0x000000010000142c, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-trait.lldb/method-on-trait.debugger.script'.
Target executable is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-trait.lldb/a'.
Current working directory is '/Users/runner/runners/2.168.2/work/1/s'
Creating a target for '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-trait.lldb/a'
settings set auto-confirm true
version
version
lldb-1103.0.22.8 Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51) 
command script import /Users/runner/runners/2.168.2/work/1/s/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::(\w+::)+)OsString$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Vec<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)VecDeque<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Rc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Arc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Cell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Ref<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefMut<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefCell<.+>$' --category Rust


breakpoint set --file 'method-on-trait.rs' --line 143
Breakpoint 1: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_by_ref::h540d69ae28ebadf2 + 32 at method-on-trait.rs:143:9, address = 0x0000000100001310 
breakpoint set --file 'method-on-trait.rs' --line 148
Breakpoint 2: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_by_val::h4ad246b05578f827 + 32 at method-on-trait.rs:148:9, address = 0x00000001000013a0 
breakpoint set --file 'method-on-trait.rs' --line 153
Breakpoint 3: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_owned::hf36434a330f9681f + 28 at method-on-trait.rs:153:9, address = 0x000000010000142c 
run
Process 63406 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001310 a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_by_ref::h540d69ae28ebadf2(self=0x00007ffeefbfd2d8, arg1=-1, arg2=-2) at method-on-trait.rs:143:9 140 impl Trait for Struct { 141 142 fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize { -> 143 zzz(); // #break ^ 144 self.x + arg1 + arg2 145 } 146 Target 0: (a) stopped. Process 63406 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-trait.lldb/a' (x86_64) 
print *self
(method_on_trait::Struct) $0 = { x = 100 } 
(long) $1 = -1 
print arg2
(long) $2 = -2 
continue
continue
print self
(method_on_trait::Struct) $3 = { x = 100 } 
(long) $4 = -3 
print arg2
(long) $5 = -4 
continue
continue
print *self
(method_on_trait::Struct) $6 = { x = 200 } 
(long) $7 = -5 
print arg2
(long) $8 = -6 
continue
continue
print self
(method_on_trait::Struct) $9 = { x = 200 } 
(long) $10 = -7 
print arg2
(long) $11 = -8 
continue
continue
print *self
(method_on_trait::Struct) $12 = { x = 200 } 
(long) $13 = -9 
print arg2
(long) $14 = -10 
continue
continue
quit


------------------------------------------
stderr:
------------------------------------------
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:148: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  watchdog_start_time = clock()
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:152: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  while clock() < watchdog_max_time:
------------------------------------------


---- [debuginfo-lldb] debuginfo/method-on-tuple-struct.rs stdout ----
---- [debuginfo-lldb] debuginfo/method-on-tuple-struct.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$0 = TupleStruct(100, -100.5)
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-tuple-struct.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-tuple-struct.lldb/method-on-tuple-struct.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`method_on_tuple_struct::TupleStruct::self_by_ref::h2a274b69ba63f60e + 28 at method-on-tuple-struct.rs:135:9, address = 0x0000000100000bec, resolved, hit count = 1 

Hit breakpoint 2.1: where = a`method_on_tuple_struct::TupleStruct::self_by_val::he19bfa7f3091c844 + 33 at method-on-tuple-struct.rs:140:9, address = 0x0000000100000c51, resolved, hit count = 1 

Hit breakpoint 1.1: where = a`method_on_tuple_struct::TupleStruct::self_by_ref::h2a274b69ba63f60e + 28 at method-on-tuple-struct.rs:135:9, address = 0x0000000100000bec, resolved, hit count = 2 

Hit breakpoint 2.1: where = a`method_on_tuple_struct::TupleStruct::self_by_val::he19bfa7f3091c844 + 33 at method-on-tuple-struct.rs:140:9, address = 0x0000000100000c51, resolved, hit count = 2 

Hit breakpoint 3.1: where = a`method_on_tuple_struct::TupleStruct::self_owned::h762b6eb920f0fa78 + 28 at method-on-tuple-struct.rs:145:9, address = 0x0000000100000cac, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-tuple-struct.lldb/method-on-tuple-struct.debugger.script'.
Target executable is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-tuple-struct.lldb/a'.
Current working directory is '/Users/runner/runners/2.168.2/work/1/s'
Creating a target for '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-tuple-struct.lldb/a'
settings set auto-confirm true
version
version
lldb-1103.0.22.8 Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51) 
command script import /Users/runner/runners/2.168.2/work/1/s/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::(\w+::)+)OsString$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Vec<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)VecDeque<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)BTreeMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashMap<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::collections::(\w+::)+)HashSet<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Rc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)Arc<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Cell<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)Ref<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefMut<.+>$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(core::(\w+::)+)RefCell<.+>$' --category Rust


breakpoint set --file 'method-on-tuple-struct.rs' --line 135
Breakpoint 1: where = a`method_on_tuple_struct::TupleStruct::self_by_ref::h2a274b69ba63f60e + 28 at method-on-tuple-struct.rs:135:9, address = 0x0000000100000bec 
breakpoint set --file 'method-on-tuple-struct.rs' --line 140
Breakpoint 2: where = a`method_on_tuple_struct::TupleStruct::self_by_val::he19bfa7f3091c844 + 33 at method-on-tuple-struct.rs:140:9, address = 0x0000000100000c51 
breakpoint set --file 'method-on-tuple-struct.rs' --line 145
Breakpoint 3: where = a`method_on_tuple_struct::TupleStruct::self_owned::h762b6eb920f0fa78 + 28 at method-on-tuple-struct.rs:145:9, address = 0x0000000100000cac 
run
Process 63429 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000bec a`method_on_tuple_struct::TupleStruct::self_by_ref::h2a274b69ba63f60e(self=0x00007ffeefbfd2c0, arg1=-1, arg2=-2) at method-on-tuple-struct.rs:135:9 132 impl TupleStruct { 133 134 fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize { -> 135 zzz(); // #break ^ 136 arg1 + arg2 137 } 138 Target 0: (a) stopped. Process 63429 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-tuple-struct.lldb/a' (x86_64) 
print *self
(method_on_tuple_struct::TupleStruct) $0 = { 0 = 100 1 = -100.5 } 
(long) $1 = -1 
print arg2
(long) $2 = -2 
continue
continue
print self
(method_on_tuple_struct::TupleStruct) $3 = { 0 = 100 1 = -100.5 } 
(long) $4 = -3 
print arg2
(long) $5 = -4 
continue
continue
print *self
(method_on_tuple_struct::TupleStruct) $6 = { 0 = 200 1 = -200.5 } 
(long) $7 = -5 
print arg2
(long) $8 = -6 
continue
continue
print self
(method_on_tuple_struct::TupleStruct) $9 = { 0 = 200 1 = -200.5 } 
(long) $10 = -7 
print arg2
(long) $11 = -8 
continue
continue
print *self
(method_on_tuple_struct::TupleStruct) $12 = { 0 = 200 1 = -200.5 } 
(long) $13 = -9 
print arg2
(long) $14 = -10 
continue
continue
quit


------------------------------------------
stderr:
------------------------------------------
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:148: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  watchdog_start_time = clock()
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:152: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  while clock() < watchdog_max_time:
------------------------------------------


---- [debuginfo-lldb] debuginfo/packed-struct-with-destructor.rs stdout ----
---- [debuginfo-lldb] debuginfo/packed-struct-with-destructor.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$0 = Packed { x: 123, y: 234, z: 345 }
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct-with-destructor.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct-with-destructor.lldb/packed-struct-with-destructor.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`packed_struct_with_destructor::main::hff115f5ddf1be034 + 2033 at packed-struct-with-destructor.rs:257:5, address = 0x00000001000017d1, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct-with-destructor.lldb/packed-struct-with-destructor.debugger.script'.
Target executable is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct-with-destructor.lldb/a'.
Current working directory is '/Users/runner/runners/2.168.2/work/1/s'
Creating a target for '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct-with-destructor.lldb/a'
settings set auto-confirm true
version
version
lldb-1103.0.22.8 Apple Swift version 5.2.2 (swiftlang-1103.0.32.6 clang-1103.0.32.51) 
command script import /Users/runner/runners/2.168.2/work/1/s/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::(\w+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(std::ffi::(\w+::)+)OsString$' --category Rust
