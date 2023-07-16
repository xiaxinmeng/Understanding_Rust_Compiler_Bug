plain
travis_time:end:00db38dc:start=1545235908893901426,finish=1545235964518825501,duration=55624924075
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
[00:52:46] 
[00:52:46] running 37 tests
[00:52:47] ERROR 2018-12-19T17:05:40Z: compiletest::runtest: None
serType(_4, o, UserTypeProjection { base: UserTypeAnnotation(1), projs: [] });
[00:53:04]        StorageLive(_5);
[00:53:04]        StorageLive(_6);
[00:53:04]        _6 = move _4;
[00:53:04]        replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[00:53:04] ... (elided)
[00:53:04]    bb2: {
[00:53:04]    bb2: {
[00:53:04]        drop(_6) -> [return: bb6, unwind: bb4];
[00:53:04] ... (elided)
[00:53:04]    bb5: {
[00:53:04]        drop(_6) -> bb4;
[00:53:04]    }
[00:53:04]    }
[00:53:04] Actual:
[00:53:04] | User Type Annotations
[00:53:04] | 0: Canonical { max_universe: U0, variables: [], value: Ty(std::option::Option<std::boxed::Box<u32>>) } at /checkout/src/test/mir-opt/basic_assignment.rs:37:18: 37:34
[00:53:04] fn main() -> (){
[00:53:04] fn main() -> (){
[00:53:04]     let mut _0: ();
[00:53:04]     scope 1 {
[00:53:04]         scope 3 {
[00:53:04]             scope 5 {
[00:53:04]                 scope 7 {
[00:53:04]                 scope 8 {
[00:53:04]                 scope 8 {
[00:53:04]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[00:53:04]             }
[00:53:04]             scope 6 {
[00:53:04]             scope 6 {
[00:53:04]                 let _4: std::option::Option<std::boxed::Box<u32>> as UserTypeProjection { base: UserTypeAnnotation(0), projs: [] };
[00:53:04]         }
[00:53:04]         scope 4 {
[00:53:04]         scope 4 {
[00:53:04]             let _2: bool;
[00:53:04]     }
[00:53:04]     scope 2 {
[00:53:04]     scope 2 {
[00:53:04]         let _1: bool;
[00:53:04]     }
[00:53:04]     let mut _3: bool;
[00:53:04]     let mut _6: std::option::Option<std::boxed::Box<u32>>;
[00:53:04]     bb0: {                              
[00:53:04]         StorageLive(_1);
[00:53:04]         _1 = const false;
[00:53:04]         FakeRead(ForLet, _1);
[00:53:04]         StorageLive(_2);
[00:53:04]         StorageLive(_3);
[00:53:04]         _3 = _1;
[00:53:04]         _2 = move _3;
[00:53:04]         StorageDead(_3);
[00:53:04]         StorageLive(_4);
[00:53:04]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[00:53:04]         FakeRead(ForLet, _4);
[00:53:04]         AscribeUserType(_4, o, UserTypeProjection { base: UserTypeAnnotation(0), projs: [] });
[00:53:04]         StorageLive(_5);
[00:53:04]         StorageLive(_6);
[00:53:04]         _6 = move _4;
[00:53:04]         replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[00:53:04]     bb1: {
[00:53:04]         resume;
[00:53:04]     }
[00:53:04]     }
[00:53:04]     bb2: {                              
[00:53:04]         drop(_6) -> [return: bb6, unwind: bb4];
[00:53:04]     bb3: {
[00:53:04]         drop(_4) -> bb1;
[00:53:04]     }
[00:53:04]     bb4: {
[00:53:04]     bb4: {
[00:53:04]         drop(_5) -> bb3;
[00:53:04]     }
[00:53:04]     bb5: {
[00:53:04]         drop(_6) -> bb4;
[00:53:04]     }
[00:53:04]     bb6: {                              
[00:53:04]         StorageDead(_6);
[00:53:04]         _0 = ();
[00:53:04]         drop(_5) -> [return: bb7, unwind: bb3];
[00:53:04]     }
[00:53:04]     bb7: {                              
[00:53:04]         StorageDead(_5);
[00:53:04]         drop(_4) -> [return: bb8, unwind: bb1];
[00:53:04]     }
[00:53:04]     bb8: {                     " "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:04] 
[00:53:04] 
[00:53:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:04] Build completed unsuccessfully in 0:10:30
[00:53:04] Build completed unsuccessfully in 0:10:30
[00:53:04] Makefile:58: recipe for target 'check' failed
[00:53:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09f48f72
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 17:05:57 UTC 2018
