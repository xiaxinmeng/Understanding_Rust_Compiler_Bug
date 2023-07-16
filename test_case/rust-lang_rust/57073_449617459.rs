plain
travis_time:end:00a64580:start=1545542584805937420,finish=1545542585833384030,duration=1027446610
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:26] 
[01:05:26] running 38 tests
[01:05:44]     let mut _12: bool;
[01:05:44]     let mut _13: bool;
[01:05:44]     let mut _14: bool;
[01:05:44]     let mut _15: bool;
[01:05:44]     let mut _16: bool;
[01:05:44]     let mut _17: u32;
[01:05:44]     let mut _18: u32;
[01:05:44]     bb0: {                              
[01:05:44]         StorageLive(_2);
[01:05:44]         StorageLive(_3);
[01:05:44]         StorageLive(_4);
[01:05:44]         StorageLive(_5);
[01:05:44]         StorageLive(_6);
[01:05:44]         StorageLive(_7);
[01:05:44]         StorageLive(_8);
[01:05:44]         _8 = _1;
[01:05:44]         _7 = const compiler_builtins::int::addsub::rust_i128_add(move _8, const 1i128) -> bb7;
[01:05:44]     }
[01:05:44]     bb1: {                              
[01:05:44]         _10 = Eq(const 4i128, const -1i128);
[01:05:44]         _11 = Eq(_5, const ::std::i128::MIN);
[01:05:44]         _12 = BitAnd(move _10, move _11);
[01:05:44]         assert(!move _12, "attempt to divide with overflow") -> bb2;
[01:05:44]     }
[01:05:44]     bb2: {                              
[01:05:44]         _4 = const compiler_builtins::int::sdiv::rust_i128_div(move _5, const 4i128) -> bb8;
[01:05:44]     }
[01:05:44]     bb3: {                              
[01:05:44]         _14 = Eq(const 5i128, const -1i128);
[01:05:44]         _15 = Eq(_4, const ::std::i128::MIN);
[01:05:44]         _16 = BitAnd(move _14, move _15);
[01:05:44]         assert(!move _16, "attempt to calculate the remainder with overflow") -> bb4;
[01:05:44]     }
[01:05:44]     bb4: {                              
[01:05:44]         _3 = const compiler_builtins::int::sderflow") -> bb7;
[01:05:44]     }
[01:05:44]     bb7: {                              
[01:05:44]         _3 = const compiler_builtins::int::sdiv::rust_i128_rem(move _4, const 5i128) -> bb15;
[01:05:44]     }
[01:05:44]     bb8: {                              
[01:05:44]         _2 = move (_20.0: i128);
[01:05:44]         StorageDead(_3);
[01:05:44]         StorageLive(_23);
[01:05:44]         _23 = const 7i32 as u128 (Misc);
[01:05:44]         _21 = const compiler_builtins::int::shift::rust_i128_shro(move _2, move _23) -> bb16;
[01:05:44]     }
[01:05:44]     bb9: {                              
[01:05:44]         _0 = move (_21.0: i128);
[01:05:44]         StorageDead(_2);
[01:05:44]         return;
[01:05:44]     }
[01:05:44]     bb10: {                             
[01:05:44]         assert(!move (_9.1: bool), "attempt to add with overflow") -> bb1;
[01:05:44]     }
[01:05:44]     bb11: {                             
[01:05:44]         assert(!move (_10.1: bool), "attempt to subtract with overflow") -> bb2;
[01:05:44]     }
[01:05:44]     bb12: {                             
[01:05:44]         assert(!move (_11.1: bool), "attempt to multiply with overflow") -> bb3;
[01:05:44]     }
[01:05:44]     bb13: {                             
[01:05:44]         StorageDead(_5);
[01:05:44]         _16 = Eq(const 5i128, const 0i128);
[01:05:44]         assert(!move _16, "attempt to calculate the remainder with a divisor of zero") -> bb6;
[01:05:44]     }
[01:05:44]     bb14: {                             
[01:05:44]         StorageDead(_22);
[01:05:44]         assert(!move (_20.1: bool), "attempt to sht-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:44] 
[01:05:44] 
[01:05:44] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:505:22
[01:05:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:44] Build completed unsuccessfully in 0:10:24
[01:05:44] Makefile:58: recipe for target 'check' failed
[01:05:44] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19da1ec5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 23 06:28:58 UTC 2018
---
travis_time:end:2332a858:start=1545546539890449956,finish=1545546539950413353,duration=59963397
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:082269c9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:169f99e9
$ dmesg | grep -i kill
