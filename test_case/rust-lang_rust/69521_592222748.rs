plain
2020-02-27T23:03:09.7584770Z test [debuginfo-lldb] debuginfo/pretty-std-collections-hash.rs ... ok
2020-02-27T23:03:10.7307620Z test [debuginfo-lldb] debuginfo/pretty-std-collections.rs ... ok
2020-02-27T23:03:10.7308590Z test [debuginfo-lldb] debuginfo/pretty-std.rs ... ignored
2020-02-27T23:03:11.3411760Z test [debuginfo-lldb] debuginfo/pretty-uninitialized-vec.rs ... ok
2020-02-27T23:03:13.4268770Z test [debuginfo-lldb] debuginfo/rc_arc.rs ... FAILED
2020-02-27T23:03:13.4271020Z test [debuginfo-lldb] debuginfo/recursive-struct.rs ... ignored
2020-02-27T23:03:15.3654080Z test [debuginfo-lldb] debuginfo/self-in-default-method.rs ... FAILED
2020-02-27T23:03:17.4960690Z test [debuginfo-lldb] debuginfo/self-in-generic-default-method.rs ... FAILED
2020-02-27T23:03:19.3397300Z test [debuginfo-lldb] debuginfo/shadowed-argument.rs ... ok
---
2020-02-27T23:04:00.9198270Z ---- [debuginfo-lldb] debuginfo/borrowed-struct.rs stdout ----
2020-02-27T23:04:00.9198920Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9199560Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9199980Z 
2020-02-27T23:04:00.9200530Z error: line not found in debugger output: [...]$0 = SomeStruct { x: 10, y: 23.5 }
2020-02-27T23:04:00.9201160Z status: exit code: 0
2020-02-27T23:04:00.9203530Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/borrowed-struct.debugger.script"
2020-02-27T23:04:00.9206980Z ------------------------------------------
2020-02-27T23:04:00.9207350Z 
2020-02-27T23:04:00.9207350Z 
2020-02-27T23:04:00.9208910Z Hit breakpoint 1.1: where = a`borrowed_struct::main::hc884a24258a3a585 + 129 at borrowed-struct.rs:87:4, address = 0x0000000100001791, resolved, hit count = 1 
2020-02-27T23:04:00.9210260Z LLDB batch-mode script
2020-02-27T23:04:00.9211250Z ----------------------
2020-02-27T23:04:00.9212930Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/borrowed-struct.debugger.script'.
2020-02-27T23:04:00.9214810Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/a'.
2020-02-27T23:04:00.9216250Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9217850Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/a'
2020-02-27T23:04:00.9219370Z settings set auto-confirm true
2020-02-27T23:04:00.9219980Z version
2020-02-27T23:04:00.9219980Z version
2020-02-27T23:04:00.9221220Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9221970Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9223440Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9224550Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9224960Z error: empty summary strings not allowed
2020-02-27T23:04:00.9225350Z type category enable Rust
2020-02-27T23:04:00.9225520Z 
2020-02-27T23:04:00.9226110Z breakpoint set --file 'borrowed-struct.rs' --line 87
2020-02-27T23:04:00.9226110Z breakpoint set --file 'borrowed-struct.rs' --line 87
2020-02-27T23:04:00.9227090Z Breakpoint 1: where = a`borrowed_struct::main::hc884a24258a3a585 + 129 at borrowed-struct.rs:87:4, address = 0x0000000100001791 
2020-02-27T23:04:00.9227570Z run
2020-02-27T23:04:00.9229640Z Process 63907 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001791 a`borrowed_struct::main::hc884a24258a3a585 at borrowed-struct.rs:87:4 84 let unique_val_interior_ref_1: &isize = &unique_val.x; 85 let unique_val_interior_ref_2: &f64 = &unique_val.y; 86 -> 87 zzz(); // #break ^ 88 } 89 90 fn zzz() {()} Target 0: (a) stopped. Process 63907 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-struct.lldb/a' (x86_64) 
2020-02-27T23:04:00.9231030Z print *stack_val_ref
2020-02-27T23:04:00.9231400Z (borrowed_struct::SomeStruct) $0 = { x = 10 y = 23.5 } 
2020-02-27T23:04:00.9231900Z print *stack_val_interior_ref_1
2020-02-27T23:04:00.9232480Z (long) $1 = 10 
2020-02-27T23:04:00.9232760Z print *stack_val_interior_ref_2
2020-02-27T23:04:00.9233000Z (double) $2 = 23.5 
2020-02-27T23:04:00.9233230Z print *ref_to_unnamed
2020-02-27T23:04:00.9233530Z (borrowed_struct::SomeStruct) $3 = { x = 11 y = 24.5 } 
2020-02-27T23:04:00.9233850Z print *unique_val_ref
2020-02-27T23:04:00.9234160Z (borrowed_struct::SomeStruct) $4 = { x = 13 y = 26.5 } 
2020-02-27T23:04:00.9234470Z print *unique_val_interior_ref_1
2020-02-27T23:04:00.9234710Z (long) $5 = 13 
2020-02-27T23:04:00.9234940Z print *unique_val_interior_ref_2
2020-02-27T23:04:00.9235190Z (double) $6 = 26.5 
2020-02-27T23:04:00.9235540Z None
2020-02-27T23:04:00.9235660Z 
2020-02-27T23:04:00.9236680Z ------------------------------------------
2020-02-27T23:04:00.9237070Z stderr:
2020-02-27T23:04:00.9237070Z stderr:
2020-02-27T23:04:00.9237960Z ------------------------------------------
2020-02-27T23:04:00.9238710Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9239720Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9240620Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9241370Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9242180Z ------------------------------------------
2020-02-27T23:04:00.9242410Z 
2020-02-27T23:04:00.9242520Z 
2020-02-27T23:04:00.9243120Z ---- [debuginfo-lldb] debuginfo/borrowed-tuple.rs stdout ----
2020-02-27T23:04:00.9243120Z ---- [debuginfo-lldb] debuginfo/borrowed-tuple.rs stdout ----
2020-02-27T23:04:00.9243780Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9244170Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9244440Z 
2020-02-27T23:04:00.9245140Z error: line not found in debugger output: [...]$0 = (-14, -19)
2020-02-27T23:04:00.9245510Z status: exit code: 0
2020-02-27T23:04:00.9246950Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/borrowed-tuple.debugger.script"
2020-02-27T23:04:00.9248360Z ------------------------------------------
2020-02-27T23:04:00.9248570Z 
2020-02-27T23:04:00.9248570Z 
2020-02-27T23:04:00.9249500Z Hit breakpoint 1.1: where = a`borrowed_tuple::main::h71018c9155b274c9 + 79 at borrowed-tuple.rs:52:4, address = 0x00000001000015ff, resolved, hit count = 1 
2020-02-27T23:04:00.9250350Z LLDB batch-mode script
2020-02-27T23:04:00.9250900Z ----------------------
2020-02-27T23:04:00.9251830Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/borrowed-tuple.debugger.script'.
2020-02-27T23:04:00.9252960Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/a'.
2020-02-27T23:04:00.9253860Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9254800Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/a'
2020-02-27T23:04:00.9255580Z settings set auto-confirm true
2020-02-27T23:04:00.9255930Z version
2020-02-27T23:04:00.9255930Z version
2020-02-27T23:04:00.9256640Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9257140Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9257960Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9258740Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9259130Z error: empty summary strings not allowed
2020-02-27T23:04:00.9259530Z type category enable Rust
2020-02-27T23:04:00.9259690Z 
2020-02-27T23:04:00.9260280Z breakpoint set --file 'borrowed-tuple.rs' --line 52
2020-02-27T23:04:00.9260280Z breakpoint set --file 'borrowed-tuple.rs' --line 52
2020-02-27T23:04:00.9261230Z Breakpoint 1: where = a`borrowed_tuple::main::h71018c9155b274c9 + 79 at borrowed-tuple.rs:52:4, address = 0x00000001000015ff 
2020-02-27T23:04:00.9261690Z run
2020-02-27T23:04:00.9263690Z Process 63930 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000015ff a`borrowed_tuple::main::h71018c9155b274c9 at borrowed-tuple.rs:52:4 49 let unique_val: Box<(i16, f32)> = box (-17, -22f32); 50 let unique_val_ref: &(i16, f32) = &*unique_val; 51 -> 52 zzz(); // #break ^ 53 } 54 55 fn zzz() {()} Target 0: (a) stopped. Process 63930 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/borrowed-tuple.lldb/a' (x86_64) 
2020-02-27T23:04:00.9265040Z print *stack_val_ref
2020-02-27T23:04:00.9265920Z ((i16, f32)) $0 = { 0 = -14 1 = -19 } 
2020-02-27T23:04:00.9266200Z print *ref_to_unnamed
2020-02-27T23:04:00.9266800Z ((i16, f32)) $1 = { 0 = -15 1 = -20 } 
2020-02-27T23:04:00.9267080Z print *unique_val_ref
2020-02-27T23:04:00.9267820Z ((i16, f32)) $2 = { 0 = -17 1 = -22 } 
2020-02-27T23:04:00.9268250Z None
2020-02-27T23:04:00.9268370Z 
2020-02-27T23:04:00.9268960Z ------------------------------------------
2020-02-27T23:04:00.9269230Z stderr:
2020-02-27T23:04:00.9269230Z stderr:
2020-02-27T23:04:00.9269770Z ------------------------------------------
2020-02-27T23:04:00.9270530Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9271250Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9271980Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9272730Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9273630Z ------------------------------------------
2020-02-27T23:04:00.9273850Z 
2020-02-27T23:04:00.9273960Z 
2020-02-27T23:04:00.9274650Z ---- [debuginfo-lldb] debuginfo/box.rs stdout ----
2020-02-27T23:04:00.9274650Z ---- [debuginfo-lldb] debuginfo/box.rs stdout ----
2020-02-27T23:04:00.9275030Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9275430Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9275680Z 
2020-02-27T23:04:00.9275970Z error: line not found in debugger output: [...]$1 = (2, 3.5)
2020-02-27T23:04:00.9276290Z status: exit code: 0
2020-02-27T23:04:00.9277650Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/box.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/box.lldb/box.debugger.script"
2020-02-27T23:04:00.9279000Z ------------------------------------------
2020-02-27T23:04:00.9279230Z 
2020-02-27T23:04:00.9279230Z 
2020-02-27T23:04:00.9279700Z Hit breakpoint 1.1: where = a`box::main::h024c49bb7a941519 + 73 at box.rs:35:4, address = 0x0000000100001059, resolved, hit count = 1 
2020-02-27T23:04:00.9280510Z LLDB batch-mode script
2020-02-27T23:04:00.9281040Z ----------------------
2020-02-27T23:04:00.9282020Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/box.lldb/box.debugger.script'.
2020-02-27T23:04:00.9283110Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/box.lldb/a'.
2020-02-27T23:04:00.9284010Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9284940Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/box.lldb/a'
2020-02-27T23:04:00.9286310Z settings set auto-confirm true
2020-02-27T23:04:00.9286680Z version
2020-02-27T23:04:00.9286680Z version
2020-02-27T23:04:00.9287400Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9287930Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9289660Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9291070Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9291490Z error: empty summary strings not allowed
2020-02-27T23:04:00.9291890Z type category enable Rust
2020-02-27T23:04:00.9292060Z 
2020-02-27T23:04:00.9292060Z 
2020-02-27T23:04:00.9292660Z breakpoint set --file 'box.rs' --line 35
2020-02-27T23:04:00.9293140Z Breakpoint 1: where = a`box::main::h024c49bb7a941519 + 73 at box.rs:35:4, address = 0x0000000100001059 
2020-02-27T23:04:00.9293630Z run
2020-02-27T23:04:00.9295890Z Process 63976 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001059 a`box::main::h024c49bb7a941519 at box.rs:35:4 32 let a = box 1; 33 let b = box (2, 3.5f64); 34 -> 35 zzz(); // #break ^ 36 } 37 38 fn zzz() { () } Target 0: (a) stopped. Process 63976 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/box.lldb/a' (x86_64) 
2020-02-27T23:04:00.9297020Z print *a
2020-02-27T23:04:00.9297210Z (int) $0 = 1 
2020-02-27T23:04:00.9297390Z print *b
2020-02-27T23:04:00.9297630Z ((i32, f64)) $1 = { 0 = 2 1 = 3.5 } 
2020-02-27T23:04:00.9298020Z None
2020-02-27T23:04:00.9298140Z 
2020-02-27T23:04:00.9298750Z ------------------------------------------
2020-02-27T23:04:00.9299000Z stderr:
2020-02-27T23:04:00.9299000Z stderr:
2020-02-27T23:04:00.9299570Z ------------------------------------------
2020-02-27T23:04:00.9300330Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9301050Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9301790Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9302520Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9303270Z ------------------------------------------
2020-02-27T23:04:00.9303500Z 
2020-02-27T23:04:00.9303610Z 
2020-02-27T23:04:00.9304220Z ---- [debuginfo-lldb] debuginfo/boxed-struct.rs stdout ----
2020-02-27T23:04:00.9304220Z ---- [debuginfo-lldb] debuginfo/boxed-struct.rs stdout ----
2020-02-27T23:04:00.9304600Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9305010Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9305260Z 
2020-02-27T23:04:00.9305680Z error: line not found in debugger output: [...]$0 = StructWithSomePadding { x: 99, y: 999, z: 9999, w: 99999 }
2020-02-27T23:04:00.9306140Z status: exit code: 0
2020-02-27T23:04:00.9308360Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/boxed-struct.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/boxed-struct.lldb/boxed-struct.debugger.script"
2020-02-27T23:04:00.9309830Z ------------------------------------------
2020-02-27T23:04:00.9310040Z 
2020-02-27T23:04:00.9310040Z 
2020-02-27T23:04:00.9310980Z Hit breakpoint 1.1: where = a`boxed_struct::main::h570ceadb55e29a07 + 112 at boxed-struct.rs:60:4, address = 0x00000001000010b0, resolved, hit count = 1 
2020-02-27T23:04:00.9312790Z LLDB batch-mode script
2020-02-27T23:04:00.9313340Z ----------------------
2020-02-27T23:04:00.9314280Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/boxed-struct.lldb/boxed-struct.debugger.script'.
2020-02-27T23:04:00.9315400Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/boxed-struct.lldb/a'.
2020-02-27T23:04:00.9316290Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9317240Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/boxed-struct.lldb/a'
2020-02-27T23:04:00.9318000Z settings set auto-confirm true
2020-02-27T23:04:00.9318360Z version
2020-02-27T23:04:00.9318360Z version
2020-02-27T23:04:00.9319060Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9319570Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9320450Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9321240Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9321850Z error: empty summary strings not allowed
2020-02-27T23:04:00.9322250Z type category enable Rust
2020-02-27T23:04:00.9322420Z 
2020-02-27T23:04:00.9322420Z 
2020-02-27T23:04:00.9323500Z breakpoint set --file 'boxed-struct.rs' --line 60
2020-02-27T23:04:00.9324510Z Breakpoint 1: where = a`boxed_struct::main::h570ceadb55e29a07 + 112 at boxed-struct.rs:60:4, address = 0x00000001000010b0 
2020-02-27T23:04:00.9325220Z run
2020-02-27T23:04:00.9327650Z Process 63999 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000010b0 a`boxed_struct::main::h570ceadb55e29a07 at boxed-struct.rs:60:4 57 let boxed_with_padding: Box<_> = box StructWithSomePadding { x: 99, y: 999, z: 9999, w: 99999 }; 58 59 let boxed_with_dtor: Box<_> = box StructWithDestructor { x: 77, y: 777, z: 7777, w: 77777 }; -> 60 zzz(); // #break ^ 61 } 62 63 fn zzz() { () } Target 0: (a) stopped. Process 63999 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/boxed-struct.lldb/a' (x86_64) 
2020-02-27T23:04:00.9329230Z print *boxed_with_padding
2020-02-27T23:04:00.9329590Z (boxed_struct::StructWithSomePadding) $0 = { x = 99 y = 999 z = 9999 w = 99999 } 
2020-02-27T23:04:00.9329960Z print *boxed_with_dtor
2020-02-27T23:04:00.9330320Z (boxed_struct::StructWithDestructor) $1 = { x = 77 y = 777 z = 7777 w = 77777 } 
2020-02-27T23:04:00.9330800Z None
2020-02-27T23:04:00.9330920Z 
2020-02-27T23:04:00.9331510Z ------------------------------------------
2020-02-27T23:04:00.9331770Z stderr:
2020-02-27T23:04:00.9331770Z stderr:
2020-02-27T23:04:00.9332320Z ------------------------------------------
2020-02-27T23:04:00.9333080Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9333800Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9334530Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9335260Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9336020Z ------------------------------------------
2020-02-27T23:04:00.9336230Z 
2020-02-27T23:04:00.9336340Z 
2020-02-27T23:04:00.9337020Z ---- [debuginfo-lldb] debuginfo/by-value-self-argument-in-trait-impl.rs stdout ----
2020-02-27T23:04:00.9337020Z ---- [debuginfo-lldb] debuginfo/by-value-self-argument-in-trait-impl.rs stdout ----
2020-02-27T23:04:00.9337470Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9337860Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9338130Z 
2020-02-27T23:04:00.9338460Z error: line not found in debugger output: [...]$1 = Struct { x: 2222, y: 3333 }
2020-02-27T23:04:00.9338840Z status: exit code: 0
2020-02-27T23:04:00.9340470Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/by-value-self-argument-in-trait-impl.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/by-value-self-argument-in-trait-impl.lldb/by-value-self-argument-in-trait-impl.debugger.script"
2020-02-27T23:04:00.9342040Z ------------------------------------------
2020-02-27T23:04:00.9342250Z 
2020-02-27T23:04:00.9342250Z 
2020-02-27T23:04:00.9343720Z Hit breakpoint 1.1: where = a`_$LT$isize$u20$as$u20$by_value_self_argument_in_trait_impl..Trait$GT$::method::h1b688296484a6ff3 + 16 at by-value-self-argument-in-trait-impl.rs:52:8, address = 0x0000000100000b40, resolved, hit count = 1 
2020-02-27T23:04:00.9344420Z 
2020-02-27T23:04:00.9345780Z Hit breakpoint 2.1: where = a`_$LT$by_value_self_argument_in_trait_impl..Struct$u20$as$u20$by_value_self_argument_in_trait_impl..Trait$GT$::method::h4c3ab6fb05fcda36 + 24 at by-value-self-argument-in-trait-impl.rs:64:8, address = 0x0000000100000b68, resolved, hit count = 1 
2020-02-27T23:04:00.9346900Z 
2020-02-27T23:04:00.9348420Z Hit breakpoint 3.1: where = a`_$LT$$LP$f64$C$$u20$isize$C$$u20$isize$C$$u20$f64$RP$$u20$as$u20$by_value_self_argument_in_trait_impl..Trait$GT$::method::hb9f502718c183cbb + 23 at by-value-self-argument-in-trait-impl.rs:71:8, address = 0x0000000100000b97, resolved, hit count = 1 
2020-02-27T23:04:00.9349640Z LLDB batch-mode script
2020-02-27T23:04:00.9350180Z ----------------------
2020-02-27T23:04:00.9351210Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/by-value-self-argument-in-trait-impl.lldb/by-value-self-argument-in-trait-impl.debugger.script'.
2020-02-27T23:04:00.9352440Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/by-value-self-argument-in-trait-impl.lldb/a'.
2020-02-27T23:04:00.9354070Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9355930Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/by-value-self-argument-in-trait-impl.lldb/a'
2020-02-27T23:04:00.9357290Z settings set auto-confirm true
2020-02-27T23:04:00.9357900Z version
2020-02-27T23:04:00.9357900Z version
2020-02-27T23:04:00.9359130Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9359890Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9361290Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9362610Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9363450Z error: empty summary strings not allowed
2020-02-27T23:04:00.9364290Z type category enable Rust
2020-02-27T23:04:00.9364900Z 
2020-02-27T23:04:00.9364900Z 
2020-02-27T23:04:00.9365990Z breakpoint set --file 'by-value-self-argument-in-trait-impl.rs' --line 52
2020-02-27T23:04:00.9367460Z Breakpoint 1: where = a`_$LT$isize$u20$as$u20$by_value_self_argument_in_trait_impl..Trait$GT$::method::h1b688296484a6ff3 + 16 at by-value-self-argument-in-trait-impl.rs:52:8, address = 0x0000000100000b40 
2020-02-27T23:04:00.9368760Z breakpoint set --file 'by-value-self-argument-in-trait-impl.rs' --line 64
2020-02-27T23:04:00.9372280Z Breakpoint 2: where = a`_$LT$by_value_self_argument_in_trait_impl..Struct$u20$as$u20$by_value_self_argument_in_trait_impl..Trait$GT$::method::h4c3ab6fb05fcda36 + 24 at by-value-self-argument-in-trait-impl.rs:64:8, address = 0x0000000100000b68 
2020-02-27T23:04:00.9374880Z breakpoint set --file 'by-value-self-argument-in-trait-impl.rs' --line 71
2020-02-27T23:04:00.9377440Z Breakpoint 3: where = a`_$LT$$LP$f64$C$$u20$isize$C$$u20$isize$C$$u20$f64$RP$$u20$as$u20$by_value_self_argument_in_trait_impl..Trait$GT$::method::hb9f502718c183cbb + 23 at by-value-self-argument-in-trait-impl.rs:71:8, address = 0x0000000100000b97 
2020-02-27T23:04:00.9378900Z run
2020-02-27T23:04:00.9381520Z Process 64022 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000b40 a`_$LT$isize$u20$as$u20$by_value_self_argument_in_trait_impl..Trait$GT$::method::h1b688296484a6ff3(self=1111) at by-value-self-argument-in-trait-impl.rs:52:8 49 50 impl Trait for isize { 51 fn method(self) -> isize { -> 52 zzz(); // #break ^ 53 self 54 } 55 } Target 0: (a) stopped. Process 64022 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/by-value-self-argument-in-trait-impl.lldb/a' (x86_64) 
2020-02-27T23:04:00.9384190Z (long) $0 = 1111 
2020-02-27T23:04:00.9384490Z continue
2020-02-27T23:04:00.9384770Z print self
2020-02-27T23:04:00.9384770Z print self
2020-02-27T23:04:00.9385290Z (by_value_self_argument_in_trait_impl::Struct) $1 = { x = 2222 y = 3333 } 
2020-02-27T23:04:00.9386060Z print self
2020-02-27T23:04:00.9386060Z print self
2020-02-27T23:04:00.9386560Z ((f64, isize, isize, f64)) $2 = { 0 = 4444.5 1 = 5555 2 = 6666 3 = 7777.5 } 
2020-02-27T23:04:00.9387640Z quit
2020-02-27T23:04:00.9387880Z None
2020-02-27T23:04:00.9388070Z 
2020-02-27T23:04:00.9388940Z ------------------------------------------
2020-02-27T23:04:00.9388940Z ------------------------------------------
2020-02-27T23:04:00.9389560Z stderr:
2020-02-27T23:04:00.9390490Z ------------------------------------------
2020-02-27T23:04:00.9391580Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9392650Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9393980Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9395590Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9397050Z ------------------------------------------
2020-02-27T23:04:00.9397270Z 
2020-02-27T23:04:00.9397400Z 
2020-02-27T23:04:00.9398040Z ---- [debuginfo-lldb] debuginfo/c-style-enum-in-composite.rs stdout ----
2020-02-27T23:04:00.9398040Z ---- [debuginfo-lldb] debuginfo/c-style-enum-in-composite.rs stdout ----
2020-02-27T23:04:00.9398480Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9398880Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9399140Z 
2020-02-27T23:04:00.9399430Z error: line not found in debugger output: [...]$0 = (0, OneHundred)
2020-02-27T23:04:00.9399780Z status: exit code: 0
2020-02-27T23:04:00.9401300Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/c-style-enum-in-composite.debugger.script"
2020-02-27T23:04:00.9404500Z ------------------------------------------
2020-02-27T23:04:00.9404860Z 
2020-02-27T23:04:00.9404860Z 
2020-02-27T23:04:00.9406670Z Hit breakpoint 1.1: where = a`c_style_enum_in_composite::main::h215c7f6b7ead7eea + 365 at c-style-enum-in-composite.rs:153:4, address = 0x0000000100000aad, resolved, hit count = 1 
2020-02-27T23:04:00.9407620Z LLDB batch-mode script
2020-02-27T23:04:00.9408190Z ----------------------
2020-02-27T23:04:00.9409840Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/c-style-enum-in-composite.debugger.script'.
2020-02-27T23:04:00.9411820Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a'.
2020-02-27T23:04:00.9413530Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9415210Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a'
2020-02-27T23:04:00.9416670Z settings set auto-confirm true
2020-02-27T23:04:00.9417320Z version
2020-02-27T23:04:00.9417320Z version
2020-02-27T23:04:00.9418470Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9419270Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9420560Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9421770Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9422630Z error: empty summary strings not allowed
2020-02-27T23:04:00.9423280Z type category enable Rust
2020-02-27T23:04:00.9423540Z 
2020-02-27T23:04:00.9423540Z 
2020-02-27T23:04:00.9424520Z breakpoint set --file 'c-style-enum-in-composite.rs' --line 153
2020-02-27T23:04:00.9426200Z Breakpoint 1: where = a`c_style_enum_in_composite::main::h215c7f6b7ead7eea + 365 at c-style-enum-in-composite.rs:153:4, address = 0x0000000100000aad 
2020-02-27T23:04:00.9427330Z run
2020-02-27T23:04:00.9431380Z Process 64045 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000aad a`c_style_enum_in_composite::main::h215c7f6b7ead7eea at c-style-enum-in-composite.rs:153:4 150 151 let struct_with_drop = (StructWithDrop { a: OneHundred, b: Vienna }, 9_i64); 152 -> 153 zzz(); // #break ^ 154 } 155 156 fn zzz() { () } Target 0: (a) stopped. Process 64045 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/c-style-enum-in-composite.lldb/a' (x86_64) 
2020-02-27T23:04:00.9433680Z print tuple_interior_padding
2020-02-27T23:04:00.9434290Z ((i16, c_style_enum_in_composite::AnEnum)) $0 = { 0 = 0 1 = OneHundred } 
2020-02-27T23:04:00.9434830Z print tuple_padding_at_end
2020-02-27T23:04:00.9435490Z (((u64, c_style_enum_in_composite::AnEnum), u64)) $1 = { 0 = { 0 = 1 1 = OneThousand } 1 = 2 } 
2020-02-27T23:04:00.9436260Z print tuple_different_enums
2020-02-27T23:04:00.9437350Z ((c_style_enum_in_composite::AnEnum, c_style_enum_in_composite::AnotherEnum, c_style_enum_in_composite::AnEnum, c_style_enum_in_composite::AnotherEnum)) $2 = { 0 = OneThousand 1 = MountainView 2 = OneMillion 3 = Vienna } 
2020-02-27T23:04:00.9440410Z print padded_struct
2020-02-27T23:04:00.9441070Z (c_style_enum_in_composite::PaddedStruct) $3 = { a = 3 b = OneMillion c = 4 d = Toronto e = 5 } 
2020-02-27T23:04:00.9441700Z print packed_struct
2020-02-27T23:04:00.9442350Z (c_style_enum_in_composite::PackedStruct) $4 = { a = 6 b = OneHundred c = 7 d = Vienna e = 8 } 
2020-02-27T23:04:00.9442980Z print non_padded_struct
2020-02-27T23:04:00.9443790Z (c_style_enum_in_composite::NonPaddedStruct) $5 = { a = OneMillion b = MountainView c = OneThousand d = Toronto } 
2020-02-27T23:04:00.9444520Z print struct_with_drop
2020-02-27T23:04:00.9445180Z ((c_style_enum_in_composite::StructWithDrop, i64)) $6 = { 0 = { a = OneHundred b = Vienna } 1 = 9 } 
2020-02-27T23:04:00.9446100Z None
2020-02-27T23:04:00.9446320Z 
2020-02-27T23:04:00.9447490Z ------------------------------------------
2020-02-27T23:04:00.9447940Z stderr:
2020-02-27T23:04:00.9447940Z stderr:
2020-02-27T23:04:00.9449000Z ------------------------------------------
2020-02-27T23:04:00.9450180Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9451310Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9452450Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9454280Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9455740Z ------------------------------------------
2020-02-27T23:04:00.9456130Z 
2020-02-27T23:04:00.9456350Z 
2020-02-27T23:04:00.9457490Z ---- [debuginfo-lldb] debuginfo/cross-crate-spans.rs stdout ----
2020-02-27T23:04:00.9457490Z ---- [debuginfo-lldb] debuginfo/cross-crate-spans.rs stdout ----
2020-02-27T23:04:00.9458130Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9458810Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9459240Z 
2020-02-27T23:04:00.9459740Z error: line not found in debugger output: [...]$0 = (17, 17)
2020-02-27T23:04:00.9460300Z status: exit code: 0
2020-02-27T23:04:00.9462940Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/cross-crate-spans.debugger.script"
2020-02-27T23:04:00.9465370Z ------------------------------------------
2020-02-27T23:04:00.9465710Z 
2020-02-27T23:04:00.9465710Z 
2020-02-27T23:04:00.9466630Z Hit breakpoint 1.2: where = a`cross_crate_spans::generic_function::hcd64f6ea46e3df62 + 85 at cross_crate_spans.rs:14:4, address = 0x00000001000009c5, resolved, hit count = 1 
2020-02-27T23:04:00.9467790Z 
2020-02-27T23:04:00.9468930Z Hit breakpoint 1.1: where = a`cross_crate_spans::generic_function::h6d7b9784986a79b6 + 92 at cross_crate_spans.rs:14:4, address = 0x000000010000091c, resolved, hit count = 1 
2020-02-27T23:04:00.9470490Z LLDB batch-mode script
2020-02-27T23:04:00.9471370Z ----------------------
2020-02-27T23:04:00.9472950Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/cross-crate-spans.debugger.script'.
2020-02-27T23:04:00.9474980Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/a'.
2020-02-27T23:04:00.9476530Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9478110Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/a'
2020-02-27T23:04:00.9479460Z settings set auto-confirm true
2020-02-27T23:04:00.9480050Z version
2020-02-27T23:04:00.9480050Z version
2020-02-27T23:04:00.9481280Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9482090Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9483630Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9484990Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9485610Z error: empty summary strings not allowed
2020-02-27T23:04:00.9486270Z type category enable Rust
2020-02-27T23:04:00.9486550Z 
2020-02-27T23:04:00.9486900Z b cross_crate_spans.rs:14
2020-02-27T23:04:00.9487320Z Breakpoint 1: 2 locations. 
2020-02-27T23:04:00.9487320Z Breakpoint 1: 2 locations. 
2020-02-27T23:04:00.9487700Z run
2020-02-27T23:04:00.9491150Z Process 64151 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2 frame #0: 0x00000001000009c5 a`cross_crate_spans::generic_function::hcd64f6ea46e3df62(val=17) at cross_crate_spans.rs:14:4 11 let result = (val.clone(), val.clone()); 12 let a_variable: u32 = 123456789; 13 let another_variable: f64 = 123456789.5; -> 14 zzz(); ^ 15 result 16 } 17 Target 0: (a) stopped. Process 64151 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/cross-crate-spans.lldb/a' (x86_64) 
2020-02-27T23:04:00.9493400Z print result
2020-02-27T23:04:00.9493980Z ((u32, u32)) $0 = { 0 = 17 1 = 17 } 
2020-02-27T23:04:00.9494410Z print a_variable
2020-02-27T23:04:00.9494840Z (unsigned int) $1 = 123456789 
2020-02-27T23:04:00.9495270Z print another_variable
2020-02-27T23:04:00.9495710Z (double) $2 = 123456789.5 
2020-02-27T23:04:00.9496420Z print result
2020-02-27T23:04:00.9496420Z print result
2020-02-27T23:04:00.9496820Z ((i16, i16)) $3 = { 0 = 1212 1 = 1212 } 
2020-02-27T23:04:00.9497370Z print a_variable
2020-02-27T23:04:00.9497780Z (unsigned int) $4 = 123456789 
2020-02-27T23:04:00.9498200Z print another_variable
2020-02-27T23:04:00.9498590Z (double) $5 = 123456789.5 
2020-02-27T23:04:00.9499290Z quit
2020-02-27T23:04:00.9499590Z None
2020-02-27T23:04:00.9499810Z 
2020-02-27T23:04:00.9500960Z ------------------------------------------
2020-02-27T23:04:00.9500960Z ------------------------------------------
2020-02-27T23:04:00.9501410Z stderr:
2020-02-27T23:04:00.9502450Z ------------------------------------------
2020-02-27T23:04:00.9503690Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9504830Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9506100Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9507570Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9508700Z ------------------------------------------
2020-02-27T23:04:00.9509000Z 
2020-02-27T23:04:00.9509410Z 
2020-02-27T23:04:00.9511010Z ---- [debuginfo-lldb] debuginfo/destructured-fn-argument.rs stdout ----
2020-02-27T23:04:00.9511010Z ---- [debuginfo-lldb] debuginfo/destructured-fn-argument.rs stdout ----
2020-02-27T23:04:00.9511610Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9512200Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9512570Z 
2020-02-27T23:04:00.9512990Z error: line not found in debugger output: [...]$6 = (6, 7)
2020-02-27T23:04:00.9513510Z status: exit code: 0
2020-02-27T23:04:00.9515760Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-fn-argument.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-fn-argument.lldb/destructured-fn-argument.debugger.script"
2020-02-27T23:04:00.9517920Z ------------------------------------------
2020-02-27T23:04:00.9518240Z 
2020-02-27T23:04:00.9518240Z 
2020-02-27T23:04:00.9519800Z Hit breakpoint 1.1: where = a`destructured_fn_argument::simple_tuple::h3d90da7c1e59e8e8 + 32 at destructured-fn-argument.rs:380:4, address = 0x00000001000013c0, resolved, hit count = 1 
2020-02-27T23:04:00.9520970Z 
2020-02-27T23:04:00.9522410Z Hit breakpoint 2.1: where = a`destructured_fn_argument::nested_tuple::hc78c17578d98556a + 31 at destructured-fn-argument.rs:384:4, address = 0x00000001000013ef, resolved, hit count = 1 
2020-02-27T23:04:00.9522990Z 
2020-02-27T23:04:00.9524050Z Hit breakpoint 3.1: where = a`destructured_fn_argument::destructure_only_first_level::h2bc7ae8ade801c85 + 27 at destructured-fn-argument.rs:388:4, address = 0x000000010000141b, resolved, hit count = 1 
2020-02-27T23:04:00.9524660Z 
2020-02-27T23:04:00.9525700Z Hit breakpoint 4.1: where = a`destructured_fn_argument::struct_as_tuple_element::h2f2ca250efb5df20 + 37 at destructured-fn-argument.rs:392:4, address = 0x0000000100001455, resolved, hit count = 1 
2020-02-27T23:04:00.9526430Z 
2020-02-27T23:04:00.9527430Z Hit breakpoint 5.1: where = a`destructured_fn_argument::struct_pattern::h4ff26120d36d2f40 + 22 at destructured-fn-argument.rs:396:4, address = 0x0000000100001476, resolved, hit count = 1 
2020-02-27T23:04:00.9527990Z 
2020-02-27T23:04:00.9529000Z Hit breakpoint 6.1: where = a`destructured_fn_argument::ignored_tuple_element::h9c1541d5afd0826e + 21 at destructured-fn-argument.rs:400:4, address = 0x00000001000014a5, resolved, hit count = 1 
2020-02-27T23:04:00.9529570Z 
2020-02-27T23:04:00.9530570Z Hit breakpoint 7.1: where = a`destructured_fn_argument::ignored_struct_field::hcf1a3ce7399281cb + 18 at destructured-fn-argument.rs:404:4, address = 0x00000001000014c2, resolved, hit count = 1 
2020-02-27T23:04:00.9531140Z 
2020-02-27T23:04:00.9532170Z Hit breakpoint 8.1: where = a`destructured_fn_argument::one_struct_destructured_one_not::h195a3c440c45c511 + 35 at destructured-fn-argument.rs:408:4, address = 0x00000001000014f3, resolved, hit count = 1 
2020-02-27T23:04:00.9532780Z 
2020-02-27T23:04:00.9533830Z Hit breakpoint 9.1: where = a`destructured_fn_argument::different_order_of_struct_fields::hea50fdb21e95a6f8 + 22 at destructured-fn-argument.rs:412:4, address = 0x0000000100001516, resolved, hit count = 1 
2020-02-27T23:04:00.9534430Z 
2020-02-27T23:04:00.9535430Z Hit breakpoint 10.1: where = a`destructured_fn_argument::complex_nesting::h9b7d836086f847c5 + 71 at destructured-fn-argument.rs:418:4, address = 0x0000000100001577, resolved, hit count = 1 
2020-02-27T23:04:00.9535990Z 
2020-02-27T23:04:00.9536960Z Hit breakpoint 11.1: where = a`destructured_fn_argument::managed_box::hf0a352fbbd1fc362 + 27 at destructured-fn-argument.rs:422:4, address = 0x00000001000015ab, resolved, hit count = 1 
2020-02-27T23:04:00.9537510Z 
2020-02-27T23:04:00.9538640Z Hit breakpoint 12.1: where = a`destructured_fn_argument::borrowed_pointer::hd41c5f3e1d333655 + 27 at destructured-fn-argument.rs:426:4, address = 0x00000001000015db, resolved, hit count = 1 
2020-02-27T23:04:00.9539470Z 
2020-02-27T23:04:00.9540760Z Hit breakpoint 13.1: where = a`destructured_fn_argument::contained_borrowed_pointer::hac05f1acddd6cbab + 23 at destructured-fn-argument.rs:430:4, address = 0x0000000100001607, resolved, hit count = 1 
2020-02-27T23:04:00.9541520Z 
2020-02-27T23:04:00.9542560Z Hit breakpoint 14.1: where = a`destructured_fn_argument::unique_pointer::h25aa644984bff782 + 39 at destructured-fn-argument.rs:434:4, address = 0x0000000100001647, resolved, hit count = 1 
2020-02-27T23:04:00.9543250Z 
2020-02-27T23:04:00.9544360Z Hit breakpoint 15.1: where = a`destructured_fn_argument::ref_binding::ha01545782a44fac4 + 12 at destructured-fn-argument.rs:438:4, address = 0x000000010000168c, resolved, hit count = 1 
2020-02-27T23:04:00.9544910Z 
2020-02-27T23:04:00.9545960Z Hit breakpoint 16.1: where = a`destructured_fn_argument::ref_binding_in_tuple::hd7d47f21cdfa1137 + 31 at destructured-fn-argument.rs:442:4, address = 0x00000001000016bf, resolved, hit count = 1 
2020-02-27T23:04:00.9546540Z 
2020-02-27T23:04:00.9547600Z Hit breakpoint 17.1: where = a`destructured_fn_argument::ref_binding_in_struct::h04038672a570759d + 29 at destructured-fn-argument.rs:446:4, address = 0x00000001000016ed, resolved, hit count = 1 
2020-02-27T23:04:00.9548180Z 
2020-02-27T23:04:00.9549210Z Hit breakpoint 18.1: where = a`destructured_fn_argument::univariant_enum::h61ddd3d1d5cf2b24 + 17 at destructured-fn-argument.rs:450:4, address = 0x0000000100001711, resolved, hit count = 1 
2020-02-27T23:04:00.9549770Z 
2020-02-27T23:04:00.9550860Z Hit breakpoint 19.1: where = a`destructured_fn_argument::univariant_enum_with_ref_binding::h8c0140ed1a4f7ccd + 19 at destructured-fn-argument.rs:454:4, address = 0x0000000100001733, resolved, hit count = 1 
2020-02-27T23:04:00.9551470Z 
2020-02-27T23:04:00.9552500Z Hit breakpoint 20.1: where = a`destructured_fn_argument::tuple_struct::hc696be94c35c41c6 + 26 at destructured-fn-argument.rs:458:4, address = 0x000000010000175a, resolved, hit count = 1 
2020-02-27T23:04:00.9553050Z 
2020-02-27T23:04:00.9554270Z Hit breakpoint 21.1: where = a`destructured_fn_argument::tuple_struct_with_ref_binding::h45e2477d88e33df4 + 41 at destructured-fn-argument.rs:462:4, address = 0x0000000100001799, resolved, hit count = 1 
2020-02-27T23:04:00.9554860Z 
2020-02-27T23:04:00.9555870Z Hit breakpoint 22.1: where = a`destructured_fn_argument::multiple_arguments::hbb2377f19614f005 + 28 at destructured-fn-argument.rs:466:4, address = 0x00000001000017cc, resolved, hit count = 1 
2020-02-27T23:04:00.9556440Z 
2020-02-27T23:04:00.9557450Z Hit breakpoint 23.1: where = a`destructured_fn_argument::main::nested_function::hcca7c800f9e928aa + 28 at destructured-fn-argument.rs:494:8, address = 0x0000000100001cfc, resolved, hit count = 1 
2020-02-27T23:04:00.9558390Z LLDB batch-mode script
2020-02-27T23:04:00.9558900Z ----------------------
2020-02-27T23:04:00.9559890Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-fn-argument.lldb/destructured-fn-argument.debugger.script'.
2020-02-27T23:04:00.9561060Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-fn-argument.lldb/a'.
2020-02-27T23:04:00.9561940Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9562890Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-fn-argument.lldb/a'
2020-02-27T23:04:00.9564060Z settings set auto-confirm true
2020-02-27T23:04:00.9564570Z version
2020-02-27T23:04:00.9564570Z version
2020-02-27T23:04:00.9565530Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9566230Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9567050Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9568160Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9568530Z error: empty summary strings not allowed
2020-02-27T23:04:00.9569180Z type category enable Rust
2020-02-27T23:04:00.9569330Z 
2020-02-27T23:04:00.9569330Z 
2020-02-27T23:04:00.9569990Z breakpoint set --file 'destructured-fn-argument.rs' --line 380
2020-02-27T23:04:00.9571020Z Breakpoint 1: where = a`destructured_fn_argument::simple_tuple::h3d90da7c1e59e8e8 + 32 at destructured-fn-argument.rs:380:4, address = 0x00000001000013c0 
2020-02-27T23:04:00.9571980Z breakpoint set --file 'destructured-fn-argument.rs' --line 384
2020-02-27T23:04:00.9573020Z Breakpoint 2: where = a`destructured_fn_argument::nested_tuple::hc78c17578d98556a + 31 at destructured-fn-argument.rs:384:4, address = 0x00000001000013ef 
2020-02-27T23:04:00.9573950Z breakpoint set --file 'destructured-fn-argument.rs' --line 388
2020-02-27T23:04:00.9575050Z Breakpoint 3: where = a`destructured_fn_argument::destructure_only_first_level::h2bc7ae8ade801c85 + 27 at destructured-fn-argument.rs:388:4, address = 0x000000010000141b 
2020-02-27T23:04:00.9576050Z breakpoint set --file 'destructured-fn-argument.rs' --line 392
2020-02-27T23:04:00.9577100Z Breakpoint 4: where = a`destructured_fn_argument::struct_as_tuple_element::h2f2ca250efb5df20 + 37 at destructured-fn-argument.rs:392:4, address = 0x0000000100001455 
2020-02-27T23:04:00.9578090Z breakpoint set --file 'destructured-fn-argument.rs' --line 396
2020-02-27T23:04:00.9579110Z Breakpoint 5: where = a`destructured_fn_argument::struct_pattern::h4ff26120d36d2f40 + 22 at destructured-fn-argument.rs:396:4, address = 0x0000000100001476 
2020-02-27T23:04:00.9580080Z breakpoint set --file 'destructured-fn-argument.rs' --line 400
2020-02-27T23:04:00.9581140Z Breakpoint 6: where = a`destructured_fn_argument::ignored_tuple_element::h9c1541d5afd0826e + 21 at destructured-fn-argument.rs:400:4, address = 0x00000001000014a5 
2020-02-27T23:04:00.9582120Z breakpoint set --file 'destructured-fn-argument.rs' --line 404
2020-02-27T23:04:00.9583180Z Breakpoint 7: where = a`destructured_fn_argument::ignored_struct_field::hcf1a3ce7399281cb + 18 at destructured-fn-argument.rs:404:4, address = 0x00000001000014c2 
2020-02-27T23:04:00.9584150Z breakpoint set --file 'destructured-fn-argument.rs' --line 408
2020-02-27T23:04:00.9585230Z Breakpoint 8: where = a`destructured_fn_argument::one_struct_destructured_one_not::h195a3c440c45c511 + 35 at destructured-fn-argument.rs:408:4, address = 0x00000001000014f3 
2020-02-27T23:04:00.9586240Z breakpoint set --file 'destructured-fn-argument.rs' --line 412
2020-02-27T23:04:00.9587320Z Breakpoint 9: where = a`destructured_fn_argument::different_order_of_struct_fields::hea50fdb21e95a6f8 + 22 at destructured-fn-argument.rs:412:4, address = 0x0000000100001516 
2020-02-27T23:04:00.9588320Z breakpoint set --file 'destructured-fn-argument.rs' --line 418
2020-02-27T23:04:00.9589360Z Breakpoint 10: where = a`destructured_fn_argument::complex_nesting::h9b7d836086f847c5 + 71 at destructured-fn-argument.rs:418:4, address = 0x0000000100001577 
2020-02-27T23:04:00.9590320Z breakpoint set --file 'destructured-fn-argument.rs' --line 422
2020-02-27T23:04:00.9591360Z Breakpoint 11: where = a`destructured_fn_argument::managed_box::hf0a352fbbd1fc362 + 27 at destructured-fn-argument.rs:422:4, address = 0x00000001000015ab 
2020-02-27T23:04:00.9592310Z breakpoint set --file 'destructured-fn-argument.rs' --line 426
2020-02-27T23:04:00.9593430Z Breakpoint 12: where = a`destructured_fn_argument::borrowed_pointer::hd41c5f3e1d333655 + 27 at destructured-fn-argument.rs:426:4, address = 0x00000001000015db 
2020-02-27T23:04:00.9594420Z breakpoint set --file 'destructured-fn-argument.rs' --line 430
2020-02-27T23:04:00.9595490Z Breakpoint 13: where = a`destructured_fn_argument::contained_borrowed_pointer::hac05f1acddd6cbab + 23 at destructured-fn-argument.rs:430:4, address = 0x0000000100001607 
2020-02-27T23:04:00.9596480Z breakpoint set --file 'destructured-fn-argument.rs' --line 434
2020-02-27T23:04:00.9597750Z Breakpoint 14: where = a`destructured_fn_argument::unique_pointer::h25aa644984bff782 + 39 at destructured-fn-argument.rs:434:4, address = 0x0000000100001647 
2020-02-27T23:04:00.9598900Z breakpoint set --file 'destructured-fn-argument.rs' --line 438
2020-02-27T23:04:00.9600010Z Breakpoint 15: where = a`destructured_fn_argument::ref_binding::ha01545782a44fac4 + 12 at destructured-fn-argument.rs:438:4, address = 0x000000010000168c 
2020-02-27T23:04:00.9600960Z breakpoint set --file 'destructured-fn-argument.rs' --line 442
---
2020-02-27T23:04:00.9628890Z (long) $15 = 19 
2020-02-27T23:04:00.9629090Z print q
2020-02-27T23:04:00.9629260Z (int) $16 = 20 
2020-02-27T23:04:00.9629440Z print r
2020-02-27T23:04:00.9629710Z (destructured_fn_argument::Struct) $17 = { a = 21 b = 22 } 
2020-02-27T23:04:00.9630160Z print s
2020-02-27T23:04:00.9630340Z (int) $18 = 24 
2020-02-27T23:04:00.9630520Z print t
2020-02-27T23:04:00.9630700Z (long) $19 = 23 
2020-02-27T23:04:00.9630700Z (long) $19 = 23 
2020-02-27T23:04:00.9630880Z continue
2020-02-27T23:04:00.9631050Z print u
2020-02-27T23:04:00.9631240Z (short) $20 = 25 
2020-02-27T23:04:00.9631600Z (int) $21 = 26 
2020-02-27T23:04:00.9631780Z print w
2020-02-27T23:04:00.9631960Z (long) $22 = 27 
2020-02-27T23:04:00.9632140Z print x
---
2020-02-27T23:04:00.9633860Z (int) $27 = 32 
2020-02-27T23:04:00.9634050Z print ue
2020-02-27T23:04:00.9634250Z (unsigned short) $28 = 33 
2020-02-27T23:04:00.9634460Z continue
2020-02-27T23:04:00.9634620Z print aa
2020-02-27T23:04:00.9634860Z ((isize, isize)) $29 = { 0 = 34 1 = 35 } 
2020-02-27T23:04:00.9635260Z print bb
2020-02-27T23:04:00.9635260Z print bb
2020-02-27T23:04:00.9635490Z ((isize, isize)) $30 = { 0 = 36 1 = 37 } 
2020-02-27T23:04:00.9635880Z print cc
2020-02-27T23:04:00.9636070Z (long) $31 = 38 
2020-02-27T23:04:00.9636250Z continue
2020-02-27T23:04:00.9636420Z print dd
2020-02-27T23:04:00.9636420Z print dd
2020-02-27T23:04:00.9636670Z ((isize, isize, isize)) $32 = { 0 = 40 1 = 41 2 = 42 } 
2020-02-27T23:04:00.9637110Z print *ee
2020-02-27T23:04:00.9637110Z print *ee
2020-02-27T23:04:00.9637370Z ((isize, isize, isize)) $33 = { 0 = 43 1 = 44 2 = 45 } 
2020-02-27T23:04:00.9637800Z print *ff
2020-02-27T23:04:00.9637990Z (long) $34 = 46 
2020-02-27T23:04:00.9638170Z print gg
2020-02-27T23:04:00.9638170Z print gg
2020-02-27T23:04:00.9638410Z ((isize, isize)) $35 = { 0 = 47 1 = 48 } 
2020-02-27T23:04:00.9638810Z print *hh
2020-02-27T23:04:00.9638980Z (int) $36 = 50 
2020-02-27T23:04:00.9639170Z continue
2020-02-27T23:04:00.9639340Z print ii
2020-02-27T23:04:00.9639340Z print ii
2020-02-27T23:04:00.9639520Z (int) $37 = 51 
2020-02-27T23:04:00.9639710Z continue
2020-02-27T23:04:00.9639870Z print *jj
2020-02-27T23:04:00.9640060Z (int) $38 = 52 
2020-02-27T23:04:00.9640230Z continue
2020-02-27T23:04:00.9640400Z print kk
2020-02-27T23:04:00.9640580Z (double) $39 = 53 
2020-02-27T23:04:00.9640950Z (long) $40 = 54 
2020-02-27T23:04:00.9641140Z continue
2020-02-27T23:04:00.9641300Z print mm
2020-02-27T23:04:00.9641300Z print mm
2020-02-27T23:04:00.9641490Z (double) $41 = 55 
2020-02-27T23:04:00.9641680Z print *nn
2020-02-27T23:04:00.9641860Z (long) $42 = 56 
2020-02-27T23:04:00.9642210Z print oo
2020-02-27T23:04:00.9642390Z (long) $43 = 57 
2020-02-27T23:04:00.9642780Z print pp
2020-02-27T23:04:00.9642980Z (long) $44 = 58 
---
2020-02-27T23:04:00.9645490Z 
2020-02-27T23:04:00.9646140Z ------------------------------------------
2020-02-27T23:04:00.9646390Z stderr:
2020-02-27T23:04:00.9646950Z ------------------------------------------
2020-02-27T23:04:00.9647690Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9648400Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9649120Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9649830Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9650560Z ------------------------------------------
2020-02-27T23:04:00.9650790Z 
2020-02-27T23:04:00.9650890Z 
2020-02-27T23:04:00.9651540Z ---- [debuginfo-lldb] debuginfo/destructured-for-loop-variable.rs stdout ----
2020-02-27T23:04:00.9651540Z ---- [debuginfo-lldb] debuginfo/destructured-for-loop-variable.rs stdout ----
2020-02-27T23:04:00.9651950Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9652340Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9652590Z 
2020-02-27T23:04:00.9652950Z error: line not found in debugger output: [...]$22 = Struct { x: 3537, y: 35437.5, z: true }
2020-02-27T23:04:00.9653350Z status: exit code: 0
2020-02-27T23:04:00.9654900Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-for-loop-variable.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-for-loop-variable.lldb/destructured-for-loop-variable.debugger.script"
2020-02-27T23:04:00.9656370Z ------------------------------------------
2020-02-27T23:04:00.9656570Z 
2020-02-27T23:04:00.9656570Z 
2020-02-27T23:04:00.9657600Z Hit breakpoint 1.1: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 455 at destructured-for-loop-variable.rs:197:8, address = 0x0000000100002107, resolved, hit count = 1 
2020-02-27T23:04:00.9658150Z 
2020-02-27T23:04:00.9659170Z Hit breakpoint 2.1: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 929 at destructured-for-loop-variable.rs:204:8, address = 0x00000001000022e1, resolved, hit count = 1 
2020-02-27T23:04:00.9659730Z 
2020-02-27T23:04:00.9660740Z Hit breakpoint 3.1: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 1206 at destructured-for-loop-variable.rs:225:8, address = 0x00000001000023f6, resolved, hit count = 1 
2020-02-27T23:04:00.9661300Z 
2020-02-27T23:04:00.9662300Z Hit breakpoint 4.1: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 1431 at destructured-for-loop-variable.rs:229:8, address = 0x00000001000024d7, resolved, hit count = 1 
2020-02-27T23:04:00.9662870Z 
2020-02-27T23:04:00.9663870Z Hit breakpoint 5.1: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 1692 at destructured-for-loop-variable.rs:238:6, address = 0x00000001000025dc, resolved, hit count = 1 
2020-02-27T23:04:00.9664440Z 
2020-02-27T23:04:00.9665430Z Hit breakpoint 6.1: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 1961 at destructured-for-loop-variable.rs:242:6, address = 0x00000001000026e9, resolved, hit count = 1 
2020-02-27T23:04:00.9666620Z LLDB batch-mode script
2020-02-27T23:04:00.9667140Z ----------------------
2020-02-27T23:04:00.9668320Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-for-loop-variable.lldb/destructured-for-loop-variable.debugger.script'.
2020-02-27T23:04:00.9669610Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-for-loop-variable.lldb/a'.
2020-02-27T23:04:00.9670510Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9671490Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-for-loop-variable.lldb/a'
2020-02-27T23:04:00.9672280Z settings set auto-confirm true
2020-02-27T23:04:00.9672610Z version
2020-02-27T23:04:00.9672610Z version
2020-02-27T23:04:00.9673370Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9673900Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9674710Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9675490Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9675850Z error: empty summary strings not allowed
2020-02-27T23:04:00.9676240Z type category enable Rust
2020-02-27T23:04:00.9676400Z 
2020-02-27T23:04:00.9676400Z 
2020-02-27T23:04:00.9677020Z breakpoint set --file 'destructured-for-loop-variable.rs' --line 197
2020-02-27T23:04:00.9678090Z Breakpoint 1: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 455 at destructured-for-loop-variable.rs:197:8, address = 0x0000000100002107 
2020-02-27T23:04:00.9679070Z breakpoint set --file 'destructured-for-loop-variable.rs' --line 204
2020-02-27T23:04:00.9680140Z Breakpoint 2: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 929 at destructured-for-loop-variable.rs:204:8, address = 0x00000001000022e1 
2020-02-27T23:04:00.9681120Z breakpoint set --file 'destructured-for-loop-variable.rs' --line 225
2020-02-27T23:04:00.9682190Z Breakpoint 3: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 1206 at destructured-for-loop-variable.rs:225:8, address = 0x00000001000023f6 
2020-02-27T23:04:00.9683180Z breakpoint set --file 'destructured-for-loop-variable.rs' --line 229
2020-02-27T23:04:00.9684230Z Breakpoint 4: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 1431 at destructured-for-loop-variable.rs:229:8, address = 0x00000001000024d7 
2020-02-27T23:04:00.9685220Z breakpoint set --file 'destructured-for-loop-variable.rs' --line 238
2020-02-27T23:04:00.9686270Z Breakpoint 5: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 1692 at destructured-for-loop-variable.rs:238:6, address = 0x00000001000025dc 
2020-02-27T23:04:00.9687240Z breakpoint set --file 'destructured-for-loop-variable.rs' --line 242
2020-02-27T23:04:00.9688310Z Breakpoint 6: where = a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 + 1961 at destructured-for-loop-variable.rs:242:6, address = 0x00000001000026e9 
2020-02-27T23:04:00.9689180Z type format add --format hex char
2020-02-27T23:04:00.9689810Z type format add --format hex 'unsigned char'
2020-02-27T23:04:00.9690050Z run
2020-02-27T23:04:00.9692130Z Process 64216 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100002107 a`destructured_for_loop_variable::main::h30d3502ccf5a2a36 at destructured-for-loop-variable.rs:197:8 194 }; 195 196 for &Struct { x, y, z } in &[s] { -> 197 zzz(); // #break ^ 198 } 199 200 let tuple: (i8, u8, i16, u16, i32, u32, i64, u64, f32, f64) = Target 0: (a) stopped. Process 64216 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-for-loop-variable.lldb/a' (x86_64) 
2020-02-27T23:04:00.9693490Z print x
2020-02-27T23:04:00.9693680Z (short) $0 = 400 
2020-02-27T23:04:00.9694090Z print y
2020-02-27T23:04:00.9694290Z (float) $1 = 401.5 
2020-02-27T23:04:00.9694480Z print z
2020-02-27T23:04:00.9694660Z (bool) $2 = true 
2020-02-27T23:04:00.9695020Z print _i8
2020-02-27T23:04:00.9695020Z print _i8
2020-02-27T23:04:00.9695350Z (char) $3 = 0x6f 
2020-02-27T23:04:00.9695590Z print _u8
2020-02-27T23:04:00.9695790Z (unsigned char) $4 = 0x70 
2020-02-27T23:04:00.9696010Z print _i16
2020-02-27T23:04:00.9696550Z (short) $5 = -113 
2020-02-27T23:04:00.9696770Z print _u16
2020-02-27T23:04:00.9696980Z (unsigned short) $6 = 114 
2020-02-27T23:04:00.9697190Z print _i32
2020-02-27T23:04:00.9697690Z (int) $7 = -115 
2020-02-27T23:04:00.9697890Z print _u32
2020-02-27T23:04:00.9698100Z (unsigned int) $8 = 116 
2020-02-27T23:04:00.9698300Z print _i64
2020-02-27T23:04:00.9698790Z (long) $9 = -117 
2020-02-27T23:04:00.9698990Z print _u64
2020-02-27T23:04:00.9699210Z (unsigned long) $10 = 118 
2020-02-27T23:04:00.9699410Z print _f32
2020-02-27T23:04:00.9699610Z (float) $11 = 119.5 
2020-02-27T23:04:00.9699810Z print _f64
2020-02-27T23:04:00.9700010Z (double) $12 = 120.5 
2020-02-27T23:04:00.9700380Z print v1
2020-02-27T23:04:00.9700570Z (int) $13 = 80000 
2020-02-27T23:04:00.9700750Z print x1
2020-02-27T23:04:00.9700750Z print x1
2020-02-27T23:04:00.9700950Z (short) $14 = 8000 
2020-02-27T23:04:00.9701140Z print *y1
2020-02-27T23:04:00.9701340Z (float) $15 = 80001.5 
2020-02-27T23:04:00.9701530Z print z1
2020-02-27T23:04:00.9701730Z (bool) $16 = false 
2020-02-27T23:04:00.9701910Z print *x2
2020-02-27T23:04:00.9702430Z (short) $17 = -30000 
2020-02-27T23:04:00.9702640Z print y2
2020-02-27T23:04:00.9703150Z (float) $18 = -300001.5 
2020-02-27T23:04:00.9703470Z print *z2
2020-02-27T23:04:00.9703660Z (bool) $19 = true 
2020-02-27T23:04:00.9703850Z print v2
2020-02-27T23:04:00.9704040Z (double) $20 = 854237.5 
2020-02-27T23:04:00.9704400Z print i
2020-02-27T23:04:00.9704590Z (int) $21 = 1234 
2020-02-27T23:04:00.9704770Z continue
2020-02-27T23:04:00.9704970Z print simple_struct_ident
2020-02-27T23:04:00.9704970Z print simple_struct_ident
2020-02-27T23:04:00.9705330Z (destructured_for_loop_variable::Struct) $22 = { x = 3537 y = 35437.5 z = true } 
2020-02-27T23:04:00.9705880Z print simple_tuple_ident
2020-02-27T23:04:00.9705880Z print simple_tuple_ident
2020-02-27T23:04:00.9706150Z ((u32, i64)) $23 = { 0 = 34903493 1 = 232323 } 
2020-02-27T23:04:00.9706560Z quit
2020-02-27T23:04:00.9706710Z None
2020-02-27T23:04:00.9706830Z 
2020-02-27T23:04:00.9707410Z ------------------------------------------
2020-02-27T23:04:00.9707410Z ------------------------------------------
2020-02-27T23:04:00.9707670Z stderr:
2020-02-27T23:04:00.9708210Z ------------------------------------------
2020-02-27T23:04:00.9708950Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9709640Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9710540Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9711270Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9712060Z ------------------------------------------
2020-02-27T23:04:00.9712260Z 
2020-02-27T23:04:00.9712390Z 
2020-02-27T23:04:00.9712990Z ---- [debuginfo-lldb] debuginfo/destructured-local.rs stdout ----
2020-02-27T23:04:00.9712990Z ---- [debuginfo-lldb] debuginfo/destructured-local.rs stdout ----
2020-02-27T23:04:00.9713400Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9713770Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9714030Z 
2020-02-27T23:04:00.9714300Z error: line not found in debugger output: [...]$6 = (6, 7)
2020-02-27T23:04:00.9714620Z status: exit code: 0
2020-02-27T23:04:00.9716050Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-local.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-local.lldb/destructured-local.debugger.script"
2020-02-27T23:04:00.9717890Z ------------------------------------------
2020-02-27T23:04:00.9718120Z 
2020-02-27T23:04:00.9718120Z 
2020-02-27T23:04:00.9719180Z Hit breakpoint 1.1: where = a`destructured_local::main::h7f2f7e5e782f0f53 + 1226 at destructured-local.rs:371:4, address = 0x00000001000013aa, resolved, hit count = 1 
2020-02-27T23:04:00.9720060Z LLDB batch-mode script
2020-02-27T23:04:00.9720610Z ----------------------
2020-02-27T23:04:00.9735390Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-local.lldb/destructured-local.debugger.script'.
2020-02-27T23:04:00.9736660Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-local.lldb/a'.
2020-02-27T23:04:00.9737580Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9738590Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-local.lldb/a'
2020-02-27T23:04:00.9739370Z settings set auto-confirm true
2020-02-27T23:04:00.9739720Z version
2020-02-27T23:04:00.9739720Z version
2020-02-27T23:04:00.9740420Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9740910Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9741710Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9742480Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9742840Z error: empty summary strings not allowed
2020-02-27T23:04:00.9743230Z type category enable Rust
2020-02-27T23:04:00.9743470Z 
2020-02-27T23:04:00.9743470Z 
2020-02-27T23:04:00.9744100Z breakpoint set --file 'destructured-local.rs' --line 371
2020-02-27T23:04:00.9745140Z Breakpoint 1: where = a`destructured_local::main::h7f2f7e5e782f0f53 + 1226 at destructured-local.rs:371:4, address = 0x00000001000013aa 
2020-02-27T23:04:00.9745640Z run
2020-02-27T23:04:00.9747790Z Process 64239 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000013aa a`destructured_local::main::h7f2f7e5e782f0f53 at destructured-local.rs:371:4 368 // tuple struct with ref binding 369 let &TupleStruct(mm, ref nn) = &TupleStruct(55.0, 56); 370 -> 371 zzz(); // #break ^ 372 } 373 374 fn zzz() { () } Target 0: (a) stopped. Process 64239 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/destructured-local.lldb/a' (x86_64) 
2020-02-27T23:04:00.9749400Z (long) $0 = 1 
2020-02-27T23:04:00.9749590Z print b
2020-02-27T23:04:00.9749590Z print b
2020-02-27T23:04:00.9749770Z (bool) $1 = false 
2020-02-27T23:04:00.9750130Z (long) $2 = 2 
2020-02-27T23:04:00.9750320Z print d
2020-02-27T23:04:00.9750510Z (unsigned short) $3 = 3 
2020-02-27T23:04:00.9750710Z print e
2020-02-27T23:04:00.9750710Z print e
2020-02-27T23:04:00.9750910Z (unsigned short) $4 = 4 
2020-02-27T23:04:00.9751110Z print f
2020-02-27T23:04:00.9751290Z (long) $5 = 5 
2020-02-27T23:04:00.9751470Z print g
2020-02-27T23:04:00.9751690Z ((u32, u32)) $6 = { 0 = 6 1 = 7 } 
2020-02-27T23:04:00.9751900Z print h
2020-02-27T23:04:00.9752080Z (short) $7 = 8 
2020-02-27T23:04:00.9752250Z print i
2020-02-27T23:04:00.9752520Z (destructured_local::Struct) $8 = { a = 9 b = 10 } 
2020-02-27T23:04:00.9752780Z print j
2020-02-27T23:04:00.9752960Z (short) $9 = 11 
2020-02-27T23:04:00.9753150Z print k
2020-02-27T23:04:00.9753320Z (long) $10 = 12 
2020-02-27T23:04:00.9753680Z (int) $11 = 13 
2020-02-27T23:04:00.9753860Z print m
2020-02-27T23:04:00.9754030Z (int) $12 = 14 
2020-02-27T23:04:00.9754220Z print n
---
2020-02-27T23:04:00.9755400Z (long) $15 = 19 
2020-02-27T23:04:00.9755590Z print q
2020-02-27T23:04:00.9755760Z (int) $16 = 20 
2020-02-27T23:04:00.9755940Z print r
2020-02-27T23:04:00.9756370Z (destructured_local::Struct) $17 = { a = 21 b = 22 } 
2020-02-27T23:04:00.9756850Z (int) $18 = 24 
2020-02-27T23:04:00.9757030Z print t
2020-02-27T23:04:00.9757210Z (long) $19 = 23 
2020-02-27T23:04:00.9757390Z print u
---
2020-02-27T23:04:00.9758990Z (long) $24 = 29 
2020-02-27T23:04:00.9759180Z print z
2020-02-27T23:04:00.9759350Z (int) $25 = 30 
2020-02-27T23:04:00.9759530Z print ae
2020-02-27T23:04:00.9759720Z (long) $26 = 31 
2020-02-27T23:04:00.9759910Z print oe
2020-02-27T23:04:00.9760080Z (int) $27 = 32 
2020-02-27T23:04:00.9760270Z print ue
2020-02-27T23:04:00.9760440Z (int) $28 = 33 
2020-02-27T23:04:00.9760630Z print aa
2020-02-27T23:04:00.9760840Z ((i32, i32)) $29 = { 0 = 34 1 = 35 } 
2020-02-27T23:04:00.9761080Z print bb
2020-02-27T23:04:00.9761290Z ((i32, i32)) $30 = { 0 = 36 1 = 37 } 
2020-02-27T23:04:00.9761530Z print cc
2020-02-27T23:04:00.9761710Z (int) $31 = 38 
2020-02-27T23:04:00.9761880Z print dd
2020-02-27T23:04:00.9762130Z ((i32, i32, i32)) $32 = { 0 = 40 1 = 41 2 = 42 } 
2020-02-27T23:04:00.9762380Z print *ee
2020-02-27T23:04:00.9762640Z ((i32, i32, i32)) $33 = { 0 = 43 1 = 44 2 = 45 } 
2020-02-27T23:04:00.9762890Z print *ff
2020-02-27T23:04:00.9763210Z (int) $34 = 46 
2020-02-27T23:04:00.9763400Z print gg
2020-02-27T23:04:00.9763630Z ((i32, i32)) $35 = { 0 = 47 1 = 48 } 
2020-02-27T23:04:00.9763940Z print *hh
2020-02-27T23:04:00.9764140Z (int) $36 = 50 
2020-02-27T23:04:00.9764560Z (int) $37 = 51 
2020-02-27T23:04:00.9764760Z print *jj
2020-02-27T23:04:00.9764940Z (int) $38 = 52 
2020-02-27T23:04:00.9765130Z print kk
2020-02-27T23:04:00.9765130Z print kk
2020-02-27T23:04:00.9765320Z (double) $39 = 53 
2020-02-27T23:04:00.9765700Z (long) $40 = 54 
2020-02-27T23:04:00.9765900Z print mm
2020-02-27T23:04:00.9765900Z print mm
2020-02-27T23:04:00.9766080Z (double) $41 = 55 
2020-02-27T23:04:00.9766280Z print *nn
2020-02-27T23:04:00.9766470Z (long) $42 = 56 
2020-02-27T23:04:00.9766800Z None
2020-02-27T23:04:00.9766930Z 
2020-02-27T23:04:00.9767580Z ------------------------------------------
2020-02-27T23:04:00.9767860Z stderr:
2020-02-27T23:04:00.9767860Z stderr:
2020-02-27T23:04:00.9768420Z ------------------------------------------
2020-02-27T23:04:00.9769180Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9769920Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9770640Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9771380Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9772150Z ------------------------------------------
2020-02-27T23:04:00.9772350Z 
2020-02-27T23:04:00.9772460Z 
2020-02-27T23:04:00.9773190Z ---- [debuginfo-lldb] debuginfo/empty-string.rs stdout ----
2020-02-27T23:04:00.9773190Z ---- [debuginfo-lldb] debuginfo/empty-string.rs stdout ----
2020-02-27T23:04:00.9773650Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9774060Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9774310Z 
2020-02-27T23:04:00.9774600Z error: line not found in debugger output: [...]empty_string = ""
2020-02-27T23:04:00.9774940Z status: exit code: 0
2020-02-27T23:04:00.9776570Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/empty-string.debugger.script"
2020-02-27T23:04:00.9778210Z ------------------------------------------
2020-02-27T23:04:00.9778440Z 
2020-02-27T23:04:00.9778440Z 
2020-02-27T23:04:00.9779430Z Hit breakpoint 1.1: where = a`empty_string::main::h16dc11aef3d9e163 + 49 at empty-string.rs:33:4, address = 0x0000000100001601, resolved, hit count = 1 
2020-02-27T23:04:00.9780310Z LLDB batch-mode script
2020-02-27T23:04:00.9780870Z ----------------------
2020-02-27T23:04:00.9781790Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/empty-string.debugger.script'.
2020-02-27T23:04:00.9782920Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a'.
2020-02-27T23:04:00.9783830Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9784770Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a'
2020-02-27T23:04:00.9785980Z settings set auto-confirm true
2020-02-27T23:04:00.9786610Z version
2020-02-27T23:04:00.9786610Z version
2020-02-27T23:04:00.9788380Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9789350Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9790860Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9792050Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9792970Z error: empty summary strings not allowed
2020-02-27T23:04:00.9793700Z type category enable Rust
2020-02-27T23:04:00.9794290Z 
2020-02-27T23:04:00.9794290Z 
2020-02-27T23:04:00.9795460Z breakpoint set --file 'empty-string.rs' --line 33
2020-02-27T23:04:00.9797180Z Breakpoint 1: where = a`empty_string::main::h16dc11aef3d9e163 + 49 at empty-string.rs:33:4, address = 0x0000000100001601 
2020-02-27T23:04:00.9797880Z run
2020-02-27T23:04:00.9800920Z Process 64262 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001601 a`empty_string::main::h16dc11aef3d9e163 at empty-string.rs:33:4 30 31 let empty_str = ""; 32 -> 33 zzz(); // #break ^ 34 } 35 36 fn zzz() {} Target 0: (a) stopped. Process 64262 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/empty-string.lldb/a' (x86_64) 
2020-02-27T23:04:00.9803090Z fr v empty_string
2020-02-27T23:04:00.9803690Z (alloc::string::String) empty_string = { vec = {} } 
2020-02-27T23:04:00.9804410Z fr v empty_str
2020-02-27T23:04:00.9804930Z (&str) empty_str = { data_ptr = 0x0000000100001700 "" length = 0 } 
2020-02-27T23:04:00.9805770Z None
2020-02-27T23:04:00.9805970Z 
2020-02-27T23:04:00.9807130Z ------------------------------------------
2020-02-27T23:04:00.9807560Z stderr:
2020-02-27T23:04:00.9807560Z stderr:
2020-02-27T23:04:00.9808700Z ------------------------------------------
2020-02-27T23:04:00.9809820Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9811260Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9812000Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9812920Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9813900Z ------------------------------------------
2020-02-27T23:04:00.9814590Z 
2020-02-27T23:04:00.9814730Z 
2020-02-27T23:04:00.9815420Z ---- [debuginfo-lldb] debuginfo/enum-thinlto.rs stdout ----
2020-02-27T23:04:00.9815420Z ---- [debuginfo-lldb] debuginfo/enum-thinlto.rs stdout ----
2020-02-27T23:04:00.9815820Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9816410Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9816720Z 
2020-02-27T23:04:00.9817040Z error: line not found in debugger output: (enum_thinlto::ABC) $0 = ABC { }
2020-02-27T23:04:00.9817410Z status: exit code: 0
2020-02-27T23:04:00.9818860Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/enum-thinlto.debugger.script"
2020-02-27T23:04:00.9820290Z ------------------------------------------
2020-02-27T23:04:00.9820500Z 
2020-02-27T23:04:00.9820500Z 
2020-02-27T23:04:00.9821430Z Hit breakpoint 1.1: where = a`enum_thinlto::f::h427be9c8ef855fd8 + 12 at enum-thinlto.rs:43:4, address = 0x000000010000162c, resolved, hit count = 1 
2020-02-27T23:04:00.9822340Z LLDB batch-mode script
2020-02-27T23:04:00.9822910Z ----------------------
2020-02-27T23:04:00.9823830Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/enum-thinlto.debugger.script'.
2020-02-27T23:04:00.9825150Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a'.
2020-02-27T23:04:00.9826130Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9827170Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a'
2020-02-27T23:04:00.9827930Z settings set auto-confirm true
2020-02-27T23:04:00.9828270Z version
2020-02-27T23:04:00.9828270Z version
2020-02-27T23:04:00.9828950Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9829460Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9830270Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9831040Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9831420Z error: empty summary strings not allowed
2020-02-27T23:04:00.9831800Z type category enable Rust
2020-02-27T23:04:00.9831960Z 
2020-02-27T23:04:00.9832520Z breakpoint set --file 'enum-thinlto.rs' --line 41
2020-02-27T23:04:00.9832520Z breakpoint set --file 'enum-thinlto.rs' --line 41
2020-02-27T23:04:00.9833420Z Breakpoint 1: where = a`enum_thinlto::f::h427be9c8ef855fd8 + 12 at enum-thinlto.rs:43:4, address = 0x000000010000162c 
2020-02-27T23:04:00.9833850Z run
2020-02-27T23:04:00.9835680Z Process 64284 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x000000010000162c a`enum_thinlto::f::h427be9c8ef855fd8(abc=0x00007ffeefbfd398) at enum-thinlto.rs:43:4 40 fn f(abc: &ABC) { 41 zzz(); // #break 42 -> 43 println!("{:?}", abc); ^ 44 } 45 46 fn zzz() {()} Target 0: (a) stopped. Process 64284 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/enum-thinlto.lldb/a' (x86_64) 
2020-02-27T23:04:00.9836880Z print *abc
2020-02-27T23:04:00.9837100Z (enum_thinlto::ABC) $0 = 
2020-02-27T23:04:00.9837460Z None
2020-02-27T23:04:00.9837570Z 
2020-02-27T23:04:00.9838120Z ------------------------------------------
2020-02-27T23:04:00.9838370Z stderr:
2020-02-27T23:04:00.9838370Z stderr:
2020-02-27T23:04:00.9838910Z ------------------------------------------
2020-02-27T23:04:00.9839640Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9840340Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9841380Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9842240Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9843190Z ------------------------------------------
2020-02-27T23:04:00.9843680Z 
2020-02-27T23:04:00.9843820Z 
2020-02-27T23:04:00.9844490Z ---- [debuginfo-lldb] debuginfo/evec-in-struct.rs stdout ----
2020-02-27T23:04:00.9844490Z ---- [debuginfo-lldb] debuginfo/evec-in-struct.rs stdout ----
2020-02-27T23:04:00.9844940Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9845540Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9845940Z 
2020-02-27T23:04:00.9847170Z error: line not found in debugger output: [...]$0 = NoPadding1 { x: [0, 1, 2], y: -3, z: [4.5, 5.5] }
2020-02-27T23:04:00.9847830Z status: exit code: 0
2020-02-27T23:04:00.9849950Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/evec-in-struct.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/evec-in-struct.lldb/evec-in-struct.debugger.script"
2020-02-27T23:04:00.9852040Z ------------------------------------------
2020-02-27T23:04:00.9852350Z 
2020-02-27T23:04:00.9852350Z 
2020-02-27T23:04:00.9853870Z Hit breakpoint 1.1: where = a`evec_in_struct::main::hf400793f45cd7d77 + 412 at evec-in-struct.rs:110:4, address = 0x0000000100000c4c, resolved, hit count = 1 
2020-02-27T23:04:00.9855120Z LLDB batch-mode script
2020-02-27T23:04:00.9855910Z ----------------------
2020-02-27T23:04:00.9857400Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/evec-in-struct.lldb/evec-in-struct.debugger.script'.
2020-02-27T23:04:00.9859010Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/evec-in-struct.lldb/a'.
2020-02-27T23:04:00.9860310Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9861670Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/evec-in-struct.lldb/a'
2020-02-27T23:04:00.9862980Z settings set auto-confirm true
2020-02-27T23:04:00.9863500Z version
2020-02-27T23:04:00.9863500Z version
2020-02-27T23:04:00.9864530Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9865300Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9866470Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9867580Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9868140Z error: empty summary strings not allowed
2020-02-27T23:04:00.9868720Z type category enable Rust
2020-02-27T23:04:00.9868970Z 
2020-02-27T23:04:00.9868970Z 
2020-02-27T23:04:00.9869820Z breakpoint set --file 'evec-in-struct.rs' --line 110
2020-02-27T23:04:00.9871210Z Breakpoint 1: where = a`evec_in_struct::main::hf400793f45cd7d77 + 412 at evec-in-struct.rs:110:4, address = 0x0000000100000c4c 
2020-02-27T23:04:00.9871900Z run
2020-02-27T23:04:00.9875080Z Process 64307 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000c4c a`evec_in_struct::main::hf400793f45cd7d77 at evec-in-struct.rs:110:4 107 y: [24, 25] 108 }; 109 -> 110 zzz(); // #break ^ 111 } 112 113 fn zzz() { () } Target 0: (a) stopped. Process 64307 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/evec-in-struct.lldb/a' (x86_64) 
2020-02-27T23:04:00.9876250Z print no_padding1
2020-02-27T23:04:00.9877120Z (evec_in_struct::NoPadding1) $0 = { x = { [0] = 0 [1] = 1 [2] = 2 } y = -3 z = { [0] = 4.5 [1] = 5.5 } } 
2020-02-27T23:04:00.9877520Z print no_padding2
2020-02-27T23:04:00.9878230Z (evec_in_struct::NoPadding2) $1 = { x = { [0] = 6 [1] = 7 [2] = 8 } y = { [0] = { [0] = 9 [1] = 10 } [1] = { [0] = 11 [1] = 12 } } } 
2020-02-27T23:04:00.9878700Z print struct_internal_padding
2020-02-27T23:04:00.9879250Z (evec_in_struct::StructInternalPadding) $2 = { x = { [0] = 13 [1] = 14 } y = { [0] = 15 [1] = 16 } } 
2020-02-27T23:04:00.9879670Z print single_vec
2020-02-27T23:04:00.9880030Z (evec_in_struct::SingleVec) $3 = { x = { [0] = 17 [1] = 18 [2] = 19 [3] = 20 [4] = 21 } } 
2020-02-27T23:04:00.9880410Z print struct_padded_at_end
2020-02-27T23:04:00.9880800Z (evec_in_struct::StructPaddedAtEnd) $4 = { x = { [0] = 22 [1] = 23 } y = { [0] = 24 [1] = 25 } } 
2020-02-27T23:04:00.9881300Z None
2020-02-27T23:04:00.9881420Z 
2020-02-27T23:04:00.9882220Z ------------------------------------------
2020-02-27T23:04:00.9882590Z stderr:
2020-02-27T23:04:00.9882590Z stderr:
2020-02-27T23:04:00.9883530Z ------------------------------------------
2020-02-27T23:04:00.9884630Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9885710Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9886930Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9887650Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9888640Z ------------------------------------------
2020-02-27T23:04:00.9888870Z 
2020-02-27T23:04:00.9888980Z 
2020-02-27T23:04:00.9889640Z ---- [debuginfo-lldb] debuginfo/generator-objects.rs stdout ----
2020-02-27T23:04:00.9889640Z ---- [debuginfo-lldb] debuginfo/generator-objects.rs stdout ----
2020-02-27T23:04:00.9890040Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9890430Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9890690Z 
2020-02-27T23:04:00.9891430Z error: line not found in debugger output: (generator_objects::main::generator-0) $0 = generator-0(&0x[...])
2020-02-27T23:04:00.9891880Z status: exit code: 0
2020-02-27T23:04:00.9893440Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generator-objects.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generator-objects.lldb/generator-objects.debugger.script"
2020-02-27T23:04:00.9894870Z ------------------------------------------
2020-02-27T23:04:00.9895080Z 
2020-02-27T23:04:00.9895080Z 
2020-02-27T23:04:00.9896170Z Hit breakpoint 1.1: where = a`generator_objects::main::hf30bd576c373d451 + 33 at generator-objects.rs:59:4, address = 0x00000001000015d1, resolved, hit count = 1 
2020-02-27T23:04:00.9896780Z 
2020-02-27T23:04:00.9898170Z Hit breakpoint 2.1: where = a`generator_objects::main::hf30bd576c373d451 + 99 at generator-objects.rs:61:4, address = 0x0000000100001613, resolved, hit count = 1 
2020-02-27T23:04:00.9898950Z 
2020-02-27T23:04:00.9900350Z Hit breakpoint 3.1: where = a`generator_objects::main::hf30bd576c373d451 + 147 at generator-objects.rs:63:4, address = 0x0000000100001643, resolved, hit count = 1 
2020-02-27T23:04:00.9901070Z 
2020-02-27T23:04:00.9902530Z Hit breakpoint 4.1: where = a`generator_objects::main::hf30bd576c373d451 + 195 at generator-objects.rs:65:4, address = 0x0000000100001673, resolved, hit count = 1 
2020-02-27T23:04:00.9903990Z LLDB batch-mode script
2020-02-27T23:04:00.9904560Z ----------------------
2020-02-27T23:04:00.9906920Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generator-objects.lldb/generator-objects.debugger.script'.
2020-02-27T23:04:00.9908790Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generator-objects.lldb/a'.
2020-02-27T23:04:00.9910120Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9911050Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generator-objects.lldb/a'
2020-02-27T23:04:00.9911980Z settings set auto-confirm true
2020-02-27T23:04:00.9912360Z version
2020-02-27T23:04:00.9912360Z version
2020-02-27T23:04:00.9913100Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9913590Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9914540Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9915310Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9915700Z error: empty summary strings not allowed
2020-02-27T23:04:00.9916100Z type category enable Rust
2020-02-27T23:04:00.9916260Z 
2020-02-27T23:04:00.9916260Z 
2020-02-27T23:04:00.9916970Z breakpoint set --file 'generator-objects.rs' --line 59
2020-02-27T23:04:00.9917940Z Breakpoint 1: where = a`generator_objects::main::hf30bd576c373d451 + 33 at generator-objects.rs:59:4, address = 0x00000001000015d1 
2020-02-27T23:04:00.9918840Z breakpoint set --file 'generator-objects.rs' --line 61
2020-02-27T23:04:00.9919790Z Breakpoint 2: where = a`generator_objects::main::hf30bd576c373d451 + 99 at generator-objects.rs:61:4, address = 0x0000000100001613 
2020-02-27T23:04:00.9920650Z breakpoint set --file 'generator-objects.rs' --line 63
2020-02-27T23:04:00.9921590Z Breakpoint 3: where = a`generator_objects::main::hf30bd576c373d451 + 147 at generator-objects.rs:63:4, address = 0x0000000100001643 
2020-02-27T23:04:00.9922460Z breakpoint set --file 'generator-objects.rs' --line 65
2020-02-27T23:04:00.9923410Z Breakpoint 4: where = a`generator_objects::main::hf30bd576c373d451 + 195 at generator-objects.rs:65:4, address = 0x0000000100001673 
2020-02-27T23:04:00.9923860Z run
2020-02-27T23:04:00.9925830Z Process 64437 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000015d1 a`generator_objects::main::hf30bd576c373d451 at generator-objects.rs:59:4 56 yield; 57 println!("{} {} {}", a, c, d); 58 }; -> 59 _zzz(); // #break ^ 60 Pin::new(&mut b).resume(()); 61 _zzz(); // #break 62 Pin::new(&mut b).resume(()); Target 0: (a) stopped. Process 64437 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generator-objects.lldb/a' (x86_64) 
2020-02-27T23:04:00.9927120Z print b
2020-02-27T23:04:00.9927780Z (generator_objects::main::generator-0) $0 = { 0 = 0x00007ffeefbfd2dc } 
2020-02-27T23:04:00.9928270Z print b
2020-02-27T23:04:00.9928270Z print b
2020-02-27T23:04:00.9928910Z (generator_objects::main::generator-0) $1 = { 0 = 0x00007ffeefbfd2dc } 
2020-02-27T23:04:00.9929390Z print b
2020-02-27T23:04:00.9929390Z print b
2020-02-27T23:04:00.9930030Z (generator_objects::main::generator-0) $2 = { 0 = 0x00007ffeefbfd2dc } 
2020-02-27T23:04:00.9930520Z print b
2020-02-27T23:04:00.9930520Z print b
2020-02-27T23:04:00.9931170Z (generator_objects::main::generator-0) $3 = { 0 = 0x00007ffeefbfd2dc } 
2020-02-27T23:04:00.9931620Z None
2020-02-27T23:04:00.9931740Z 
2020-02-27T23:04:00.9932270Z ------------------------------------------
2020-02-27T23:04:00.9932530Z stderr:
2020-02-27T23:04:00.9932530Z stderr:
2020-02-27T23:04:00.9933080Z ------------------------------------------
2020-02-27T23:04:00.9933870Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9934580Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9935290Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9936200Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9936990Z ------------------------------------------
2020-02-27T23:04:00.9937190Z 
2020-02-27T23:04:00.9937310Z 
2020-02-27T23:04:00.9938060Z ---- [debuginfo-lldb] debuginfo/generic-function.rs stdout ----
2020-02-27T23:04:00.9938060Z ---- [debuginfo-lldb] debuginfo/generic-function.rs stdout ----
2020-02-27T23:04:00.9938490Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9938880Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9939130Z 
2020-02-27T23:04:00.9939420Z error: line not found in debugger output: [...]$2 = ((1, 2.5), (2.5, 1))
2020-02-27T23:04:00.9939770Z status: exit code: 0
2020-02-27T23:04:00.9941210Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generic-function.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generic-function.lldb/generic-function.debugger.script"
2020-02-27T23:04:00.9942610Z ------------------------------------------
2020-02-27T23:04:00.9942820Z 
2020-02-27T23:04:00.9942820Z 
2020-02-27T23:04:00.9943970Z Hit breakpoint 1.2: where = a`generic_function::dup_tup::hb026b15343de9ebe + 200 at generic-function.rs:88:4, address = 0x0000000100001648, resolved, hit count = 1 
2020-02-27T23:04:00.9944470Z 
2020-02-27T23:04:00.9945450Z Hit breakpoint 1.3: where = a`generic_function::dup_tup::hc017fe23e276c316 + 210 at generic-function.rs:88:4, address = 0x0000000100001782, resolved, hit count = 1 
2020-02-27T23:04:00.9945950Z 
2020-02-27T23:04:00.9947010Z Hit breakpoint 1.1: where = a`generic_function::dup_tup::h1769e524729b9e84 + 303 at generic-function.rs:88:4, address = 0x000000010000150f, resolved, hit count = 1 
2020-02-27T23:04:00.9947860Z LLDB batch-mode script
2020-02-27T23:04:00.9948390Z ----------------------
2020-02-27T23:04:00.9949300Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generic-function.lldb/generic-function.debugger.script'.
2020-02-27T23:04:00.9950420Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generic-function.lldb/a'.
2020-02-27T23:04:00.9951290Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9952210Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generic-function.lldb/a'
2020-02-27T23:04:00.9952970Z settings set auto-confirm true
2020-02-27T23:04:00.9953310Z version
2020-02-27T23:04:00.9953310Z version
2020-02-27T23:04:00.9953990Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9954490Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9955270Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9956040Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9956430Z error: empty summary strings not allowed
2020-02-27T23:04:00.9956800Z type category enable Rust
2020-02-27T23:04:00.9956970Z 
2020-02-27T23:04:00.9956970Z 
2020-02-27T23:04:00.9957540Z breakpoint set --file 'generic-function.rs' --line 88
2020-02-27T23:04:00.9957860Z Breakpoint 1: 3 locations. 
2020-02-27T23:04:00.9958060Z run
2020-02-27T23:04:00.9960270Z Process 64460 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2 frame #0: 0x0000000100001648 a`generic_function::dup_tup::hb026b15343de9ebe(t0=0x0000000100001b60, t1=0x0000000100001b68) at generic-function.rs:88:4 85 86 fn dup_tup<T0: Clone, T1: Clone>(t0: &T0, t1: &T1) -> ((T0, T1), (T1, T0)) { 87 let ret = ((t0.clone(), t1.clone()), (t1.clone(), t0.clone())); -> 88 zzz(); // #break ^ 89 ret 90 } 91 Target 0: (a) stopped. Process 64460 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/generic-function.lldb/a' (x86_64) 
2020-02-27T23:04:00.9961940Z print *t0
2020-02-27T23:04:00.9962130Z (int) $0 = 1 
2020-02-27T23:04:00.9962310Z print *t1
2020-02-27T23:04:00.9962500Z (double) $1 = 2.5 
2020-02-27T23:04:00.9962690Z print ret
2020-02-27T23:04:00.9963150Z (((i32, f64), (f64, i32))) $2 = { 0 = { 0 = 1 1 = 2.5 } 1 = { 0 = 2.5 1 = 1 } } 
2020-02-27T23:04:00.9963900Z print *t0
2020-02-27T23:04:00.9963900Z print *t0
2020-02-27T23:04:00.9964190Z (double) $3 = 3.5 
2020-02-27T23:04:00.9964510Z print *t1
2020-02-27T23:04:00.9964820Z (unsigned short) $4 = 4 
2020-02-27T23:04:00.9965130Z print ret
2020-02-27T23:04:00.9965600Z (((f64, u16), (u16, f64))) $5 = { 0 = { 0 = 3.5 1 = 4 } 1 = { 0 = 4 1 = 3.5 } } 
2020-02-27T23:04:00.9966330Z print *t0
2020-02-27T23:04:00.9966600Z (int) $6 = 5 
2020-02-27T23:04:00.9966880Z print *t1
2020-02-27T23:04:00.9966880Z print *t1
2020-02-27T23:04:00.9967270Z (generic_function::Struct) $7 = { a = 6 b = 7.5 } 
2020-02-27T23:04:00.9967670Z print ret
2020-02-27T23:04:00.9968410Z (((i32, generic_function::Struct), (generic_function::Struct, i32))) $8 = { 0 = { 0 = 5 1 = { a = 6 b = 7.5 } } 1 = { 0 = { a = 6 b = 7.5 } 1 = 5 } } 
2020-02-27T23:04:00.9969440Z quit
2020-02-27T23:04:00.9969670Z None
2020-02-27T23:04:00.9969850Z 
2020-02-27T23:04:00.9970710Z ------------------------------------------
2020-02-27T23:04:00.9970710Z ------------------------------------------
2020-02-27T23:04:00.9971100Z stderr:
2020-02-27T23:04:00.9971890Z ------------------------------------------
2020-02-27T23:04:00.9972970Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9974020Z   watchdog_start_time = time.clock()
2020-02-27T23:04:00.9974760Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:00.9975490Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:00.9976320Z ------------------------------------------
2020-02-27T23:04:00.9976530Z 
2020-02-27T23:04:00.9976640Z 
2020-02-27T23:04:00.9977250Z ---- [debuginfo-lldb] debuginfo/issue-22656.rs stdout ----
2020-02-27T23:04:00.9977250Z ---- [debuginfo-lldb] debuginfo/issue-22656.rs stdout ----
2020-02-27T23:04:00.9977630Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:00.9978040Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:00.9978290Z 
2020-02-27T23:04:00.9978590Z error: line not found in debugger output: [...]$0 = vec![1, 2, 3]
2020-02-27T23:04:00.9978920Z status: exit code: 0
2020-02-27T23:04:00.9980330Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/issue-22656.debugger.script"
2020-02-27T23:04:00.9981740Z ------------------------------------------
2020-02-27T23:04:00.9981960Z 
2020-02-27T23:04:00.9981960Z 
2020-02-27T23:04:00.9982870Z Hit breakpoint 1.1: where = a`issue_22656::main::h34c4c6bd35d83e23 + 91 at issue-22656.rs:46:4, address = 0x0000000100001fbb, resolved, hit count = 1 
2020-02-27T23:04:00.9983730Z LLDB batch-mode script
2020-02-27T23:04:00.9984260Z ----------------------
2020-02-27T23:04:00.9985190Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/issue-22656.debugger.script'.
2020-02-27T23:04:00.9986280Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a'.
2020-02-27T23:04:00.9987160Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:00.9988100Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a'
2020-02-27T23:04:00.9988870Z settings set auto-confirm true
2020-02-27T23:04:00.9989420Z version
2020-02-27T23:04:00.9989420Z version
2020-02-27T23:04:00.9990190Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:00.9990880Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:00.9991760Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9992550Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:00.9992920Z error: empty summary strings not allowed
2020-02-27T23:04:00.9993320Z type category enable Rust
2020-02-27T23:04:00.9993480Z 
2020-02-27T23:04:00.9994060Z breakpoint set --file 'issue-22656.rs' --line 46
2020-02-27T23:04:00.9994060Z breakpoint set --file 'issue-22656.rs' --line 46
2020-02-27T23:04:00.9995020Z Breakpoint 1: where = a`issue_22656::main::h34c4c6bd35d83e23 + 91 at issue-22656.rs:46:4, address = 0x0000000100001fbb 
2020-02-27T23:04:00.9995680Z run
2020-02-27T23:04:00.9998330Z Process 64618 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100001fbb a`issue_22656::main::h34c4c6bd35d83e23 at issue-22656.rs:46:4 43 w: 456 44 }; 45 -> 46 zzz(); // #break ^ 47 } 48 49 fn zzz() { () } Target 0: (a) stopped. Process 64618 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-22656.lldb/a' (x86_64) 
2020-02-27T23:04:00.9999930Z print v
2020-02-27T23:04:01.0000360Z (alloc::vec::Vec<int>) $0 = { [0] = 1 [1] = 2 [2] = 3 } 
2020-02-27T23:04:01.0000790Z print zs
2020-02-27T23:04:01.0001260Z (issue_22656::StructWithZeroSizedField) $1 = { x = y = 123 z = w = 456 } 
2020-02-27T23:04:01.0001960Z None
2020-02-27T23:04:01.0002160Z 
2020-02-27T23:04:01.0002970Z ------------------------------------------
2020-02-27T23:04:01.0003460Z stderr:
2020-02-27T23:04:01.0003460Z stderr:
2020-02-27T23:04:01.0004080Z ------------------------------------------
2020-02-27T23:04:01.0004840Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:01.0005570Z   watchdog_start_time = time.clock()
2020-02-27T23:04:01.0006920Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:01.0007680Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:01.0008620Z ------------------------------------------
2020-02-27T23:04:01.0008820Z 
2020-02-27T23:04:01.0008950Z 
2020-02-27T23:04:01.0009560Z ---- [debuginfo-lldb] debuginfo/issue-57822.rs stdout ----
2020-02-27T23:04:01.0009560Z ---- [debuginfo-lldb] debuginfo/issue-57822.rs stdout ----
2020-02-27T23:04:01.0009950Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:01.0010330Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:01.0010590Z 
2020-02-27T23:04:01.0011350Z error: line not found in debugger output: (issue_57822::main::closure-1) $0 = closure-1(closure-0(1))
2020-02-27T23:04:01.0011790Z status: exit code: 0
2020-02-27T23:04:01.0013170Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-57822.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-57822.lldb/issue-57822.debugger.script"
2020-02-27T23:04:01.0014560Z ------------------------------------------
2020-02-27T23:04:01.0014770Z 
2020-02-27T23:04:01.0014770Z 
2020-02-27T23:04:01.0015680Z Hit breakpoint 1.1: where = a`issue_57822::main::ha8683423c0a80017 + 64 at issue-57822.rs:52:4, address = 0x0000000100000bd0, resolved, hit count = 1 
2020-02-27T23:04:01.0016910Z LLDB batch-mode script
2020-02-27T23:04:01.0017990Z ----------------------
2020-02-27T23:04:01.0019100Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-57822.lldb/issue-57822.debugger.script'.
2020-02-27T23:04:01.0020540Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-57822.lldb/a'.
2020-02-27T23:04:01.0021630Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:01.0022650Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-57822.lldb/a'
2020-02-27T23:04:01.0023440Z settings set auto-confirm true
2020-02-27T23:04:01.0023800Z version
2020-02-27T23:04:01.0023800Z version
2020-02-27T23:04:01.0024500Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:01.0025020Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:01.0025850Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:01.0026640Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:01.0027030Z error: empty summary strings not allowed
2020-02-27T23:04:01.0027430Z type category enable Rust
2020-02-27T23:04:01.0027590Z 
2020-02-27T23:04:01.0028180Z breakpoint set --file 'issue-57822.rs' --line 52
2020-02-27T23:04:01.0028180Z breakpoint set --file 'issue-57822.rs' --line 52
2020-02-27T23:04:01.0029130Z Breakpoint 1: where = a`issue_57822::main::ha8683423c0a80017 + 64 at issue-57822.rs:52:4, address = 0x0000000100000bd0 
2020-02-27T23:04:01.0029580Z run
2020-02-27T23:04:01.0031280Z Process 64641 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x0000000100000bd0 a`issue_57822::main::ha8683423c0a80017 at issue-57822.rs:52:4 49 yield; 50 }; 51 -> 52 zzz(); // #break ^ 53 } 54 55 fn zzz() { () } Target 0: (a) stopped. Process 64641 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/issue-57822.lldb/a' (x86_64) 
2020-02-27T23:04:01.0032370Z print g
2020-02-27T23:04:01.0034020Z (issue_57822::main::closure-1) $0 = { 0 = { 0 = 1 } } 
2020-02-27T23:04:01.0034480Z print b
2020-02-27T23:04:01.0035460Z (issue_57822::main::generator-3) $1 = { 0 = { 0 = 2 } } 
2020-02-27T23:04:01.0036140Z None
2020-02-27T23:04:01.0036330Z 
2020-02-27T23:04:01.0037160Z ------------------------------------------
2020-02-27T23:04:01.0037560Z stderr:
2020-02-27T23:04:01.0037560Z stderr:
2020-02-27T23:04:01.0038390Z ------------------------------------------
2020-02-27T23:04:01.0039480Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:142: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:01.0040540Z   watchdog_start_time = time.clock()
2020-02-27T23:04:01.0041610Z /Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py:146: DeprecationWarning: time.clock has been deprecated in Python 3.3 and will be removed from Python 3.8: use time.perf_counter or time.process_time instead
2020-02-27T23:04:01.0042690Z   while time.clock() < watchdog_max_time:
2020-02-27T23:04:01.0044130Z ------------------------------------------
2020-02-27T23:04:01.0044350Z 
2020-02-27T23:04:01.0044480Z 
2020-02-27T23:04:01.0045110Z ---- [debuginfo-lldb] debuginfo/method-on-struct.rs stdout ----
2020-02-27T23:04:01.0045110Z ---- [debuginfo-lldb] debuginfo/method-on-struct.rs stdout ----
2020-02-27T23:04:01.0045530Z NOTE: compiletest thinks it is using LLDB version 1100
2020-02-27T23:04:01.0045920Z NOTE: compiletest thinks it is using LLDB without native rust support
2020-02-27T23:04:01.0046180Z 
2020-02-27T23:04:01.0046490Z error: line not found in debugger output: [...]$0 = Struct { x: 100 }
2020-02-27T23:04:01.0046850Z status: exit code: 0
2020-02-27T23:04:01.0048310Z command: "/usr/bin/python3" "/Users/runner/runners/2.165.0/work/1/s/src/etc/lldb_batchmode.py" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-struct.lldb/a" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-struct.lldb/method-on-struct.debugger.script"
2020-02-27T23:04:01.0050060Z ------------------------------------------
2020-02-27T23:04:01.0050270Z 
2020-02-27T23:04:01.0050270Z 
2020-02-27T23:04:01.0051450Z Hit breakpoint 1.1: where = a`method_on_struct::Struct::self_by_ref::h72efdb4a22987c81 + 32 at method-on-struct.rs:137:8, address = 0x00000001000010f0, resolved, hit count = 1 
2020-02-27T23:04:01.0052020Z 
2020-02-27T23:04:01.0053070Z Hit breakpoint 2.1: where = a`method_on_struct::Struct::self_by_val::h84fd9602a448e317 + 32 at method-on-struct.rs:142:8, address = 0x0000000100001180, resolved, hit count = 1 
2020-02-27T23:04:01.0053590Z 
2020-02-27T23:04:01.0054580Z Hit breakpoint 1.1: where = a`method_on_struct::Struct::self_by_ref::h72efdb4a22987c81 + 32 at method-on-struct.rs:137:8, address = 0x00000001000010f0, resolved, hit count = 2 
2020-02-27T23:04:01.0055110Z 
2020-02-27T23:04:01.0056100Z Hit breakpoint 2.1: where = a`method_on_struct::Struct::self_by_val::h84fd9602a448e317 + 32 at method-on-struct.rs:142:8, address = 0x0000000100001180, resolved, hit count = 2 
2020-02-27T23:04:01.0056640Z 
2020-02-27T23:04:01.0057650Z Hit breakpoint 3.1: where = a`method_on_struct::Struct::self_owned::h83be291dc87e8eea + 28 at method-on-struct.rs:147:8, address = 0x000000010000120c, resolved, hit count = 1 
2020-02-27T23:04:01.0058550Z LLDB batch-mode script
2020-02-27T23:04:01.0059110Z ----------------------
2020-02-27T23:04:01.0060050Z Debugger commands script is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-struct.lldb/method-on-struct.debugger.script'.
2020-02-27T23:04:01.0061190Z Target executable is '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-struct.lldb/a'.
2020-02-27T23:04:01.0062090Z Current working directory is '/Users/runner/runners/2.165.0/work/1/s'
2020-02-27T23:04:01.0063020Z Creating a target for '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-struct.lldb/a'
2020-02-27T23:04:01.0063830Z settings set auto-confirm true
2020-02-27T23:04:01.0064180Z version
2020-02-27T23:04:01.0064180Z version
2020-02-27T23:04:01.0064880Z lldb-1100.0.30.12 Apple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15) 
2020-02-27T23:04:01.0065410Z command script import /Users/runner/runners/2.165.0/work/1/s/./src/etc/lldb_lookup.py
2020-02-27T23:04:01.0066220Z type synthetic add -l lldb_lookup.synthetic_lookup -x '.*' --category Rust
2020-02-27T23:04:01.0067020Z type summary add -l lldb_lookup.summary_lookup -x '.*' --category Rust
2020-02-27T23:04:01.0067410Z error: empty summary strings not allowed
2020-02-27T23:04:01.0067790Z type category enable Rust
2020-02-27T23:04:01.0067970Z 
2020-02-27T23:04:01.0067970Z 
2020-02-27T23:04:01.0068550Z breakpoint set --file 'method-on-struct.rs' --line 137
2020-02-27T23:04:01.0069580Z Breakpoint 1: where = a`method_on_struct::Struct::self_by_ref::h72efdb4a22987c81 + 32 at method-on-struct.rs:137:8, address = 0x00000001000010f0 
2020-02-27T23:04:01.0070510Z breakpoint set --file 'method-on-struct.rs' --line 142
2020-02-27T23:04:01.0071540Z Breakpoint 2: where = a`method_on_struct::Struct::self_by_val::h84fd9602a448e317 + 32 at method-on-struct.rs:142:8, address = 0x0000000100001180 
2020-02-27T23:04:01.0072490Z breakpoint set --file 'method-on-struct.rs' --line 147
2020-02-27T23:04:01.0073620Z Breakpoint 3: where = a`method_on_struct::Struct::self_owned::h83be291dc87e8eea + 28 at method-on-struct.rs:147:8, address = 0x000000010000120c 
2020-02-27T23:04:01.0074130Z run
2020-02-27T23:04:01.0076350Z Process 64928 stopped * thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1 frame #0: 0x00000001000010f0 a`method_on_struct::Struct::self_by_ref::h72efdb4a22987c81(self=0x00007ffeefbfd338, arg1=-1, arg2=-2) at method-on-struct.rs:137:8 134 impl Struct { 135 136 fn self_by_ref(&self, arg1: isize, arg2: isize) -> isize { -> 137 zzz(); // #break ^ 138 self.x + arg1 + arg2 139 } 140 Target 0: (a) stopped. Process 64928 launched: '/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/debuginfo/method-on-struct.lldb/a' (x86_64) 
2020-02-27T23:04:01.0078040Z print *self
2020-02-27T23:04:01.0078310Z (method_on_struct::Struct) $0 = { x = 100 } 
2020-02-27T23:04:01.0078580Z print arg1
2020-02-27T23:04:01.0079300Z (long) $1 = -1 
2020-02-27T23:04:01.0079540Z print arg2
2020-02-27T23:04:01.0080080Z (long) $2 = -2 
2020-02-27T23:04:01.0080460Z print self
2020-02-27T23:04:01.0080460Z print self
2020-02-27T23:04:01.0080720Z (method_on_struct::Struct) $3 = { x = 100 } 
2020-02-27T23:04:01.0080970Z print arg1
2020-02-27T23:04:01.0081480Z (long) $4 = -3 
