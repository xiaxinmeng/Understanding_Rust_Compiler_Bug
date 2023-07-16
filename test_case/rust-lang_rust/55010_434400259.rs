plain
[00:03:36]       Memory: 8 GB
[00:03:36]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:36]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:36]       SMC Version (system): 2.8f0
[00:03:36]       Serial Number (system): VMQMjgmTWfc5
[00:03:36] 
[00:03:36] hw.ncpu: 4
[00:03:36] hw.byteorder: 1234
[00:03:36] hw.memsize: 8589934592
---
[01:29:08] test [debuginfo-both] debuginfo/vec.rs ... ok
[01:29:08] 
[01:29:08] failures:
[01:29:08] 
[01:29:08] ---- [debuginfo-both] debuginfo/associated-types.rs stdout ----
[01:29:08] NOTE: compiletest thinks it is using LLDB version 902
[01:29:08] NOTE: compiletest thinks it is using LLDB without native rust support
[01:29:08] 
[01:29:08] error: line not found in debugger output: [...]$0 = Struct<i32> { b: -1, b1: 0 }
[01:29:08] status: exit code: 0
[01:29:08] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types/a" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types/associated-types.debugger.script"
[01:29:08] ------------------------------------------
[01:29:08] ------------------------------------------
[01:29:08] LLDB batch-mode script
[01:29:08] ----------------------
[01:29:08] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types/associated-types.debugger.script'.
[01:29:08] Target executable is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types/a'.
[01:29:08] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:29:08] Creating a target for '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types/a'
[01:29:08] settings set auto-confirm true
[01:29:08] version
[01:29:08] version
[01:29:08] lldb-902.0.73.1 Swift-4.1 
[01:29:08] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:29:08] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:29:08] type category enable Rust
[01:29:08] 
[01:29:08] breakpoint set --file 'associated-types.rs' --line 119
[01:29:08] Breakpoint 1: where = a`associated_types::assoc_struct::h44227cf59dd6d64b + 15 at associated-types.rs:119, address = 0x000000010000154f 
[01:29:08] breakpoint set --file 'associated-types.rs' --line 126
[01:29:08] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:29:08] make: *** [check] Error 1
[01:29:08] make: INTERNAL: Exiting with 3 jobserver tokens available; should be 4!
[01:29:08] Breakpoint 2: where = a`associated_types::assoc_local::h54501cffe235f1b9 + 70 at associated-types.rs:126, address = 0x00000001000015d6 
[01:29:08] breakpoint set --file 'associated-types.rs' --line 130
[01:29:08] Breakpoint 3: where = a`associated_types::assoc_arg::h4bbbcaac5017ebf2 + 12 at associated-types.rs:130, address = 0x000000010000161c 
[01:29:08] breakpoint set --file 'associated-types.rs' --line 138
[01:29:08] Breakpoint 4: where = a`associated_types::assoc_tuple::h0c8ad203784a97cc + 15 at associated-types.rs:138, address = 0x000000010000169f 
[01:29:08] breakpoint set --file 'associated-types.rs' --line 145
[01:29:08] Breakpoint 5: where = a`associated_types::assoc_enum::h96bb104a18eccaf2 + 138 at associated-types.rs:145, address = 0x000000010000175a 
[01:29:08] breakpoint set --file 'associated-types.rs' --line 148
[01:29:08] Breakpoint 6: where = a`associated_types::assoc_enum::h96bb104a18eccaf2 + 182 at associated-types.rs:148, address = 0x0000000100001786 
[01:29:08] run
[01:29:08] Hit breakpoint 1.1: where = a`associated_types::assoc_struct::h44227cf59dd6d64b + 15 at associated-types.rs:119, address = 0x000000010000154f, resolved, hit count = 1 
[01:29:08] Process 69403 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x000000010000154f a`associated_types::assoc_struct::h44227cf59dd6d64b(arg=Struct<int> { b: -1, b1: 0 }) at associated-types.rs:119 116 } 117 118 fn assoc_struct<T: TraitWithAssocType>(arg: Struct<T>) { -> 119  zzz(); // #break 120 } 121 122 fn assoc_local<T: TraitWithAssocType>(x: T) { Target 0: (a) stopped. Process 69403 launched: '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/associated-types/a' (x86_64) 
[01:29:08] print arg
[01:29:08] (associated_types::Struct<int>) $0 = Struct<int> { b: -1, b1: 0 } 
[01:29:08] continue
[01:29:08] Hit breakpoint 2.1: where = a`associated_types::assoc_local::h54501cffe235f1b9 + 70 at associated-types.rs:126, address = 0x00000001000015d6, resolved, hit count = 1 
[01:29:08] print inferred
[01:29:08] (long) $1 = 1 
[01:29:08] print explicitly
[01:29:08] (long) $2 = 1 
[01:29:08] continue
[01:29:08] Hit breakpoint 3.1: where = a`associated_types::assoc_arg::h4bbbcaac5017ebf2 + 12 at associated-types.rs:130, address = 0x000000010000161c, resolved, hit count = 1 
[01:29:08] print arg
[01:29:08] (long) $3 = 2 
[01:29:08] continue
[01:29:08] Hit breakpoint 4.1: where = a`associated_types::assoc_tuple::h0c8ad203784a97cc + 15 at associated-types.rs:138, address = 0x000000010000169f, resolved, hit count = 1 
[01:29:08] print arg
[01:29:08] ((i32, i64)) $4 = (4, 5) 
[01:29:08] continue
[01:29:08] Hit breakpoint 5.1: where = a`associated_types::assoc_enum::h96bb104a18eccaf2 + 138 at associated-types.rs:145, address = 0x000000010000175a, resolved, hit count = 1 
[01:29:08] print a
[01:29:08] (int) $5 = 6 
[01:29:08] print b
[01:29:08] (long) $6 = 7 
[01:29:08] continue
[01:29:08] Hit breakpoint 6.1: where = a`associated_types::assoc_enum::h96bb104a18eccaf2 + 182 at associated-types.rs:148, address = 0x0000000100001786, resolved, hit count = 1 
[01:29:08] print a
[01:29:08] (long) $7 = 8 
[01:29:08] print b
[01:29:08] (int) $8 = 9 
[01:29:08] quit
[01:29:08] None
[01:29:08] 
[01:29:08] ------------------------------------------
---
[01:29:08] 
[01:29:08] thread '[debuginfo-both] debuginfo/associated-types.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:29:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:29:08] 
[01:29:08] ---- [debuginfo-both] debuginfo/generic-method-on-generic-struct.rs stdout ----
[01:29:08] NOTE: compiletest thinks it is using LLDB version 902
[01:29:08] NOTE: compiletest thinks it is using LLDB without native rust support
[01:29:08] 
[01:29:08] error: line not found in debugger output: [...]$6 = Struct<f64> { x: 1234.5 }
[01:29:08] status: exit code: 0
[01:29:08] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct/a" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct/generic-method-on-generic-struct.debugger.script"
[01:29:08] ------------------------------------------
[01:29:08] ------------------------------------------
[01:29:08] LLDB batch-mode script
[01:29:08] ----------------------
[01:29:08] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct/generic-method-on-generic-struct.debugger.script'.
[01:29:08] Target executable is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct/a'.
[01:29:08] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:29:08] Creating a target for '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct/a'
[01:29:08] settings set auto-confirm true
[01:29:08] version
[01:29:08] version
[01:29:08] lldb-902.0.73.1 Swift-4.1 
[01:29:08] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:29:08] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:29:08] type category enable Rust
[01:29:08] 
[01:29:08] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 147
[01:29:08] Breakpoint 1: 2 locations. 
[01:29:08] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 152
[01:29:08] Breakpoint 2: 2 locations. 
[01:29:08] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 157
[01:29:08] Breakpoint 3: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_owned::h9df7546eb8986fae + 21 at generic-method-on-generic-struct.rs:157, address = 0x0000000100001705 
[01:29:08] run
[01:29:08] Hit breakpoint 1.1: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_ref::h5a0c5e7d2b0ebec6 + 23 at generic-method-on-generic-struct.rs:147, address = 0x00000001000015c7, resolved, hit count = 1 
[01:29:08] Process 69647 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000015c7 a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_ref::h5a0c5e7d2b0ebec6(self=&0x7ffeefbfdfb0, arg1=-1, arg2=2) at generic-method-on-generic-struct.rs:147 144 impl<T1> Struct<T1> { 145 146 fn self_by_ref<T2>(&self, arg1: isize, arg2: T2) -> isize { -> 147  zzz(); // #break 148 arg1 149 } 150 Target 0: (a) stopped. Process 69647 launched: '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-method-on-generic-struct/a' (x86_64) 
[01:29:08] print *self
[01:29:08] (generic_method_on_generic_struct::Struct<(u32, i32)>) $0 = Struct<(u32, i32)> { x: (8888, -8888) } 
[01:29:08] print arg1
[01:29:08] (long) $1 = -1 
[01:29:08] print arg2
[01:29:08] (unsigned short) $2 = 2 
[01:29:08] continue
[01:29:08] Hit breakpoint 2.1: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_val::h4da777e071282d9f + 25 at generic-method-on-generic-struct.rs:152, address = 0x0000000100001669, resolved, hit count = 1 
[01:29:08] print self
[01:29:08] (generic_method_on_generic_struct::Struct<(u32, i32)>) $3 = Struct<(u32, i32)> { x: (8888, -8888) } 
[01:29:08] print arg1
[01:29:08] (long) $4 = -3 
[01:29:08] print arg2
[01:29:08] (short) $5 = -4 
[01:29:08] continue
[01:29:08] Hit breakpoint 1.2: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_ref::hebf4764983adb481 + 19 at generic-method-on-generic-struct.rs:147, address = 0x0000000100001613, resolved, hit count = 1 
[01:29:08] print *self
[01:29:08] (generic_method_on_generic_struct::Struct<double>) $6 = Struct<double> { x: 1234.5 } 
[01:29:08] print arg1
[01:29:08] (long) $7 = -5 
[01:29:08] print arg2
[01:29:08] (int) $8 = -6 
[01:29:08] continue
[01:29:08] Hit breakpoint 2.2: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_val::hbe1131295656e121 + 21 at generic-method-on-generic-struct.rs:152, address = 0x00000001000016b5, resolved, hit count = 1 
[01:29:08] print self
[01:29:08] (generic_method_on_generic_struct::Struct<double>) $9 = Struct<double> { x: 1234.5 } 
[01:29:08] print arg1
[01:29:08] (long) $10 = -7 
[01:29:08] print arg2
[01:29:08] (long) $11 = -8 
[01:29:08] continue
[01:29:08] Hit breakpoint 3.1: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_owned::h9df7546eb8986fae + 21 at generic-method-on-generic-struct.rs:157, address = 0x0000000100001705, resolved, hit count = 1 
[01:29:08] print *self
[01:29:08] (generic_method_on_generic_struct::Struct<double>) $12 = Struct<double> { x: 1234.5 } 
[01:29:08] print arg1
[01:29:08] (long) $13 = -9 
[01:29:08] print arg2
[01:29:08] (float) $14 = -10.5 
[01:29:08] quit
[01:29:08] None
[01:29:08] 
[01:29:08] ------------------------------------------
[01:29:08] ------------------------------------------
[01:29:08] stderr:
[01:29:08] ------------------------------------------
[01:29:08] 
[01:29:08] ------------------------------------------
[01:29:08] 
[01:29:08] thread '[debuginfo-both] debuginfo/generic-method-on-generic-struct.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:29:08] 
[01:29:08] ---- [debuginfo-both] debuginfo/generic-struct.rs stdout ----
[01:29:08] NOTE: compiletest thinks it is using LLDB version 902
[01:29:08] NOTE: compiletest thinks it is using LLDB without native rust support
[01:29:08] 
[01:29:08] error: line not found in debugger output: [...]$0 = AGenericStruct<i32, i32> { key: 0, value: 1 }
[01:29:08] status: exit code: 0
[01:29:08] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-struct/a" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-struct/generic-struct.debugger.script"
[01:29:08] ------------------------------------------
[01:29:08] ------------------------------------------
[01:29:08] LLDB batch-mode script
[01:29:08] ----------------------
[01:29:08] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-struct/generic-struct.debugger.script'.
[01:29:08] Target executable is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-struct/a'.
[01:29:08] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:29:08] Creating a target for '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-struct/a'
[01:29:08] settings set auto-confirm true
[01:29:08] version
[01:29:08] version
[01:29:08] lldb-902.0.73.1 Swift-4.1 
[01:29:08] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:29:08] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:29:08] type category enable Rust
[01:29:08] 
[01:29:08] breakpoint set --file 'generic-struct.rs' --line 70
[01:29:08] Breakpoint 1: where = a`generic_struct::main::h8bb33f6c1dcf1f13 + 111 at generic-struct.rs:70, address = 0x0000000100000c9f 
[01:29:08] run
[01:29:08] Hit breakpoint 1.1: where = a`generic_struct::main::h8bb33f6c1dcf1f13 + 111 at generic-struct.rs:70, address = 0x0000000100000c9f, resolved, hit count = 1 
[01:29:08] Process 69666 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000c9f a`generic_struct::main::h8bb33f6c1dcf1f13 at generic-struct.rs:70 67 value: AGenericStruct { key: 7, value: 8.5f64 }, 68 }; 69 -> 70  zzz(); // #break 71 } 72 73 fn zzz() { () } Target 0: (a) stopped. Process 69666 launched: '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/generic-struct/a' (x86_64) 
[01:29:08] print int_int
[01:29:08] (generic_struct::AGenericStruct<int, int>) $0 = AGenericStruct<int, int> { key: 0, value: 1 } 
[01:29:08] print int_float
[01:29:08] (generic_struct::AGenericStruct<int, double>) $1 = AGenericStruct<int, double> { key: 2, value: 3.5 } 
[01:29:08] print float_int
[01:29:08] (generic_struct::AGenericStruct<double, int>) $2 = AGenericStruct<double, int> { key: 4.5, value: 5 } 
[01:29:08] print float_int_float
[01:29:08] (generic_struct::AGenericStruct<double, generic_struct::AGenericStruct<int, double> >) $3 = AGenericStruct<double, generic_struct::AGenericStruct<int, double> > { key: 6.5, value: AGenericStruct<int, double> { key: 7, value: 8.5 } } 
[01:29:08] quit
[01:29:08] 
[01:29:08] ------------------------------------------
[01:29:08] stderr:
[01:29:08] ------------------------------------------
[01:29:08] ------------------------------------------
[01:29:08] 
[01:29:08] ------------------------------------------
[01:29:08] 
[01:29:08] thread '[debuginfo-both] debuginfo/generic-struct.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:29:08] 
[01:29:08] ---- [debuginfo-both] debuginfo/method-on-generic-struct.rs stdout ----
[01:29:08] NOTE: compiletest thinks it is using LLDB version 902
[01:29:08] NOTE: compiletest thinks it is using LLDB without native rust support
[01:29:08] 
[01:29:08] error: line not found in debugger output: [...]$6 = Struct<f64> { x: 1234.5 }
[01:29:08] status: exit code: 0
[01:29:08] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/method-on-generic-struct/a" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/method-on-generic-struct/method-on-generic-struct.debugger.script"
[01:29:08] ------------------------------------------
[01:29:08] ------------------------------------------
[01:29:08] LLDB batch-mode script
[01:29:08] ----------------------
[01:29:08] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/method-on-generic-struct/method-on-generic-struct.debugger.script'.
[01:29:08] Target executable is '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/method-on-generic-struct/a'.
[01:29:08] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:29:08] Creating a target for '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/method-on-generic-struct/a'
[01:29:08] settings set auto-confirm true
[01:29:08] version
[01:29:08] version
[01:29:08] lldb-902.0.73.1 Swift-4.1 
[01:29:08] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:29:08] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:29:08] type category enable Rust
[01:29:08] 
[01:29:08] breakpoint set --file 'method-on-generic-struct.rs' --line 147
[01:29:08] Breakpoint 1: 2 locations. 
[01:29:08] breakpoint set --file 'method-on-generic-struct.rs' --line 152
[01:29:08] Breakpoint 2: 2 locations. 
[01:29:08] breakpoint set --file 'method-on-generic-struct.rs' --line 157
[01:29:08] Breakpoint 3: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_owned::h2e182fd3f18bb4de + 20 at method-on-generic-struct.rs:157, address = 0x00000001000015a4 
[01:29:08] run
[01:29:08] Hit breakpoint 1.2: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_ref::haa416f63fe4ae79d + 20 at method-on-generic-struct.rs:147, address = 0x0000000100001474, resolved, hit count = 1 
[01:29:08] Process 69813 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2 frame #0: 0x0000000100001474 a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_ref::haa416f63fe4ae79d(self=&0x7ffeefbfdfc0, arg1=-1, arg2=-2) at method-on-generic-struct.rs:147 144 impl<T> Struct<T> { 145 146 fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize { -> 147  zzz(); // #break 148 arg1 + arg2 149 } 150 Target 0: (a) stopped. Process 69813 launched: '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo/method-on-generic-struct/a' (x86_64) 
[01:29:08] print *self
[01:29:08] (method_on_generic_struct::Struct<(u32, i32)>) $0 = Struct<(u32, i32)> { x: (8888, -8888) } 
[01:29:08] print arg1
[01:29:08] (long) $1 = -1 
[01:29:08] print arg2
[01:29:08] (long) $2 = -2 
[01:29:08] continue
[01:29:08] Hit breakpoint 2.1: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_val::h50e4107eed3eaa69 + 22 at method-on-generic-struct.rs:152, address = 0x00000001000014c6, resolved, hit count = 1 
[01:29:08] print self
[01:29:08] (method_on_generic_struct::Struct<(u32, i32)>) $3 = Struct<(u32, i32)> { x: (8888, -8888) } 
[01:29:08] print arg1
[01:29:08] (long) $4 = -3 
[01:29:08] print arg2
[01:29:08] (long) $5 = -4 
[01:29:08] continue
[01:29:08] Hit breakpoint 1.1: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_ref::h2f351a348113829a + 20 at method-on-generic-struct.rs:147, address = 0x0000000100001424, resolved, hit count = 1 
[01:29:08] print *self
[01:29:08] (method_on_generic_struct::Struct<double>) $6 = Struct<double> { x: 1234.5 } 
[01:29:08] print arg1
[01:29:08] (long) $7 = -5 
[01:29:08] print arg2
[01:29:08] (long) $8 = -6 
[01:29:08] continue
[01:29:08] Hit breakpoint 2.2: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_val::he801552d33f51407 + 21 at method-on-generic-struct.rs:152, address = 0x0000000100001535, resolved, hit count = 1 
[01:29:08] print self
[01:29:08] (method_on_generic_struct::Struct<double>) $9 = Struct<double> { x: 1234.5 } 
[01:29:08] print arg1
[01:29:08] (long) $10 = -7 
[01:29:08] print arg2
[01:29:08] (long) $11 = -8 
[01:29:08] continue
[01:29:08] Hit breakpoint 3.1: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_owned::h2e182fd3f18bb4de + 20 at method-on-generic-struct.rs:157, address = 0x00000001000015a4, resolved, hit count = 1 
[01:29:08] print *self
[01:29:08] (method_on_generic_struct::Struct<double>) $12 = Struct<double> { x: 1234.5 } 
[01:29:08] print arg1
[01:29:08] (long) $13 = -9 
[01:29:08] print arg2
[01:29:08] (long) $14 = -10 
[01:29:08] quit
[01:29:08] None
[01:29:08] 
[01:29:08] ------------------------------------------
---
[01:29:08] test result: FAILED. 87 passed; 4 failed; 19 ignored; 0 measured; 0 filtered out
[01:29:08] 
[01:29:08] 
[01:29:08] 
[01:29:08] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/debuginfo" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/debuginfo" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "debuginfo-both" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:29:08] 
[01:29:08] 
[01:29:08] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:29:08] Build completed unsuccessfully in 0:33:06
---
travis_fold:start:after_failure.2
travis_time:start:196969d2
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1264
drwx------  22 travis  staff    748 Oct 30 17:33 .
-rw-------@  1 travis  staff  62246 Oct 30 17:33 a_2018-10-30-173302-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Oct 30 17:33 a_2018-10-30-173302_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60386 Oct 30 17:32 a_2018-10-30-173255-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37237 Oct 30 17:32 a_2018-10-30-173255_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Oct 30 17:32 a_2018-10-30-173250_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Oct 30 17:32 a_2018-10-30-173245_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Oct 30 17:32 a_2018-10-30-173236-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9845 Oct 30 17:32 a_2018-10-30-173236_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Oct 30 17:32 a_2018-10-30-173207_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63072 Oct 30 17:31 a_2018-10-30-173157_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65066 Oct 30 17:31 a_2018-10-30-173154-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64234 Oct 30 17:31 a_2018-10-30-173154-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63914 Oct 30 17:31 a_2018-10-30-173154_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11731 Oct 30 17:29 a_2018-10-30-172951_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Oct 30 17:29 a_2018-10-30-172905_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Oct 30 17:27 a_2018-10-30-172752_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10484 Oct 30 17:26 a_2018-10-30-172657-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10486 Oct 30 17:26 a_2018-10-30-172657_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 Oct 30 17:26 a_2018-10-30-172627_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Oct 30 17:24 a_2018-10-30-172453_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:236c4066
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-10-30-172453_Traviss-Mac-1044.crash
Process:               a [52499]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [52492]
Responsible:           a [52499]
User ID:               501
Date/Time:             2018-10-30 17:24:46.983 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00007000039a6e50
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0x7000039a6e50:
    __LINKEDIT             000000010f82f000-000000010f84a000 [  108K] r--/rwx SM=COW  /usr/lib/dyld
