
------------------------------------------
stderr:
------------------------------------------
This application has requested the Runtime to terminate it in an unusual way.
Please contact the application's support team for more information.
Assertion failed!
Program: C:\projects\rust\build\x86_64-pc-windows-gnu\test\run-make\extern-fn-struct-passing-abi.stage2-x86_64-pc-windows-gnu\test.exe
File: test.c, Line 310
Expression: f1.x == 7.
make: *** [Makefile:5: all] Error 1
------------------------------------------
thread '[run-make] run-make\extern-fn-struct-passing-abi' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2435:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    [run-make] run-make\extern-fn-struct-passing-abi
test result: FAILED. 158 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:322:21
