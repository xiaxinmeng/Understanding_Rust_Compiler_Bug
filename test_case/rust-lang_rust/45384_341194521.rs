
[00:57:41] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[00:57:41] 	thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:57:41] Expected Line: "        _8 = CheckedAdd(const 1i32, _7); // scope 1 at /checkout/src/test/mir-opt/match_false_edges.rs:23:31: 23:36"
[00:57:41] Actual Line: "     _1 = Add(const 1i32, _7);"
[00:57:41] Expected:
[00:57:41] ... (elided)
[00:57:41]  bb0: {
[00:57:41] ... (elided)
[00:57:41]      _2 = std::option::Option<i32>::Some(const 42i32,);
[00:57:41]      _5 = discriminant(_2);
[00:57:41]      switchInt(_5) -> [0isize: bb5, otherwise: bb3];
[00:57:41]  }
[00:57:41]  bb1: { // arm1
[00:57:41]      StorageLive(_7);
[00:57:41]      _7 = _3;
[00:57:41]      _1 = Add(const 1i32, _7);
[00:57:41] ... (elided)
[00:57:41]      goto -> bb11;
[00:57:41]  }
[00:57:41]  bb2: { // binding3(empty) and arm3
[00:57:41]      _1 = const 3i32;
[00:57:41]      goto -> bb11;
[00:57:41]  }
[00:57:41]  bb3: {
[00:57:41]      falseEdges -> [real: bb7, imaginary: bb4]; //pre_binding1
[00:57:41]  }
[00:57:41]  bb4: {
[00:57:41]      falseEdges -> [real: bb10, imaginary: bb5]; //pre_binding2
[00:57:41]  }
[00:57:41]  bb5: {
[00:57:41]      falseEdges -> [real: bb2, imaginary: bb6]; //pre_binding3
[00:57:41]  }
[00:57:41]  bb6: {
[00:57:41]      unreachable;
[00:57:41]  }
[00:57:41]  bb7: { // binding1 and guard
[00:57:41]      StorageLive(_3);
[00:57:41]      _3 = ((_2 as Some).0: i32);
[00:57:41]      StorageLive(_6);
[00:57:41]      _6 = const guard() -> bb8;
[00:57:41]  }
[00:57:41]  bb8: { // end of guard
[00:57:41]      switchInt(_6) -> [0u8: bb9, otherwise: bb1];
[00:57:41]  }
[00:57:41]  bb9: { // to pre_binding2
[00:57:41]      falseEdges -> [real: bb4, imaginary: bb4];
[00:57:41]  }
[00:57:41]  bb10: { // binding2 and arm2
[00:57:41]      StorageLive(_4);
[00:57:41]      _4 = ((_2 as Some).0: i32);
[00:57:41]      StorageLive(_8);
[00:57:41]      _8 = _4;
[00:57:41]      _1 = Add(const 2i32, _8);
[00:57:41]      StorageDead(_8);
[00:57:41]      goto -> bb11;
[00:57:41]  }
[00:57:41]  bb11: {
[00:57:41] ... (elided)
[00:57:41]      return;
[00:57:41]  }
[00:57:41] Actual:
[00:57:41] fn full_tested_match() -> () {
[00:57:41]     let mut _0: ();
[00:57:41]     scope 1 {
[00:57:41]         let _3: i32;
[00:57:41]         scope 2 {
[00:57:41]             let _4: i32;
[00:57:41]         }
[00:57:41]     }
[00:57:41]     let mut _1: i32;
[00:57:41]     let mut _2: std::option::Option<i32>;
[00:57:41]     let mut _5: isize;
[00:57:41]     let mut _6: bool;
[00:57:41]     let mut _7: i32;
[00:57:41]     let mut _8: (i32, bool);
[00:57:41]     let mut _9: i32;
[00:57:41]     let mut _10: (i32, bool);
[00:57:41]     bb0: {                              
[00:57:41]         StorageLive(_1);
[00:57:41]         StorageLive(_2);
[00:57:41]         _2 = std::option::Option<i32>::Some(const 42i32,);
[00:57:41]         _5 = discriminant(_2);
[00:57:41]         switchInt(_5) -> [0isize: bb5, otherwise: bb3];
[00:57:41]     }
[00:57:41]     bb1: {                              
[00:57:41]         StorageLive(_7);
[00:57:41]         _7 = _3;
[00:57:41]         _8 = CheckedAdd(const 1i32, _7);
[00:57:41]         assert(!(_8.1: bool), "attempt to add with overflow") -> bb12;
[00:57:41]     }
[00:57:41]     bb2: {                              
[00:57:41]         _1 = const 3i32;
[00:57:41]         goto -> bb11;
[00:57:41]     }
[00:57:41]     bb3: {                              
[00:57:41]         falseEdges -> [real: bb7, imaginary: bb4];
[00:57:41]     }
[00:57:41]     bb4: {                              
[00:57:41]         falseEdges -> [real: bb10, imaginary: bb5];
[00:57:41]     }
[00:57:41]     bb5: {                              
[00:57:41]         falseEdges -> [real: bb2, imaginary: bb6];
[00:57:41]     }
[00:57:41]     bb6: {                              
[00:57:41]         unreachable;
[00:57:41]     }
[00:57:41]     bb7: {                              
[00:57:41]         StorageLive(_3);
[00:57:41]         _3 = ((_2 as Some).0: i32);
[00:57:41]         StorageLive(_6);
[00:57:41]         _6 = const guard() -> bb8;
[00:57:41]     }
[00:57:41]     bb8: {                              
[00:57:41]         switchInt(_6) -> [0u8: bb9, otherwise: bb1];
[00:57:41]     }
[00:57:41]     bb9: {                              
[00:57:41]         falseEdges -> [real: bb4, imaginary: bb4];
[00:57:41]     }
[00:57:41]     bb10: {                             
[00:57:41]         StorageLive(_4);
[00:57:41]         _4 = ((_2 as Some).0: i32);
[00:57:41]         StorageLive(_9);
[00:57:41]         _9 = _4;
[00:57:41]         _10 = CheckedAdd(const 2i32, _9);
[00:57:41]         assert(!(_10.1: bool), "attempt to add with overflow") -> bb13;
[00:57:41]     }
[00:57:41]     bb11: {                             
[00:57:41]         StorageDead(_4);
[00:57:41]         StorageDead(_6);
[00:57:41]         StorageDead(_3);
[00:57:41]         StorageDead(_1);
[00:57:41]         StorageDead(_2);
[00:57:41]         _0 = ();
[00:57:41]         return;
[00:57:41]     }
[00:57:41]     bb12: {                             
[00:57:41]         _1 = (_8.0: i32);
[00:57:41]         StorageDead(_7);
[00:57:41]         goto -> bb11;
[00:57:41]     }
[00:57:41]     bb13: {                             
[00:57:41]         _1 = (_10.0: i32);
[00:57:41]         StorageDead(_9);
[00:57:41]         goto -> bb11;
[00:57:41]     }
[00:57:41] }', /checkout/src/tools/compiletest/src/runtest.rs:2324:12
[00:57:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:41] 
[00:57:41] 
[00:57:41] failures:
[00:57:41]     [mir-opt] mir-opt/match_false_edges.rs
[00:57:41] 
[00:57:41] test result: [31mFAILED(B[m. 32 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