--> Stack Guard            00007000039a6000-00007000039a7000 [    4K] ---/rwx SM=NUL  
    Stack                  00007000039a7000-0000700003ba9000 [ 2056K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff68de005a __semwait_signal + 10
1   libsystem_pthread.dylib        0x00007fff68f1f8ec _pthread_join + 626
2   a                              0x000000010d4f60d4 stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2772
3   a                              0x000000010d511576 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4d90ea0a71c76709 + 6
4   a                              0x000000010d4f79ba main + 522
5   libdyld.dylib                  0x00007fff68c90115 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0x00007fff68ddfe3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff68f1e150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff68d3c312 abort + 127
3   a                              0x000000010d4f9209 std::sys::unix::abort_internal::h127beb97219cb4c9 + 9
4   a                              0x000000010d4f91fb std::sys_common::util::abort::h4f1c42bb5c3676b2 + 91
5   a                              0x000000010d50af59 std::sys::unix::stack_overflow::imp::signal_handler::hef36ab96c29dc5a2 + 665
6   libsystem_platform.dylib       0x00007fff68f11f5a _sigtramp + 26
7   ???                            000000000000000000 0 + 0
8   a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
9   a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
10  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
11  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
12  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
13  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
14  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
15  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
16  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
17  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
18  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
19  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
20  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
21  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
22  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
23  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
24  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
25  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
26  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
27  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
28  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
29  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
30  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
31  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
32  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
33  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
34  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
35  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
36  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
37  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
38  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
39  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
40  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
41  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
42  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
43  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
44  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
45  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
46  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
47  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
48  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
49  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
50  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
51  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
52  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
53  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
54  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
55  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
56  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
57  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
58  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
59  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
60  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
61  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
62  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
63  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
64  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
65  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
66  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
67  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
68  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
69  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
70  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
71  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
72  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
73  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
74  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
75  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
76  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
77  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
78  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
79  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
80  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
81  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
82  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
83  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
84  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
85  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
86  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
87  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
88  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
89  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
90  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
91  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
92  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
93  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
94  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
95  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
96  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
97  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
98  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
99  a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
100 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
101 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
102 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
103 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
104 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
105 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
106 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
107 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
108 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
109 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
110 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
111 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
112 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
113 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
114 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
115 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
116 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
117 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
118 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
119 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
120 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
121 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
122 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
123 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
124 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
125 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
126 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
127 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
128 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
129 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
130 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
131 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
132 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
133 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
134 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
135 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
136 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
137 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
138 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
139 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
140 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
141 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
142 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
143 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
144 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
145 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
146 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
147 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
148 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
149 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
150 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
151 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
152 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
153 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
154 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
155 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
156 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
157 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
158 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
159 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
160 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
161 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
162 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
163 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
164 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
165 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
166 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
167 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
168 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
169 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
170 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
171 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
172 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
173 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
174 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
175 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
176 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
177 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
178 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
179 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
180 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
181 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
182 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
183 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
184 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
185 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
186 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
187 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
188 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
189 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
190 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
191 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
192 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
193 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
194 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
195 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
196 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
197 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
198 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
199 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
200 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
201 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
202 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
203 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
204 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
205 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
206 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
207 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
208 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
209 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
210 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
211 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
212 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
213 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
214 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
215 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
216 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
217 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
218 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
219 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
220 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
221 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
222 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
223 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
224 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
225 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
226 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
227 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
228 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
229 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
230 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
231 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
232 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
233 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
234 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
235 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
236 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
237 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
238 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
239 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
240 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
241 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
242 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
243 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
244 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
245 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
246 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
247 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
248 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
249 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
250 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
251 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
252 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
253 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
254 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
255 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
256 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
257 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
258 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
259 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
260 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
261 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
262 a                              0x000000010d4f66a2 stack_probes_lto::recurse::h907252696a8f0ddd + 34
263 a                              0x000000010d510c61 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h5ff40af796075746 + 129
264 a                              0x000000010d50b388 std::sys::unix::thread::Thread::new::thread_start::hc7d248493042ec4c + 136
265 libsystem_pthread.dylib        0x00007fff68f1b6c1 _pthread_body + 340
266 libsystem_pthread.dylib        0x00007fff68f1b56d _pthread_start + 377
267 libsystem_pthread.dylib        0x00007fff68f1ac5d thread_start + 13
Thread 1 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x0000700003ba7000  rcx: 0x000000010d5c4968  rdx: 0x0000000000000000
  rdi: 0x0000000000001303  rsi: 0x0000000000000006  rbp: 0x000000010d5c49a0  rsp: 0x000000010d5c4968
   r8: 0x000000010d546ff8   r9: 0x0000000000000001  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000001303  r13: 0x000000010d818060  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff68ddfe3e  rfl: 0x0000000000000206  cr2: 0x000000010d4f6052
Logical CPU:     0
Error Code:      0x0200014e
Trap Number:     133
Binary Images:
       0x10d4f4000 -        0x10d553fef +a (0) <F1F2D6F1-65FB-3DE6-9042-FF8546831F13> /Users/USER/*/a
       0x10f7ac000 -        0x10f7f698f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff664fa000 -     0x7fff6652dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff66a0c000 -     0x7fff66a0dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff66cc2000 -     0x7fff66d18fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff66d19000 -     0x7fff66d3dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6808f000 -     0x7fff684803b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6854d000 -     0x7fff68569ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff68b27000 -     0x7fff68b2bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff68b2c000 -     0x7fff68b36ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff68b37000 -     0x7fff68b3efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff68b3f000 -     0x7fff68b47ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff68b48000 -     0x7fff68bcdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff68c55000 -     0x7fff68c8eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff68c8f000 -     0x7fff68cacff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff68cad000 -     0x7fff68cadffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff68cbb000 -     0x7fff68cbbff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff68cbc000 -     0x7fff68cc0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff68cc1000 -     0x7fff68cc3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff68cc4000 -     0x7fff68cc5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff68cc6000 -     0x7fff68cddfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff68cde000 -     0x7fff68cdefff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff68cdf000 -     0x7fff68d68ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff68d69000 -     0x7fff68d6cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff68d6d000 -     0x7fff68d70ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff68d71000 -     0x7fff68d72fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff68d73000 -     0x7fff68d79ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff68d7a000 -     0x7fff68dc3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff68dc4000 -     0x7fff68de9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff68dea000 -     0x7fff68e35fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff68e36000 -     0x7fff68e55fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff68e56000 -     0x7fff68efaff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff68efb000 -     0x7fff68f05ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff68f06000 -     0x7fff68f0fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff68f10000 -     0x7fff68f17ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff68f18000 -     0x7fff68f23fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff68f24000 -     0x7fff68f27ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff68f28000 -     0x7fff68f29ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff68f2a000 -     0x7fff68f31ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff68f32000 -     0x7fff68f45ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff68f47000 -     0x7fff68f4cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff68f4d000 -     0x7fff68f79ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by all processes on this machine:
  Calls made by all processes on this machine:
    task_for_pid: 2044
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=197.8M resident=0K(0%) swapped_out_or_unallocated=197.8M(100%)
Writable regions: Total=78.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=78.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                       4228K        5 
VM_ALLOCATE                       4228K        5 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            2344K       43 
__LINKEDIT                       188.6M        4 
__TEXT                            9384K       43 
===========                     =======  ======= 
TOTAL                            280.4M      110 
TOTAL                            280.4M      110 
TOTAL, minus reserved VM space   280.3M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-10-30-172627_Traviss-Mac-1044.crash
Process:               a [54879]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [54878]
Responsible:           a [54879]
User ID:               501
Date/Time:             2018-10-30 17:26:27.374 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010825c78e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x000000010825bb89 std::panicking::try::do_call::h4e230f7aae0aa6f8 (.llvm.1795092263000012212) + 9
2   libstd-f7dd09cf573f28a7.dylib  0x00000001082a84ff __rust_maybe_catch_panic + 31
3   a                              0x000000010825c9e1 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x000000010825af56 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9cbf2464952f9640 + 6
5   libstd-f7dd09cf573f28a7.dylib  0x000000010828c4c8 std::panicking::try::do_call::h8122f79c8f5c9b16 + 24
6   libstd-f7dd09cf573f28a7.dylib  0x00000001082a84ff __rust_maybe_catch_panic + 31
7   libstd-f7dd09cf573f28a7.dylib  0x000000010828ce5b std::rt::lang_start_internal::h615662c89d296452 + 379
8   a                              0x000000010825ccec main + 44
9   libdyld.dylib                  0x00007fff68c90115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x000000010881c050  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee79a3338  rsi: 0xfffffffffffffce8  rbp: 0x00007ffee79a3d40  rsp: 0x00007ffee79a3d40
   r8: 0xffffffff00000000   r9: 0x0000000108344110  r10: 0x000000010e8d78d8  r11: 0x00007fff68f4796c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee79a3e60  r15: 0x00007ffee79a3da8
  rip: 0x000000010825c78e  rfl: 0x0000000000010206  cr2: 0x000000010831d488
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10825a000 -        0x10825dfff +a (0) <4BE7264E-AE34-38A3-B09E-4D0CD8CC34ED> /Users/USER/*/a
       0x108267000 -        0x10833afd7 +libstd-f7dd09cf573f28a7.dylib (0) <4B0714A6-54A5-3BFA-BE9B-F9AEF0F181D6> /Users/USER/*/libstd-f7dd09cf573f28a7.dylib
       0x10e885000 -        0x10e8cf98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff664fa000 -     0x7fff6652dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff66a0c000 -     0x7fff66a0dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff66cc2000 -     0x7fff66d18fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff66d19000 -     0x7fff66d3dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6808f000 -     0x7fff684803b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6854d000 -     0x7fff68569ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff68b27000 -     0x7fff68b2bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff68b2c000 -     0x7fff68b36ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff68b37000 -     0x7fff68b3efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff68b3f000 -     0x7fff68b47ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff68b48000 -     0x7fff68bcdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff68c55000 -     0x7fff68c8eff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff68c8f000 -     0x7fff68cacff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff68cad000 -     0x7fff68cadffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff68cbb000 -     0x7fff68cbbff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff68cbc000 -     0x7fff68cc0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff68cc1000 -     0x7fff68cc3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff68cc4000 -     0x7fff68cc5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff68cc6000 -     0x7fff68cddfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff68cde000 -     0x7fff68cdefff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff68cdf000 -     0x7fff68d68ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff68d69000 -     0x7fff68d6cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff68d6d000 -     0x7fff68d70ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff68d71000 -     0x7fff68d72fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff68d73000 -     0x7fff68d79ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff68d7a000 -     0x7fff68dc3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff68dc4000 -     0x7fff68de9ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff68dea000 -     0x7fff68e35fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff68e36000 -     0x7fff68e55fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff68e56000 -     0x7fff68efaff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff68efb000 -     0x7fff68f05ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff68f06000 -     0x7fff68f0fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff68f10000 -     0x7fff68f17ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff68f18000 -     0x7fff68f23fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
