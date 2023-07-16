
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 136 tests
i........i..i..i.......ii.i........F......i.............iii....ii.......i.......i................... 100/136
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..i.................................
failures:

---- [debuginfo-gdb] debuginfo/drop-locations.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support

error: compilation failed!
status: signal: 11 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/debuginfo/drop-locations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/drop-locations.gdb/a" "-Crpath" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-O" "-C" "no-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/drop-locations.gdb/auxiliary"
stdout: none
stderr: none



failures:
    [debuginfo-gdb] debuginfo/drop-locations.rs

test result: FAILED. 119 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out; finished in 4.09s
