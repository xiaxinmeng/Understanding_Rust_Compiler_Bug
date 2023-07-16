

failures:

---- [run-pass] run-pass/issue-23611-enum-swap-in-drop.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/issue-23611-enum-swap-in-drop.stage2-x86_64-apple-darwin 
stdout:
------------------------------------------
                                        created empty log
+-- Make D(test_1, 10000000)
| +-- Make D(g_b_5, 50000001)
| |                                     in g_B(b2b0) from E::drop, b=b4b0
| +-- Drop D(g_b_5, 50000001)
|                                       
| +-- Make D(drop_6, 60000002)
| | +-- Make D(GaspA::drop_2, 20000003)
| | | +-- Make D(f_a_4, 40000004)
| | | |                                 in f_A(a3a0) from GaspA::drop
| | | +-- Drop D(f_a_4, 40000004)
| | |                                   
| | +-- Drop D(GaspA::drop_2, 20000003)
| |                                     
| +-- Drop D(drop_6, 60000002)
|                                       

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: `(left == right)` (left: `[50000001, 40000004, 20000003, 60000002]`, right: `[50000001, 50000003, 50000005, 30000004, 10000000, 40000007, 20000006, 60000002]`)', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/issue-23611-enum-swap-in-drop.rs:27

------------------------------------------

thread '[run-pass] run-pass/issue-23611-enum-swap-in-drop.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/compiletest/runtest.rs:1512



failures:
    [run-pass] run-pass/issue-23611-enum-swap-in-drop.rs
