plain
##[section]Starting: macOS x86_64-apple
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 24'
Agent machine name: 'Mac-1400'
Current agent version: '2.168.2'
##[group]Operating System
Mac OS X
19E287
19E287
##[endgroup]
##[group]Virtual Environment
Environment: macos-10.15
Version: 20200507.1
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
[command]/bin/bash --noprofile --norc /Users/runner/runners/2.168.2/work/_temp/25873ea1-e853-4609-9638-da44355f4e93.sh

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
   Compiling rustc_ast_pretty v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_hir)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_session v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_session)
   Compiling rustc_query_system v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_query_system)
   Compiling rustc_parse v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_parse)
   Compiling rustc_hir_pretty v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_hir_pretty)
   Compiling rustc_ast_lowering v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/Users/runner/runners/2.168.2/work/1/s/src/librustc_ast_passes)
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
.................................................................i.................................. 1800/10189
.................................................................................................... 1900/10189
....................................................................................i..i............ 2000/10189
.................................................................................................... 2100/10189
..........................................................................iiiii..................... 2200/10189
.................................................................................................... 2400/10189
.................................................................................................... 2500/10189
.................................................................................................... 2600/10189
.................................................................................................... 2700/10189
---
......i............................................................................................. 5200/10189
.................................................................................................... 5300/10189
.....................................i.............................................................. 5400/10189
.................................................................................................... 5500/10189
.......................................ii.ii........i...i........................................... 5600/10189
.........................................................................................i.......... 5800/10189
.................................................................................................... 5900/10189
....................................ii.....................................i........................ 6000/10189
.................................................................................................... 6100/10189
.................................................................................................... 6100/10189
....i............................................................................................... 6200/10189
.................................................................................................ii. 6300/10189
..i..ii...........i................................................................................. 6400/10189
.................................................................................................... 6600/10189
.................................................................................................... 6700/10189
.................................................................................................... 6700/10189
..............................i..ii................................................................. 6800/10189
.......................i.......................................................ii..i................ 7000/10189
....................................................................................i............... 7100/10189
.................................................................................................... 7200/10189
.................................................................................................... 7300/10189
---
.................................................................................................... 8100/10189
.................................................................................................... 8200/10189
.................................................................................................... 8300/10189
.......i............................................................................................ 8400/10189
...............................................................iiii...i............................. 8500/10189
.................................................................................................... 8700/10189
.................................................................................................... 8800/10189
.................................................................................................... 8900/10189
.................................................................................................... 9000/10189
---
 finished in 12.557
Check compiletest suite=codegen mode=codegen (x86_64-apple-darwin -> x86_64-apple-darwin)

running 189 tests
i......................i.i..........i........i......................i...................ii.....i.... 100/189
........i...i....ii..iiiiiiii...iiiii.........................i..................i.......

 finished in 6.318
Check compiletest suite=codegen-units mode=codegen-units (x86_64-apple-darwin -> x86_64-apple-darwin)

---
 finished in 2.297
Check compiletest suite=assembly mode=assembly (x86_64-apple-darwin -> x86_64-apple-darwin)

running 9 tests
iiiiiiiii

 finished in 0.053
Check compiletest suite=incremental mode=incremental (x86_64-apple-darwin -> x86_64-apple-darwin)

---
 finished in 23.031
Check compiletest suite=debuginfo mode=debuginfo (x86_64-apple-darwin -> x86_64-apple-darwin)

running 116 tests
i.......i.....i.F.........iFF..i..ii..i..i..ii.i..F..........i.i.ii.FF.....iFF...i.FiiFF...i.FF.iF.i 100/116
F..FFiiFi..FFFFF

---- [debuginfo-lldb] debuginfo/c-style-enum-in-composite.rs stdout ----

