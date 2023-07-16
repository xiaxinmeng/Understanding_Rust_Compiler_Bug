plain
test [mir-opt] src/test/mir-opt/inline/polymorphic_recursion.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/coverage_graphviz.rs stdout ----
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
7     bcb1__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb1</td></tr><tr><td align="left" balign="left">Expression(bcb0 + bcb3) at 10:5-11:17<br align="left"/>    11:12-11:17: @2.Call: _2 = bar() -&gt; [return: bb3, unwind: bb6]</td></tr><tr><td align="left" balign="left">bb1: FalseUnwind<br align="left"/>bb2: Call</td></tr><tr><td align="left" balign="left">bb3: SwitchInt</td></tr></table>>];
8     bcb0__Cov_0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">bcb0</td></tr><tr><td align="left" balign="left"></td></tr><tr><td align="left" balign="left">Counter(bcb0) at 9:1-9:11<br align="left"/>    </td></tr><tr><td align="left" balign="left">bb0: Goto</td></tr></table>>];
9     bcb3__Cov_0_3 -> bcb1__Cov_0_3 [label=<>];
-     bcb1__Cov_0_3 -> bcb3__Cov_0_3 [label=<false>];
+     bcb1__Cov_0_3 -> bcb3__Cov_0_3 [label=<0>];
11     bcb1__Cov_0_3 -> bcb2__Cov_0_3 [label=<otherwise>];
12     bcb0__Cov_0_3 -> bcb1__Cov_0_3 [label=<>];


thread '[mir-opt] src/test/mir-opt/coverage_graphviz.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/coverage_graphviz.main.InstrumentCoverage.0.dot', src/tools/compiletest/src/runtest.rs:3451:21

---- [mir-opt] src/test/mir-opt/instrument_coverage.rs stdout ----
26       }
27   
27   
28       bb3: {
-           switchInt(move _2) -> [false: bb5, otherwise: bb4]; // scope 0 at /the/src/instrument_coverage.rs:+2:12: +2:17
+           switchInt(move _2) -> [0: bb5, otherwise: bb4]; // scope 0 at /the/src/instrument_coverage.rs:+2:12: +2:17
31   
32       bb4: {


thread '[mir-opt] src/test/mir-opt/instrument_coverage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/instrument_coverage.main.InstrumentCoverage.diff', src/tools/compiletest/src/runtest.rs:3451:21

failures:
    [mir-opt] src/test/mir-opt/coverage_graphviz.rs
    [mir-opt] src/test/mir-opt/instrument_coverage.rs
