plain
 finished in 0.521 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 190 tests
..........F...F..................................i...................................... 88/190
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..............
failures:


---- [mir-opt] src/test/mir-opt/building/custom/simple_assign.rs stdout ----
thread '[mir-opt] src/test/mir-opt/building/custom/simple_assign.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/simple_assign/simple_assign.simple.mir_map.0.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/simple_assign`', src/tools/compiletest/src/runtest.rs:3426:21


---- [mir-opt] src/test/mir-opt/building/custom/references.rs stdout ----
thread '[mir-opt] src/test/mir-opt/building/custom/references.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/references/references.mut_ref.mir_map.0.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/references`', src/tools/compiletest/src/runtest.rs:3426:21

failures:
failures:
    [mir-opt] src/test/mir-opt/building/custom/references.rs
    [mir-opt] src/test/mir-opt/building/custom/simple_assign.rs
test result: FAILED. 183 passed; 2 failed; 5 ignored; 0 measured; 0 filtered out; finished in 6.23s

Build completed unsuccessfully in 0:10:52
