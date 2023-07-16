plain
test [debuginfo-lldb] src/test/debuginfo/vec.rs ... ok

failures:

---- [debuginfo-lldb] src/test/debuginfo/numeric-types.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 1300
NOTE: compiletest thinks it is using LLDB without native rust support

error: line not found in debugger output: [...]$0 = 11
status: exit status: 0
command: "/usr/bin/python3" "/Users/runner/work/rust/rust/src/etc/lldb_batchmode.py" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/numeric-types.lldb/a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/numeric-types.lldb/numeric-types.debugger.script"
--- stdout -------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/numeric-types.lldb/numeric-types.debugger.script'.
Target executable is '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/numeric-types.lldb/a'.
Current working directory is '/Users/runner/work/rust/rust'
Creating a target for '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/numeric-types.lldb/a'
settings set auto-confirm true
version
version
lldb-1300.0.42.3 Swift version 5.5.2-dev 
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

breakpoint set --file 'numeric-types.rs' --line 288
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`numeric_types::main::h0759cd9c4a18020f + 963 at numeric-types.rs:288:5, address = 0x00000001000030f3 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback
run
run
Process 96821 stopped * thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000030f3 a`numeric_types::main::h0759cd9c4a18020f at numeric-types.rs:288:5 285 let a_u64 = AtomicU64::new(512); 286 let a_usize = AtomicUsize::new(1024); 287 -> 288 zzz(); // #break ^ 289 } 290 291 fn zzz() { } Target 0: (a) stopped. Process 96821 launched: '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/debuginfo/numeric-types.lldb/a' (x86_64) 
print nz_i8
(core::num::nonzero::NonZeroI8) $0 = '\v' { __0 = '\v' } 
print nz_i16
(core::num::nonzero::NonZeroI16) $1 = 22 { __0 = 22 } 
print nz_i32
(core::num::nonzero::NonZeroI32) $2 = 33 { __0 = 33 } 
print nz_i64
(core::num::nonzero::NonZeroI64) $3 = 44 { __0 = 44 } 
print nz_i128
(core::num::nonzero::NonZeroI128) $4 = 55 { __0 = 55 } 
print nz_isize
(core::num::nonzero::NonZeroIsize) $5 = 66 { __0 = 66 } 
print nz_u8
(core::num::nonzero::NonZeroU8) $6 = 'M' { __0 = 'M' } 
print nz_u16
(core::num::nonzero::NonZeroU16) $7 = 88 { __0 = 88 } 
print nz_u32
(core::num::nonzero::NonZeroU32) $8 = 99 { __0 = 99 } 
print nz_u64
(core::num::nonzero::NonZeroU64) $9 = 100 { __0 = 100 } 
print nz_u128
(core::num::nonzero::NonZeroU128) $10 = 111 { __0 = 111 } 
print nz_usize
(core::num::nonzero::NonZeroUsize) $11 = 122 { __0 = 122 } 
------------------------------------------
stderr: none


