
failures:

---- [debuginfo-lldb] debuginfo/pretty-std-collections.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1200
NOTE: compiletest thinks it is using LLDB without native rust support

error: Error while running LLDB
status: exit code: 1
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-apple-darwin target=x86_64-apple-darwin
command: "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/pretty-std-collections.debugger.script"
stdout:
------------------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/pretty-std-collections.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a'
settings set auto-confirm true

version
lldb-1200.0.32.1 Apple Swift version 5.3 (swiftlang-1200.0.29.2 clang-1200.0.30.1) 
command script import /Users/runner/work/rust/rust/./src/etc/lldb_lookup.py
type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^(alloc::([a-z_]+::)+)String$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&str$' --category Rust
type summary add -F lldb_lookup.summary_lookup  -e -x -h '^&\[.+\]$' --category Rust
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
type category enable Rust

breakpoint set --file 'pretty-std-collections.rs' --line 158
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`pretty_std_collections::main::h1ef1d7ce5967b6a7 + 1513 at pretty-std-collections.rs:158:5, address = 0x000000010003d3d9 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback

run
Process 66288 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x000000010003d3d9 a`pretty_std_collections::main::h1ef1d7ce5967b6a7 at pretty-std-collections.rs:158:5 155 hash_set.insert(i); 156 } 157 -> 158 zzz(); // #break ^ 159 } 160 161 fn zzz() { Target 0: (a) stopped. Process 66288 launched: '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/pretty-std-collections.lldb/a' (x86_64) 
print vec_deque
(alloc::collections::vec_deque::VecDeque<int>) $0 = size=3 { [0] = 5 [1] = 3 [2] = 7 } 
print vec_deque2
(alloc::collections::vec_deque::VecDeque<int>) $1 = size=7 { [0] = 2 [1] = 3 [2] = 4 [3] = 5 [4] = 6 [5] = 7 [6] = 8 } 
print hash_map

------------------------------------------
stderr:
------------------------------------------
error: need to add support for DW_TAG_base_type '()' encoded with DW_ATE = 0x7, bit_size = 0
clang: CommandLine Error: Option 'h' registered more than once!
LLVM ERROR: inconsistency in registered CommandLine options

------------------------------------------



failures:
    [debuginfo-lldb] debuginfo/pretty-std-collections.rs

test result: FAILED. 89 passed; 1 failed; 26 ignored; 0 measured; 0 filtered out
