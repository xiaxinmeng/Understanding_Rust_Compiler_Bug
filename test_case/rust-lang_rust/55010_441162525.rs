
[01:29:06] failures:
[01:29:06] 
[01:29:06] ---- [debuginfo-both] debuginfo/associated-types.rs stdout ----
[01:29:06] NOTE: compiletest thinks it is using LLDB version 902
[01:29:06] NOTE: compiletest thinks it is using LLDB without native rust support
[01:29:06] 
[01:29:06] error: line not found in debugger output: [...]$0 = Struct<i32> { b: -1, b1: 0 }
[01:29:06] status: exit code: 0
[01:29:06] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/associated-types/a" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/associated-types/associated-types.debugger.script"
[01:29:06] stdout:
[01:29:06] ------------------------------------------
[01:29:06] LLDB batch-mode script
[01:29:06] ----------------------
[01:29:06] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/associated-types/associated-types.debugger.script'.
[01:29:06] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/associated-types/a'.
[01:29:06] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:29:06] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/associated-types/a'
[01:29:06] settings set auto-confirm true
[01:29:06] 
[01:29:06] version
[01:29:06] lldb-902.0.73.1 Swift-4.1 
[01:29:06] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:29:06] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:29:06] type category enable Rust
[01:29:06] 
[01:29:06] breakpoint set --file 'associated-types.rs' --line 123
[01:29:06] Breakpoint 1: where = a`associated_types::assoc_struct::ha2dcd7b6de6979f1 + 24 at associated-types.rs:123, address = 0x00002648 
[01:29:06] breakpoint set --file 'associated-types.rs' --line 130
[01:29:06] Breakpoint 2: where = a`associated_types::assoc_local::h54501cffe235f1b9 + 93 at associated-types.rs:130, address = 0x000026ed 
[01:29:06] breakpoint set --file 'associated-types.rs' --line 134
[01:29:06] Breakpoint 3: where = a`associated_types::assoc_arg::h4bbbcaac5017ebf2 + 12 at associated-types.rs:134, address = 0x0000272c 
[01:29:06] breakpoint set --file 'associated-types.rs' --line 142
[01:29:06] Breakpoint 4: where = a`associated_types::assoc_tuple::h0c8ad203784a97cc + 24 at associated-types.rs:142, address = 0x000027c8 
[01:29:06] breakpoint set --file 'associated-types.rs' --line 149
[01:29:06] Breakpoint 5: where = a`associated_types::assoc_enum::h6d1737c7f978d85f + 135 at associated-types.rs:149, address = 0x00002877 
[01:29:06] breakpoint set --file 'associated-types.rs' --line 152
[01:29:06] Breakpoint 6: where = a`associated_types::assoc_enum::h6d1737c7f978d85f + 180 at associated-types.rs:152, address = 0x000028a4 
[01:29:06] run
[01:29:06] Hit breakpoint 1.1: where = a`associated_types::assoc_struct::ha2dcd7b6de6979f1 + 24 at associated-types.rs:123, address = 0x00002648, resolved, hit count = 1 
[01:29:06] Process 70577 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00002648 a`associated_types::assoc_struct::ha2dcd7b6de6979f1(arg=Struct<int> { b: -1, b1: 0 }) at associated-types.rs:123 120 } 121 122 fn assoc_struct<T: TraitWithAssocType>(arg: Struct<T>) { -> 123 [4m [0mzzz(); // #break 124 } 125 126 fn assoc_local<T: TraitWithAssocType>(x: T) { Target 0: (a) stopped. Process 70577 launched: '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/associated-types/a' (i386) 
[01:29:06] print arg
[01:29:06] (associated_types::Struct<int>) $0 = Struct<int> { b: -1, b1: 0 } 
[01:29:06] continue
[01:29:06] Hit breakpoint 2.1: where = a`associated_types::assoc_local::h54501cffe235f1b9 + 93 at associated-types.rs:130, address = 0x000026ed, resolved, hit count = 1 
[01:29:06] print inferred
[01:29:06] (long long) $1 = 1 
[01:29:06] print explicitly
[01:29:06] (long long) $2 = 1 
[01:29:06] continue
[01:29:06] Hit breakpoint 3.1: where = a`associated_types::assoc_arg::h4bbbcaac5017ebf2 + 12 at associated-types.rs:134, address = 0x0000272c, resolved, hit count = 1 
[01:29:06] print arg
[01:29:06] (long long) $3 = 2 
[01:29:06] continue
[01:29:06] Hit breakpoint 4.1: where = a`associated_types::assoc_tuple::h0c8ad203784a97cc + 24 at associated-types.rs:142, address = 0x000027c8, resolved, hit count = 1 
[01:29:06] print arg
[01:29:06] ((i32, i64)) $4 = (4, 5) 
[01:29:06] continue
[01:29:06] Hit breakpoint 5.1: where = a`associated_types::assoc_enum::h6d1737c7f978d85f + 135 at associated-types.rs:149, address = 0x00002877, resolved, hit count = 1 
[01:29:06] print a
[01:29:06] (int) $5 = 6 
[01:29:06] print b
[01:29:06] (long long) $6 = 7 
[01:29:06] continue
[01:29:06] Hit breakpoint 6.1: where = a`associated_types::assoc_enum::h6d1737c7f978d85f + 180 at associated-types.rs:152, address = 0x000028a4, resolved, hit count = 1 
[01:29:06] print a
[01:29:06] (long long) $7 = 8 
[01:29:06] print b
[01:29:06] (int) $8 = 9 
[01:29:06] continue
[01:29:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:29:06] quit
[01:29:06] None
[01:29:06] 
[01:29:06] ------------------------------------------
[01:29:06] stderr:
[01:29:06] ------------------------------------------
[01:29:06] 
[01:29:06] ------------------------------------------
[01:29:06] 
[01:29:06] thread '[debuginfo-both] debuginfo/associated-types.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:29:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:29:06] make: *** [check] Error 1
[01:29:06] 
[01:29:06] ---- [debuginfo-both] debuginfo/generic-method-on-generic-struct.rs stdout ----
[01:29:06] NOTE: compiletest thinks it is using LLDB version 902
[01:29:06] NOTE: compiletest thinks it is using LLDB without native rust support
[01:29:06] 
[01:29:06] error: line not found in debugger output: [...]$6 = Struct<f64> { x: 1234.5 }
[01:29:06] status: exit code: 0
[01:29:06] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-method-on-generic-struct/a" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-method-on-generic-struct/generic-method-on-generic-struct.debugger.script"
[01:29:06] stdout:
[01:29:06] ------------------------------------------
[01:29:06] LLDB batch-mode script
[01:29:06] ----------------------
[01:29:06] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-method-on-generic-struct/generic-method-on-generic-struct.debugger.script'.
[01:29:06] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-method-on-generic-struct/a'.
[01:29:06] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:29:06] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-method-on-generic-struct/a'
[01:29:06] settings set auto-confirm true
[01:29:06] 
[01:29:06] version
[01:29:06] lldb-902.0.73.1 Swift-4.1 
[01:29:06] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:29:06] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:29:06] type category enable Rust
[01:29:06] 
[01:29:06] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 151
[01:29:06] Breakpoint 1: 2 locations. 
[01:29:06] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 156
[01:29:06] Breakpoint 2: 2 locations. 
[01:29:06] breakpoint set --file 'generic-method-on-generic-struct.rs' --line 161
[01:29:06] Breakpoint 3: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_owned::hd277517aab233e26 + 17 at generic-method-on-generic-struct.rs:161, address = 0x00002831 
[01:29:06] run
[01:29:06] Hit breakpoint 1.2: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_ref::hfb71d7b952992a85 + 16 at generic-method-on-generic-struct.rs:151, address = 0x00002720, resolved, hit count = 1 
[01:29:06] Process 70813 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2 frame #0: 0x00002720 a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_ref::hfb71d7b952992a85(self=&0xbfffe248, arg1=-1, arg2=2) at generic-method-on-generic-struct.rs:151 148 impl<T1> Struct<T1> { 149 150 fn self_by_ref<T2>(&self, arg1: isize, arg2: T2) -> isize { -> 151 [4m [0mzzz(); // #break 152 arg1 153 } 154 Target 0: (a) stopped. Process 70813 launched: '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-method-on-generic-struct/a' (i386) 
[01:29:06] print *self
[01:29:06] (generic_method_on_generic_struct::Struct<(u32, i32)>) $0 = Struct<(u32, i32)> { x: (8888, -8888) } 
[01:29:06] print arg1
[01:29:06] (int) $1 = -1 
[01:29:06] print arg2
[01:29:06] (unsigned short) $2 = 2 
[01:29:06] continue
[01:29:06] Hit breakpoint 2.1: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_val::h67de54a2509a1cb5 + 26 at generic-method-on-generic-struct.rs:156, address = 0x0000277a, resolved, hit count = 1 
[01:29:06] print self
[01:29:06] (generic_method_on_generic_struct::Struct<(u32, i32)>) $3 = Struct<(u32, i32)> { x: (8888, -8888) } 
[01:29:06] print arg1
[01:29:06] (int) $4 = -3 
[01:29:06] print arg2
[01:29:06] (short) $5 = -4 
[01:29:06] continue
[01:29:06] Hit breakpoint 1.1: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_ref::h04f267fd97e6c00d + 15 at generic-method-on-generic-struct.rs:151, address = 0x000026cf, resolved, hit count = 1 
[01:29:06] print *self
[01:29:06] (generic_method_on_generic_struct::Struct<double>) $6 = Struct<double> { x: 1234.5 } 
[01:29:06] print arg1
[01:29:06] (int) $7 = -5 
[01:29:06] print arg2
[01:29:06] (int) $8 = -6 
[01:29:06] continue
[01:29:06] Hit breakpoint 2.2: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_by_val::hbb45b81426683031 + 20 at generic-method-on-generic-struct.rs:156, address = 0x000027d4, resolved, hit count = 1 
[01:29:06] print self
[01:29:06] (generic_method_on_generic_struct::Struct<double>) $9 = Struct<double> { x: 1234.5 } 
[01:29:06] print arg1
[01:29:06] (int) $10 = -7 
[01:29:06] print arg2
[01:29:06] (long long) $11 = -8 
[01:29:06] continue
[01:29:06] Hit breakpoint 3.1: where = a`_$LT$generic_method_on_generic_struct..Struct$LT$T1$GT$$GT$::self_owned::hd277517aab233e26 + 17 at generic-method-on-generic-struct.rs:161, address = 0x00002831, resolved, hit count = 1 
[01:29:06] print *self
[01:29:06] (generic_method_on_generic_struct::Struct<double>) $12 = Struct<double> { x: 1234.5 } 
[01:29:06] print arg1
[01:29:06] (int) $13 = -9 
[01:29:06] print arg2
[01:29:06] (float) $14 = -10.5 
[01:29:06] continue
[01:29:06] quit
[01:29:06] None
[01:29:06] 
[01:29:06] ------------------------------------------
[01:29:06] stderr:
[01:29:06] ------------------------------------------
[01:29:06] 
[01:29:06] ------------------------------------------
[01:29:06] 
[01:29:06] thread '[debuginfo-both] debuginfo/generic-method-on-generic-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:29:06] 
[01:29:06] ---- [debuginfo-both] debuginfo/generic-struct.rs stdout ----
[01:29:06] NOTE: compiletest thinks it is using LLDB version 902
[01:29:06] NOTE: compiletest thinks it is using LLDB without native rust support
[01:29:06] 
[01:29:06] error: line not found in debugger output: [...]$0 = AGenericStruct<i32, i32> { key: 0, value: 1 }
[01:29:06] status: exit code: 0
[01:29:06] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-struct/a" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-struct/generic-struct.debugger.script"
[01:29:06] stdout:
[01:29:06] ------------------------------------------
[01:29:06] LLDB batch-mode script
[01:29:06] ----------------------
[01:29:06] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-struct/generic-struct.debugger.script'.
[01:29:06] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-struct/a'.
[01:29:06] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:29:06] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-struct/a'
[01:29:06] settings set auto-confirm true
[01:29:06] 
[01:29:06] version
[01:29:06] lldb-902.0.73.1 Swift-4.1 
[01:29:06] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:29:06] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:29:06] type category enable Rust
[01:29:06] 
[01:29:06] breakpoint set --file 'generic-struct.rs' --line 74
[01:29:06] Breakpoint 1: where = a`generic_struct::main::h8bb33f6c1dcf1f13 + 115 at generic-struct.rs:74, address = 0x00001d13 
[01:29:06] run
[01:29:06] Hit breakpoint 1.1: where = a`generic_struct::main::h8bb33f6c1dcf1f13 + 115 at generic-struct.rs:74, address = 0x00001d13, resolved, hit count = 1 
[01:29:06] Process 70832 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00001d13 a`generic_struct::main::h8bb33f6c1dcf1f13 at generic-struct.rs:74 71 value: AGenericStruct { key: 7, value: 8.5f64 }, 72 }; 73 -> 74 [4m [0mzzz(); // #break 75 } 76 77 fn zzz() { () } Target 0: (a) stopped. Process 70832 launched: '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/generic-struct/a' (i386) 
[01:29:06] print int_int
[01:29:06] (generic_struct::AGenericStruct<int, int>) $0 = AGenericStruct<int, int> { key: 0, value: 1 } 
[01:29:06] print int_float
[01:29:06] (generic_struct::AGenericStruct<int, double>) $1 = AGenericStruct<int, double> { key: 2, value: 3.5 } 
[01:29:06] print float_int
[01:29:06] (generic_struct::AGenericStruct<double, int>) $2 = AGenericStruct<double, int> { key: 4.5, value: 5 } 
[01:29:06] print float_int_float
[01:29:06] (generic_struct::AGenericStruct<double, generic_struct::AGenericStruct<int, double> >) $3 = AGenericStruct<double, generic_struct::AGenericStruct<int, double> > { key: 6.5, value: AGenericStruct<int, double> { key: 7, value: 8.5 } } 
[01:29:06] quit
[01:29:06] None
[01:29:06] 
[01:29:06] ------------------------------------------
[01:29:06] stderr:
[01:29:06] ------------------------------------------
[01:29:06] 
[01:29:06] ------------------------------------------
[01:29:06] 
[01:29:06] thread '[debuginfo-both] debuginfo/generic-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:29:06] 
[01:29:06] ---- [debuginfo-both] debuginfo/method-on-generic-struct.rs stdout ----
[01:29:06] NOTE: compiletest thinks it is using LLDB version 902
[01:29:06] NOTE: compiletest thinks it is using LLDB without native rust support
[01:29:06] 
[01:29:06] error: line not found in debugger output: [...]$6 = Struct<f64> { x: 1234.5 }
[01:29:06] status: exit code: 0
[01:29:06] command: "/usr/bin/python" "/Users/travis/build/rust-lang/rust/src/etc/lldb_batchmode.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/method-on-generic-struct/a" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/method-on-generic-struct/method-on-generic-struct.debugger.script"
[01:29:06] stdout:
[01:29:06] ------------------------------------------
[01:29:06] LLDB batch-mode script
[01:29:06] ----------------------
[01:29:06] Debugger commands script is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/method-on-generic-struct/method-on-generic-struct.debugger.script'.
[01:29:06] Target executable is '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/method-on-generic-struct/a'.
[01:29:06] Current working directory is '/Users/travis/build/rust-lang/rust'
[01:29:06] Creating a target for '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/method-on-generic-struct/a'
[01:29:06] settings set auto-confirm true
[01:29:06] 
[01:29:06] version
[01:29:06] lldb-902.0.73.1 Swift-4.1 
[01:29:06] command script import /Users/travis/build/rust-lang/rust/./src/etc/lldb_rust_formatters.py
[01:29:06] type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
[01:29:06] type category enable Rust
[01:29:06] 
[01:29:06] breakpoint set --file 'method-on-generic-struct.rs' --line 151
[01:29:06] Breakpoint 1: 2 locations. 
[01:29:06] breakpoint set --file 'method-on-generic-struct.rs' --line 156
[01:29:06] Breakpoint 2: 2 locations. 
[01:29:06] breakpoint set --file 'method-on-generic-struct.rs' --line 161
[01:29:06] Breakpoint 3: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_owned::h41b46cf76b654458 + 22 at method-on-generic-struct.rs:161, address = 0x00002756 
[01:29:06] run
[01:29:06] Hit breakpoint 1.2: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_ref::h57cf4021f509d72a + 22 at method-on-generic-struct.rs:151, address = 0x000025f6, resolved, hit count = 1 
[01:29:06] Process 70971 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2 frame #0: 0x000025f6 a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_ref::h57cf4021f509d72a(self=&0xbfffe258, arg1=-1, arg2=-2) at method-on-generic-struct.rs:151 148 impl<T> Struct<T> { 149 150 fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize { -> 151 [4m [0mzzz(); // #break 152 arg1 + arg2 153 } 154 Target 0: (a) stopped. Process 70971 launched: '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/debuginfo/method-on-generic-struct/a' (i386) 
[01:29:06] print *self
[01:29:06] (method_on_generic_struct::Struct<(u32, i32)>) $0 = Struct<(u32, i32)> { x: (8888, -8888) } 
[01:29:06] print arg1
[01:29:06] (int) $1 = -1 
[01:29:06] print arg2
[01:29:06] (int) $2 = -2 
[01:29:06] continue
[01:29:06] Hit breakpoint 2.1: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_val::h3d76adbe1c53070b + 32 at method-on-generic-struct.rs:156, address = 0x00002660, resolved, hit count = 1 
[01:29:06] print self
[01:29:06] (method_on_generic_struct::Struct<(u32, i32)>) $3 = Struct<(u32, i32)> { x: (8888, -8888) } 
[01:29:06] print arg1
[01:29:06] (int) $4 = -3 
[01:29:06] print arg2
[01:29:06] (int) $5 = -4 
[01:29:06] continue
[01:29:06] Hit breakpoint 1.1: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_ref::h55c9054351dbda3b + 22 at method-on-generic-struct.rs:151, address = 0x00002596, resolved, hit count = 1 
[01:29:06] print *self
[01:29:06] (method_on_generic_struct::Struct<double>) $6 = Struct<double> { x: 1234.5 } 
[01:29:06] print arg1
[01:29:06] (int) $7 = -5 
[01:29:06] print arg2
[01:29:06] (int) $8 = -6 
[01:29:06] continue
[01:29:06] Hit breakpoint 2.2: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_by_val::h84a830b984b8414e + 23 at method-on-generic-struct.rs:156, address = 0x000026d7, resolved, hit count = 1 
[01:29:06] print self
[01:29:06] (method_on_generic_struct::Struct<double>) $9 = Struct<double> { x: 1234.5 } 
[01:29:06] print arg1
[01:29:06] (int) $10 = -7 
[01:29:06] print arg2
[01:29:06] (int) $11 = -8 
[01:29:06] continue
[01:29:06] Hit breakpoint 3.1: where = a`_$LT$method_on_generic_struct..Struct$LT$T$GT$$GT$::self_owned::h41b46cf76b654458 + 22 at method-on-generic-struct.rs:161, address = 0x00002756, resolved, hit count = 1 
[01:29:06] print *self
[01:29:06] (method_on_generic_struct::Struct<double>) $12 = Struct<double> { x: 1234.5 } 
[01:29:06] print arg1
[01:29:06] (int) $13 = -9 
[01:29:06] print arg2
[01:29:06] (int) $14 = -10 
[01:29:06] continue
[01:29:06] quit
[01:29:06] None
[01:29:06] 
[01:29:06] ------------------------------------------
[01:29:06] stderr:
[01:29:06] ------------------------------------------
[01:29:06] 
[01:29:06] ------------------------------------------
[01:29:06] 
[01:29:06] thread '[debuginfo-both] debuginfo/method-on-generic-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:29:06] 
[01:29:06] 
[01:29:06] failures:
[01:29:06]     [debuginfo-both] debuginfo/associated-types.rs
[01:29:06]     [debuginfo-both] debuginfo/generic-method-on-generic-struct.rs
[01:29:06]     [debuginfo-both] debuginfo/generic-struct.rs
[01:29:06]     [debuginfo-both] debuginfo/method-on-generic-struct.rs
[01:29:06] 
[01:29:06] test result: [31mFAILED(B[m. 82 passed; 4 failed; 32 ignored; 0 measured; 0 filtered out
