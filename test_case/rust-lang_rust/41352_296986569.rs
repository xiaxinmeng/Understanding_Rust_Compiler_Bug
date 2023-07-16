
[01:28:58] failures:
[01:28:58] 
[01:28:58] ---- [run-make] run-make/sysroot-crates-are-unstable stdout ----
[01:28:58] 	
[01:28:58] error: make failed
[01:28:58] status: exit code: 2
[01:28:58] command: "make"
[01:28:58] stdout:
[01:28:58] ------------------------------------------
[01:28:58] verifying alloc is an unstable crate
[01:28:58] verifying alloc_jemalloc is an unstable crate
[01:28:58] verifying alloc_system is an unstable crate
[01:28:58] verifying arena is an unstable crate
[01:28:58] verifying bitflags is an unstable crate
[01:28:58] verifying clang_rt.asan_osx_dynamic.dylib is an unstable crate
[01:28:58] verifying clang_rt.tsan_osx_dynamic.dylib is an unstable crate
[01:28:58] crate clang_rt.asan_osx_dynamic.dylib is not unstable
[01:28:58] error: expected one of `;` or `as`, found `.`
[01:28:58]  --> <anon>:1:22
[01:28:58]   |
[01:28:58] 1 | extern crate clang_rt.asan_osx_dynamic.dylib;
[01:28:58]   |                      ^ expected one of `;` or `as` here
[01:28:58] 
[01:28:58] error: aborting due to previous error
[01:28:58] 
[01:28:58] crate clang_rt.tsan_osx_dynamic.dylib is not unstable
[01:28:58] error: expected one of `;` or `as`, found `.`
[01:28:58]  --> <anon>:1:22
[01:28:58]   |
[01:28:58] 1 | extern crate clang_rt.tsan_osx_dynamic.dylib;
[01:28:58]   |                      ^ expected one of `;` or `as` here
[01:28:58] 
[01:28:58] error: aborting due to previous error
[01:28:58] 
[01:28:58] 
[01:28:58] ------------------------------------------
[01:28:58] stderr:
[01:28:58] ------------------------------------------
[01:28:58] make[1]: *** [check-crate-clang_rt.asan_osx_dynamic.dylib-is-unstable] Error 1
[01:28:58] make[1]: *** Waiting for unfinished jobs....
[01:28:58] make[1]: *** [check-crate-clang_rt.tsan_osx_dynamic.dylib-is-unstable] Error 1
[01:28:58] 
[01:28:58] ------------------------------------------
[01:28:58] 
[01:28:58] thread '[run-make] run-make/sysroot-crates-are-unstable' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2627
[01:28:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:28:58] 
[01:28:58] 
[01:28:58] failures:
[01:28:58]     [run-make] run-make/sysroot-crates-are-unstable
[01:28:58] 
[01:28:58] test result: FAILED. 149 passed; 1 failed; 0 ignored; 0 measured
