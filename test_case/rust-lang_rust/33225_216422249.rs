

failures:

---- [debuginfo-lldb] debuginfo-lldb/struct-namespace.rs stdout ----
    NOTE: compiletest thinks it is using LLDB version 330

error: line not found in debugger output: (struct_namespace::Struct1) $0 = [...]
status: exit code: 0
command: "/usr/bin/python" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/src/etc/lldb_batchmode.py" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/debuginfo/struct-namespace.stage2-x86_64-apple-darwin" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/debuginfo/struct-namespace.debugger.script"
stdout:
------------------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/debuginfo/struct-namespace.debugger.script'.
Target executable is '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/debuginfo/struct-namespace.stage2-x86_64-apple-darwin'.
Current working directory is '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj'
Creating a target for '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/debuginfo/struct-namespace.stage2-x86_64-apple-darwin'
settings set auto-confirm true

version
lldb-330.0.48 
command script import /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/./src/etc/lldb_rust_formatters.py
type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
type category enable Rust

breakpoint set --file 'struct-namespace.rs' --line 66
Breakpoint 1: where = struct-namespace.stage2-x86_64-apple-darwin`struct_namespace::main + 70 at struct-namespace.rs:66, address = 0x0000000100000e66 
run
Hit breakpoint 1.1: where = struct-namespace.stage2-x86_64-apple-darwin`struct_namespace::main + 70 at struct-namespace.rs:66, address = 0x0000000100000e66, resolved, hit count = 1 
Process 58804 stopped * thread #1: tid = 0x20d4ef9, 0x0000000100000e66 struct-namespace.stage2-x86_64-apple-darwin`struct_namespace::main + 70 at struct-namespace.rs:66, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000e66 struct-namespace.stage2-x86_64-apple-darwin`struct_namespace::main + 70 at struct-namespace.rs:66 63 64 let mod1_struct2 = mod1::Struct2(5); 65 -> 66 zzz(); // #break 67 } 68 69 #[inline(never)] Process 58804 launched: '/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/debuginfo/struct-namespace.stage2-x86_64-apple-darwin' (x86_64) 
p struct1
(struct_namespace::Struct1) $0 = Struct1 { a: 0, b: 1 } 
p struct2
(struct_namespace::Struct2) $1 = Struct2(2) 
p mod1_struct1
(struct_namespace::mod1::Struct1) $2 = Struct1 { a: 3, b: 4 } 
p mod1_struct2
(struct_namespace::mod1::Struct2) $3 = Struct2(5) 
quit


------------------------------------------
stderr:
------------------------------------------
2016-05-02 19:34:16.217 Python[58803:34426613] Metadata.framework [Error]: couldn't get the client port

------------------------------------------

thread '[debuginfo-lldb] debuginfo-lldb/struct-namespace.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/src/tools/compiletest/src/runtest.rs:1595
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [debuginfo-lldb] debuginfo-lldb/struct-namespace.rs

test result: FAILED. 91 passed; 1 failed; 10 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/src/tools/compiletest/src/main.rs:275


command did not execute successfully: "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2-tools/x86_64-apple-darwin/release/compiletest" "--compile-lib-path" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/src/test/debuginfo" "--aux-base" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/src/test/auxiliary" "--build-base" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/test/debuginfo" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "debuginfo-lldb" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath" "--target-rustcflags" "-Crpath -Lnative=/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt-rustbuild/build/obj/build/x86_64-apple-darwin/rust-test-helpers" "--android-cross-path" "" "--docck-python" "python" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-330.0.48" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" ""
expected success, got: exit code: 101

