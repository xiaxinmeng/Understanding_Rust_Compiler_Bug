
---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----
12	-         falseUnwind -> [real: bb1, cleanup: bb2]; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
13	+         StorageLive(_4);                 // scope 0 at /the/src/instrument_coverage.rs:10:11: 10:11
14	+         _4 = const std::intrinsics::count_code_region(const 16004455475339839479_u64, const 0_u32, const "/the/src/instrument_coverage.rs", const 10_u32, const 11_u32, const 16_u32, const 2_u32) -> bb7; // scope 0 at /the/src/instrument_coverage.rs:10:11: 10:11
-	+                                          // ty::Const
-	+                                          // + ty: unsafe extern "rust-intrinsic" fn(u64, u32, &'static str, u32, u32, u32, u32) {std::intrinsics::count_code_region}
-	+                                          // + val: Value(Scalar(<ZST>))
18	+                                          // mir::Constant
19	+                                          // + span: /the/src/instrument_coverage.rs:10:11: 10:11
20	+                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(u64, u32, &'static str, u32, u32, u32, u32) {std::intrinsics::count_code_region}, val: Value(Scalar(<ZST>)) }

29	      bb1: {
30	          StorageLive(_2);                 // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
31	          _2 = const bar() -> [return: bb3, unwind: bb2]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
-	                                           // ty::Const
-	                                           // + ty: fn() -> bool {bar}
-	                                           // + val: Value(Scalar(<ZST>))
35	                                           // mir::Constant
36	                                           // + span: /the/src/instrument_coverage.rs:12:12: 12:15
37	                                           // + literal: Const { ty: fn() -> bool {bar}, val: Value(Scalar(<ZST>)) }

thread '[mir-opt] mir-opt/instrument_coverage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/instrument_coverage.main.InstrumentCoverage.diff', src/tools/compiletest/src/runtest.rs:3239:25