error: compilation failed!
error: compilation failed!
status: exit code: 1
command: "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.168.2/work/1/s/src/test/debuginfo/c-style-enum-in-composite.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "-C" "prefer-dynamic" "-o" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a" "-Crpath" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-g" "-L" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [debuginfo-lldb] debuginfo/empty-string.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]empty_string = "" { vec = size=0 }
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/empty-string.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`empty_string::main::h16dc11aef3d9e163 + 36 at empty-string.rs:33:5, address = 0x0000000100001344, resolved, hit count = 1 
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
Breakpoint 1: where = a`empty_string::main::h16dc11aef3d9e163 + 36 at empty-string.rs:33:5, address = 0x0000000100001344 
run
Process 62835 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001344 a`empty_string::main::h16dc11aef3d9e163 at empty-string.rs:33:5 30 31 let empty_str = ""; 32 -> 33 zzz(); // #break ^ 34 } 35 36 fn zzz() {} Target 0: (a) stopped. Process 62835 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a' (x86_64) 
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


Hit breakpoint 1.1: where = a`enum_thinlto::f::h427be9c8ef855fd8 + 12 at enum-thinlto.rs:42:5, address = 0x000000010000166c, resolved, hit count = 1 
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
Breakpoint 1: where = a`enum_thinlto::f::h427be9c8ef855fd8 + 12 at enum-thinlto.rs:42:5, address = 0x000000010000166c 
run
Process 62857 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x000000010000166c a`enum_thinlto::f::h427be9c8ef855fd8(abc=0x00007ffeefbfd338) at enum-thinlto.rs:42:5 39 fn f(abc: &ABC) { 40 zzz(); // #break 41 -> 42 println!("{:?}", abc); ^ 43 } 44 45 fn zzz() {()} Target 0: (a) stopped. Process 62857 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a' (x86_64) 
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
Process 63190 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000020ce a`issue_22656::main::h34c4c6bd35d83e23 at issue-22656.rs:46:5 43 w: 456 44 }; 45 -> 46 zzz(); // #break ^ 47 } 48 49 fn zzz() { () } Target 0: (a) stopped. Process 63190 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a' (x86_64) 
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

Hit breakpoint 3.1: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_owned::h3f75cb974f59a9f0 + 28 at method-on-trait.rs:153:9, address = 0x000000010000142c, resolved, hit count = 1 
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
Breakpoint 3: where = a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_owned::h3f75cb974f59a9f0 + 28 at method-on-trait.rs:153:9, address = 0x000000010000142c 
run
Process 63522 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001310 a`_$LT$method_on_trait..Struct$u20$as$u20$method_on_trait..Trait$GT$::self_by_ref::h540d69ae28ebadf2(self=0x00007ffeefbfd2d8, arg1=-1, arg2=-2) at method-on-trait.rs:143:9 140 impl Trait for Struct { 141 142 fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize { -> 143 zzz(); // #break ^ 144 self.x + arg1 + arg2 145 } 146 Target 0: (a) stopped. Process 63522 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-trait.lldb/a' (x86_64) 
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


Hit breakpoint 1.1: where = a`method_on_tuple_struct::TupleStruct::self_by_ref::h2a274b69ba63f60e + 28 at method-on-tuple-struct.rs:135:9, address = 0x000000010000135c, resolved, hit count = 1 

Hit breakpoint 2.1: where = a`method_on_tuple_struct::TupleStruct::self_by_val::he19bfa7f3091c844 + 33 at method-on-tuple-struct.rs:140:9, address = 0x00000001000013c1, resolved, hit count = 1 

Hit breakpoint 1.1: where = a`method_on_tuple_struct::TupleStruct::self_by_ref::h2a274b69ba63f60e + 28 at method-on-tuple-struct.rs:135:9, address = 0x000000010000135c, resolved, hit count = 2 

Hit breakpoint 2.1: where = a`method_on_tuple_struct::TupleStruct::self_by_val::he19bfa7f3091c844 + 33 at method-on-tuple-struct.rs:140:9, address = 0x00000001000013c1, resolved, hit count = 2 

Hit breakpoint 3.1: where = a`method_on_tuple_struct::TupleStruct::self_owned::h2362073a01c85449 + 28 at method-on-tuple-struct.rs:145:9, address = 0x000000010000141c, resolved, hit count = 1 
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
Breakpoint 1: where = a`method_on_tuple_struct::TupleStruct::self_by_ref::h2a274b69ba63f60e + 28 at method-on-tuple-struct.rs:135:9, address = 0x000000010000135c 
breakpoint set --file 'method-on-tuple-struct.rs' --line 140
Breakpoint 2: where = a`method_on_tuple_struct::TupleStruct::self_by_val::he19bfa7f3091c844 + 33 at method-on-tuple-struct.rs:140:9, address = 0x00000001000013c1 
breakpoint set --file 'method-on-tuple-struct.rs' --line 145
Breakpoint 3: where = a`method_on_tuple_struct::TupleStruct::self_owned::h2362073a01c85449 + 28 at method-on-tuple-struct.rs:145:9, address = 0x000000010000141c 
run
Process 63545 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x000000010000135c a`method_on_tuple_struct::TupleStruct::self_by_ref::h2a274b69ba63f60e(self=0x00007ffeefbfd2c0, arg1=-1, arg2=-2) at method-on-tuple-struct.rs:135:9 132 impl TupleStruct { 133 134 fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize { -> 135 zzz(); // #break ^ 136 arg1 + arg2 137 } 138 Target 0: (a) stopped. Process 63545 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-tuple-struct.lldb/a' (x86_64) 
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

breakpoint set --file 'packed-struct-with-destructor.rs' --line 257
breakpoint set --file 'packed-struct-with-destructor.rs' --line 257
Breakpoint 1: where = a`packed_struct_with_destructor::main::hff115f5ddf1be034 + 2033 at packed-struct-with-destructor.rs:257:5, address = 0x00000001000017d1 
run
Process 63678 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000017d1 a`packed_struct_with_destructor::main::hff115f5ddf1be034 at packed-struct-with-destructor.rs:257:5 254 } 255 }; 256 -> 257 zzz(); // #break ^ 258 } 259 260 fn zzz() {()} Target 0: (a) stopped. Process 63678 launched: '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct-with-destructor.lldb/a' (x86_64) 
print packed
(packed_struct_with_destructor::Packed) $0 = { x = 123 y = 234 z = 345 } 
print packedInPacked
(packed_struct_with_destructor::PackedInPacked) $1 = { a = 1111 b = { x = 2222 y = 3333 z = 4444 } c = 5555 d = { x = 6666 y = 7777 z = 8888 } } 
print packedInUnpacked
(packed_struct_with_destructor::PackedInUnpacked) $2 = { a = -1111 b = { x = -2222 y = -3333 z = -4444 } c = -5555 d = { x = -6666 y = -7777 z = -8888 } } 
print unpackedInPacked
(packed_struct_with_destructor::UnpackedInPacked) $3 = { a = 987 b = { x = 876 y = 765 z = 654 } c = { x = 543 y = 432 z = 321 } d = 210 } 
print packedInPackedWithDrop
(packed_struct_with_destructor::PackedInPackedWithDrop) $4 = { a = 11 b = { x = 22 y = 33 z = 44 } c = 55 d = { x = 66 y = 77 z = 88 } } 
print packedInUnpackedWithDrop
(packed_struct_with_destructor::PackedInUnpackedWithDrop) $5 = { a = -11 b = { x = -22 y = -33 z = -44 } c = -55 d = { x = -66 y = -77 z = -88 } } 
print unpackedInPackedWithDrop
(packed_struct_with_destructor::UnpackedInPackedWithDrop) $6 = { a = 98 b = { x = 87 y = 76 z = 65 } c = { x = 54 y = 43 z = 32 } d = 21 } 
print deeplyNested
(packed_struct_with_destructor::DeeplyNested) $7 = { a = { a = 1 b = { x = 2 y = 3 z = 4 } c = 5 d = { x = 6 y = 7 z = 8 } } b = { a = 9 b = { x = 10 y = 11 z = 12 } c = { x = 13 y = 14 z = 15 } d = 16 } c = { a = 17 b = { x = 18 y = 19 z = 20 } c = 21 d = { x = 22 y = 23 z = 24 } } d = { a = 25 b = { x = 26 y = 27 z = 28 } c = 29 d = { x = 30 y = 31 z = 32 } } e = { a = 33 b = { x = 34 y = 35 z = 36 } c = { x = 37 y = 38 z = 39 } d = 40 } f = { a = 41 b = { x = 42 y = 43 z = 44 } c = 45 d = { x = 46 y = 47 z = 48 } } } 


------------------------------------------
stderr:
stderr:
------------------------------------------
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:148: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  watchdog_start_time = clock()
/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py:152: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
  while clock() < watchdog_max_time:
------------------------------------------


---- [debuginfo-lldb] debuginfo/packed-struct.rs stdout ----
---- [debuginfo-lldb] debuginfo/packed-struct.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1103
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$0 = Packed { x: 123, y: 234, z: 345 }
status: exit code: 0
command: "/usr/bin/python3" "/Users/runner/runners/2.168.2/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct.lldb/a" "/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct.lldb/packed-struct.debugger.script"
------------------------------------------


Hit breakpoint 1.1: where = a`packed_struct::main::h93f55ab4a9698d73 + 415 at packed-struct.rs:130:5, address = 0x0000000100000c6f, resolved, hit count = 1 
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct.lldb/packed-struct.debugger.script'.
Target executable is '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct.lldb/a'.
Current working directory is '/Users/runner/runners/2.168.2/work/1/s'
Creating a target for '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/debuginfo/packed-struct.lldb/a'
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
