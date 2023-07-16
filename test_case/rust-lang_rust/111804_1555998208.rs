plain
..................iii.iiiii.........ii........i................................

failures:

---- [mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs' panicked at 'the mir dump file for inherit_overflow.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/inherit_overflow.rs)', src/tools/compiletest/src/runtest.rs:3652:17

---- [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs' panicked at 'the mir dump file for inherit_overflow.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/inherit_overflow.rs)', src/tools/compiletest/src/runtest.rs:3652:17

failures:
    [mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs
    [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs
