
[00:57:45] /checkout/src/test/codegen/alloc-optimisation.rs:18:17: error: CHECK-NEXT: is not on the line after the previous match
[00:57:45]  // CHECK-NEXT: ret void
[00:57:45]                 ^
[00:57:45] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:1198:2: note: 'next' match was here
[00:57:45]  ret void
[00:57:45]  ^
[00:57:45] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:1183:7: note: previous match ended here
[00:57:45] start:
[00:57:45]       ^
[00:57:45] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:1184:1: note: non-matching line after previous match is here
[00:57:45]  %_7 = alloca %"collections::vec::Vec<u8>"
[00:57:45] ^
[00:57:45] /checkout/src/test/codegen/alloc-optimisation.rs:27:17: error: CHECK-NEXT: is not on the line after the previous match
[00:57:45]  // CHECK-NEXT: ret void
[00:57:45]                 ^
[00:57:45] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:1214:2: note: 'next' match was here
[00:57:45]  ret void
[00:57:45]  ^
[00:57:45] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:1203:7: note: previous match ended here
[00:57:45] start:
[00:57:45]       ^
[00:57:45] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:1204:1: note: non-matching line after previous match is here
[00:57:45]  %1 = call i8* @_ZN5alloc4heap15exchange_malloc17h749231974d061700E(i64 4, i64 4)
[00:57:45] ^
[00:57:45]
[00:57:45] ------------------------------------------
[00:57:45]
[00:57:45] thread '[codegen] codegen/alloc-optimisation.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2480
[00:57:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:45]
[00:57:45]
[00:57:45] failures:
[00:57:45]     [codegen] codegen/alloc-optimisation.rs
