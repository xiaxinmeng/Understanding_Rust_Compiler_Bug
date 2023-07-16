plain

failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/instrument_coverage.rs stdout ----
22           _2 = bar() -> [return: bb3, unwind: bb6]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
23                                            // mir::Constant
24                                            // + span: /the/src/instrument_coverage.rs:12:12: 12:15
-                                            // + literal: Const { ty: fn() -> bool {bar}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> bool {bar}, val: Value(<ZST>) }
27   
28       bb3: {


thread '[mir-opt] src/test/mir-opt/instrument_coverage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/instrument_coverage.main.InstrumentCoverage.diff', src/tools/compiletest/src/runtest.rs:3466:25


failures:
    [mir-opt] src/test/mir-opt/instrument_coverage.rs
