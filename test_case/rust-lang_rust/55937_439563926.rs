plain
travis_time:end:0451ecdb:start=1542408162033090880,finish=1542408220208908539,duration=58175817659
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a6/da/c99b10bfc509cbbea520886d2e8fe0e738e3ce22e2f528381f3bb2229433/awscli-1.16.57-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 15.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:55:40] .................................................................................................... 100/5021
[00:55:43] .................................................................................................... 200/5021
[00:55:46] .............................ii............................................ii...................ii.. 300/5021
[00:55:48] ..............................................................................................iii... 400/5021
[00:55:51] .....iiiiiiii.iii............................iii...........................................i........ 500/5021
[00:55:59] .................................................................................................... 700/5021
[00:56:05] .................................................................................i...........i...... 800/5021
[00:56:09] .................................................................................................... 900/5021
[00:56:12] iiiii...................iiiiii...................................................................... 1000/5021
---
[00:56:50] .................................................................................................... 2200/5021
[00:56:54] .................................................................................................... 2300/5021
[00:56:58] .................................................................................................... 2400/5021
[00:57:02] .................................................................................................... 2500/5021
[00:57:06] .................................................................................iiiiiiiii.......... 2600/5021
[00:57:13] ...............................................ii................................................... 2800/5021
[00:57:16] .................................................................................................... 2900/5021
[00:57:20] .................................................................................................... 3000/5021
[00:57:24] ..........................................i......................................................... 3100/5021
---
[01:12:11] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:12:11] F..............................................
[01:12:11] failures:
[01:12:11] 
[01:12:11] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:12:11] thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:11] Current block: None
[01:12:11] Actual Line: "        AscribeUserType(_4, o, UserTypeProjection { base: UserTypeAnnotation(1), projs: [] });"
[01:12:11] Expected Line: "       AscribeUserType(_4, o, UserTypeProjection { base: Ty(Canonical { max_universe: U0, variables: [], value: std::option::Option<std::boxed::Box<u32>> }), projs: [] });"
[01:12:11] Test Name: rustc.main.SimplifyCfg-initial.after.mir
[01:12:11] ... (elided)
[01:12:11]    bb0: {
[01:12:11]    bb0: {
[01:12:11]        StorageLive(_1);
[01:12:11]        _1 = const false;
[01:12:11]        FakeRead(ForLet, _1);
[01:12:11]        StorageLive(_2);
[01:12:11]        StorageLive(_3);
[01:12:11]        _3 = _1;
[01:12:11]        _2 = move _3;
[01:12:11]        StorageDead(_3);
[01:12:11]        StorageLive(_4);
[01:12:11]        _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:12:11]        FakeRead(ForLet, _4);
[01:12:11]        AscribeUserType(_4, o, UserTypeProjection { base: Ty(Canonical { max_universe: U0, variables: [], value: std::option::Option<std::boxed::Box<u32>> }), projs: [] });
[01:12:11]        StorageLive(_5);
[01:12:11]        StorageLive(_6);
[01:12:11]        _6 = move _4;
[01:12:11]        replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:12:11] ... (elided)
[01:12:11]    bb2: {
[01:12:11]    bb2: {
[01:12:11]        drop(_6) -> [return: bb6, unwind: bb4];
[01:12:11] ... (elided)
[01:12:11]    bb5: {
[01:12:11]        drop(_6) -> bb4;
[01:12:11]    }
[01:12:11]    }
[01:12:11] Actual:
[01:12:11] fn main() -> (){
[01:12:11]     let mut _0: ();
[01:12:11]     scope 1 {
[01:12:11]         scope 3 {
[01:12:11]             scope 5 {
[01:12:11]                 scope 7 {
[01:12:11]                 scope 8 {
[01:12:11]                 scope 8 {
[01:12:11]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[01:12:11]             }
[01:12:11]             scope 6 {
[01:12:11]             scope 6 {
[01:12:11]                 let _4: std::option::Option<std::boxed::Box<u32>> as UserTypeProjection { base: UserTypeAnnotation(0), projs: [] };
[01:12:11]         }
[01:12:11]         scope 4 {
[01:12:11]         scope 4 {
[01:12:11]             let _2: bool;
[01:12:11]     }
[01:12:11]     scope 2 {
[01:12:11]     scope 2 {
[01:12:11]         let _1: bool;
[01:12:11]     }
[01:12:11]     let mut _3: bool;
[01:12:11]     let mut _6: std::option::Option<std::boxed::Box<u32>>;
[01:12:11]     bb0: {                              
[01:12:11]         StorageLive(_1);
[01:12:11]         _1 = const false;
[01:12:11]         FakeRead(ForLet, _1);
[01:12:11]         StorageLive(_2);
[01:12:11]         StorageLive(_3);
[01:12:11]         _3 = _1;
[01:12:11]         _2 = move _3;
[01:12:11]         StorageDead(_3);
[01:12:11]         StorageLive(_4);
[01:12:11]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:12:11]         FakeRead(ForLet, _4);
[01:12:11]         AscribeUserType(_4, o, UserTypeProjection { base: UserTypeAnnotation(1), projs: [] });
[01:12:11]         StorageLive(_5);
[01:12:11]         StorageLive(_6);
[01:12:11]         _6 = move _4;
[01:12:11]         replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:12:11]     bb1: {
[01:12:11]         resume;
[01:12:11]     }
[01:12:11]     }
[01:12:11]     bb2: {                              
[01:12:11]         drop(_6) -> [return: bb6, unwind: bb4];
[01:12:11]     bb3: {
[01:12:11]         drop(_4) -> bb1;
[01:12:11]     }
[01:12:11]     bb4: {
[01:12:11]     bb4: {
[01:12:11]         drop(_5) -> bb3;
[01:12:11]     }
[01:12:11]     bb5: {
[01:12:11]         drop(_6) -> bb4;
[01:12:11]     }
[01:12:11]     bb6: {                              
[01:12:11]         StorageDead(_6);
[01:12:11]         _0 = ();
[01:12:11]         drop(_5) -> [return: bb7, unwind: bb3];
[01:12:11]     }
[01:12:11]     bb7: {                              
[01:12:11]         StorageDead(_5);
[01:12:11]         drop(_4) -> [return: bb8, unwind: bb1];
[01:12:11]     }
-cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:11] 
[01:12:11] 
[01:12:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:11] Build completed unsuccessfully in 0:20:30
[01:12:11] Build completed unsuccessfully in 0:20:30
[01:12:11] make: *** [check] Error 1
[01:12:11] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08faf214
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 16 23:56:00 UTC 2018
---
travis_time:end:0517e900:start=1542412562242349711,finish=1542412562385407920,duration=143058209
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:195d0a82
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:034b0ab4
$ dmesg | grep -i kill
