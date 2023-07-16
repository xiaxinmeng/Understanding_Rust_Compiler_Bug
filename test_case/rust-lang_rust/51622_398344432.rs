plain
[00:55:15] ............................................................i.......................................
[00:55:34] ...........................................................i........................................
[00:56:03] .......................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:56:05] .............
[00:56:22] ...F................................................................................................
[00:57:22] ................................................i.ii................................................
[00:57:56] ........................................................iiiiiii.....................................
[00:58:15] ....................................................................................................
[00:58:33] ....................................................................................................
---
[00:59:02] ---- [run-pass] run-pass/range_inclusive.rs stdout ----
[00:59:02] 
[00:59:02] error: compilation failed!
[00:59:02] status: exit code: 101
[00:59:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/range_inclusive.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/range_inclusive/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/range_inclusive/auxiliary"
[00:59:02] ------------------------------------------
[00:59:02] 
[00:59:02] ------------------------------------------
[00:59:02] stderr:
[00:59:02] stderr:
[00:59:02] ------------------------------------------
[00:59:02] warning: unused import: `RangeInclusive`
[00:59:02]   --> /checkout/src/test/run-pass/range_inclusive.rs:13:16
[00:59:02]    |
[00:59:02] 13 | use std::ops::{RangeInclusive, RangeToInclusive};
[00:59:02]    |
[00:59:02]    = note: #[warn(unused_imports)] on by default
[00:59:02] 
[00:59:02] 
[00:59:02] error[E0034]: multiple applicable items in scope
[00:59:02]    |
[00:59:02]    |
[00:59:02] 86 |     assert!(short.is_empty());
[00:59:02]    |                   ^^^^^^^^ multiple `is_empty` found
[00:59:02]    |
[00:59:02]    = note: candidate #1 is defined in an impl for the type `std::ops::RangeInclusive<_>`
[00:59:02]    = note: candidate #2 is defined in an impl of the trait `std::iter::ExactSizeIterator` for the type `std::ops::RangeInclusive<u8>`
[00:59:02] 
[00:59:02] error[E0034]: multiple applicable items in scope
[00:59:02]     |
[00:59:02]     |
[00:59:02] 101 |     assert!(long.is_empty());
[00:59:02]     |                  ^^^^^^^^ multiple `is_empty` found
[00:59:02]     |
[00:59:02]     = note: candidate #1 is defined in an impl for the type `std::ops::RangeInclusive<_>`
[00:59:02]     = note: candidate #2 is defined in an impl of the trait `std::iter::ExactSizeIterator` for the type `std::ops::RangeInclusive<u8>`
[00:59:02] 
[00:59:02] error[E0034]: multiple applicable items in scope
[00:59:02]     |
[00:59:02]     |
[00:59:02] 106 |     assert!(narrow.is_empty());
[00:59:02]     |                    ^^^^^^^^ multiple `is_empty` found
[00:59:02]     |
[00:59:02]     = note: candidate #1 is defined in an impl for the type `std::ops::RangeInclusive<_>`
[00:59:02] note: candidate #2 is defined in the trait `std::iter::ExactSizeIterator`
[00:59:02]     |
[00:59:02] 718 |     fn is_empty(&self) -> bool {
[00:59:02]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:02]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:02]     = help: to disambiguate the method call, write `std::iter::ExactSizeIterator::is_empty(narrow)` instead
[00:59:02] 
[00:59:02] error[E0034]: multiple applicable items in scope
[00:59:02]     |
[00:59:02]     |
[00:59:02] 110 |     assert!(zero.is_empty());
[00:59:02]     |                  ^^^^^^^^ multiple `is_empty` found
[00:59:02]     |
[00:59:02]     = note: candidate #1 is defined in an impl for the type `std::ops::RangeInclusive<_>`
[00:59:02]     = note: candidate #2 is defined in an impl of the trait `std::iter::ExactSizeIterator` for the type `std::ops::RangeInclusive<u8>`
[00:59:02] 
[00:59:02] error[E0034]: multiple applicable items in scope
[00:59:02]     |
[00:59:02]     |
[00:59:02] 114 |     assert!(high.is_empty());
[00:59:02]     |                  ^^^^^^^^ multiple `is_empty` found
[00:59:02]     |
[00:59:02]     = note: candidate #1 is defined in an impl for the type `std::ops::RangeInclusive<_>`
[00:59:02]     = note: candidate #2 is defined in an impl of the trait `std::iter::ExactSizeIterator` for the type `std::ops::RangeInclusive<u8>`
[00:59:02] 
[00:59:02] error[E0034]: multiple applicable items in scope
[00:59:02]     |
[00:59:02]     |
[00:59:02] 119 |     assert!(nonsense.is_empty());
[00:59:02]     |                      ^^^^^^^^ multiple `is_empty` found
[00:59:02]     |
[00:59:02]     = note: candidate #1 is defined in an impl for the type `std::ops::RangeInclusive<_>`
[00:59:02] note: candidate #2 is defined in the trait `std::iter::ExactSizeIterator`
[00:59:02]     |
[00:59:02] 718 |     fn is_empty(&self) -> bool {
[00:59:02]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:02]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:02]     = help: to disambiguate the method call, write `std::iter::ExactSizeIterator::is_empty(nonsense)` instead
[00:59:02] error: aborting due to 6 previous errors
[00:59:02] 
[00:59:02] For more information about this error, try `rustc --explain E0034`.
[00:59:02] 
---
[00:59:02] 
[00:59:02] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:59:02] 
[00:59:02] 
[00:59:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:02] 
[00:59:02] 
[00:59:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:02] Build completed unsuccessfully in 0:12:52
[00:59:02] Build completed unsuccessfully in 0:12:52
[00:59:02] Makefile:58: recipe for target 'check' failed
[00:59:02] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b579346
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
