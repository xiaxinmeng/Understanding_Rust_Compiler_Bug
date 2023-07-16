
failures:

---- [debuginfo-lldb] debuginfo/pretty-slices.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1205
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output:  (&mut [i32]) $1 = size=4 { [0] = 2 [1] = 3 [2] = 5 [3] = 7 }
status: exit status: 0
command: "/usr/bin/python3" "/Users/t1mb0sl1c3/Developer/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/t1mb0sl1c3/Developer/rust-lang/rust/compiler/build/x86_64-apple-darwin/test/debuginfo/pretty-slices.lldb/a" "/Users/t1mb0sl1c3/Developer/rust-lang/rust/compiler/build/x86_64-apple-darwin/test/debuginfo/pretty-slices.lldb/pretty-slices.debugger.script"
stdout:
------------------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/t1mb0sl1c3/Developer/rust-lang/rust/compiler/build/x86_64-apple-darwin/test/debuginfo/pretty-slices.lldb/pretty-slices.debugger.script'.
Target executable is '/Users/t1mb0sl1c3/Developer/rust-lang/rust/compiler/build/x86_64-apple-darwin/test/debuginfo/pretty-slices.lldb/a'.
Current working directory is '/Users/t1mb0sl1c3/Developer/rust-lang/rust/compiler'
Creating a target for '/Users/t1mb0sl1c3/Developer/rust-lang/rust/compiler/build/x86_64-apple-darwin/test/debuginfo/pretty-slices.lldb/a'
settings set auto-confirm true

version
lldb-1205.0.27.3 Apple Swift version 5.4 (swiftlang-1205.0.26.9 clang-1205.0.19.55) 
command script import /Users/t1mb0sl1c3/Developer/rust-lang/rust/./src/etc/lldb_lookup.py
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
type category enable Rust

breakpoint set --file 'pretty-slices.rs' --line 45
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`pretty_slices::main::h926e671f9dfe1c03 + 155 at pretty-slices.rs:45:5, address = 0x0000000100002cab 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback

run
Process 39336 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100002cab a`pretty_slices::main::h926e671f9dfe1c03 at pretty-slices.rs:45:5 42 let mut mut_str_slice_buffer = String::from("mutable string slice"); 43 let mut_str_slice: &mut str = mut_str_slice_buffer.as_mut_str(); 44 -> 45 b(); // #break ^ 46 } Target 0: (a) stopped. Process 39336 launched: '/Users/t1mb0sl1c3/Developer/rust-lang/rust/compiler/build/x86_64-apple-darwin/test/debuginfo/pretty-slices.lldb/a' (x86_64) 
print slice
(&[i32]) $0 = size=3 { [0] = 0 [1] = 1 [2] = 2 } 
print mut_slice
(&mut &[i32]) $1 = { data_ptr = 0x00007ffeefbff288 length = 4 } 
print str_slice
(&str) $2 = "string slice" { data_ptr = 0x0000000100003e58 "string slicemutable string slicecalled `Result::unwrap()` on an `Err` value" length = 12 } 
print mut_str_slice
(&mut &str) $3 = { data_ptr = 0x0000000100606070 "mutable string slice" length = 20 } 
quit


------------------------------------------
stderr:
------------------------------------------

------------------------------------------



failures:
    [debuginfo-lldb] debuginfo/pretty-slices.rs

test result: FAILED. 1 passed; 1 failed; 131 ignored; 0 measured; 0 filtered out; finished in 4.69s

Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-apple-darwin target=x86_64-apple-darwin
