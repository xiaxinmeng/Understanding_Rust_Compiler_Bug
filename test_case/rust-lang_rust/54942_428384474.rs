plain
[00:54:02] .................................................................................................... 2200/4578
[00:54:07] ......i............................................................................................. 2300/4578
[00:54:11] .................................................................................................... 2400/4578
[00:54:14] .................................................................................................... 2500/4578
[00:54:18] ...................iiiiiiiii........................................................................ 2600/4578
[00:54:24] .................................................................................................... 2800/4578
[00:54:28] .................................................................................................... 2900/4578
[00:54:31] .......................................i............................................................ 3000/4578
[00:54:34] ...................................................................................................i 3100/4578
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:22] 
[01:07:22] running 47 tests
[01:07:22] ERROR 2018-10-09T23:13:49Z: compiletest::runtest: None
[01:07:41]        AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });
[01:07:41]        FakeRead(ForLet, _4);
[01:07:41]        StorageLive(_5);
[01:07:41]        StorageLive(_6);
[01:07:41]        _6 = move _4;
[01:07:41]        replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:07:41] ... (elided)
[01:07:41]    bb2: {
[01:07:41]    bb2: {
[01:07:41]        drop(_6) -> [return: bb6, unwind: bb4];
[01:07:41] ... (elided)
[01:07:41]    bb5: {
[01:07:41]        drop(_6) -> bb4;
[01:07:41]    }
[01:07:41]    }
[01:07:41] Actual:
[01:07:41] fn main() -> (){
[01:07:41]     let mut _0: ();
[01:07:41]     scope 1 {
[01:07:41]         scope 3 {
[01:07:41]             scope 5 {
[01:07:41]                 scope 7 {
[01:07:41]                 scope 8 {
[01:07:41]                 scope 8 {
[01:07:41]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[01:07:41]             }
[01:07:41]             scope 6 {
[01:07:41]             scope 6 {
[01:07:41]                 let _4: std::option::Option<std::boxed::Box<u32>> as Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> };
[01:07:41]         }
[01:07:41]         scope 4 {
[01:07:41]         scope 4 {
[01:07:41]             let _2: bool;
[01:07:41]     }
[01:07:41]     scope 2 {
[01:07:41]     scope 2 {
[01:07:41]         let _1: bool;
[01:07:41]     }
[01:07:41]     let mut _3: bool;
[01:07:41]     let mut _6: std::option::Option<std::boxed::Box<u32>>;
[01:07:41]     bb0: {                              
[01:07:41]         StorageLive(_1);
[01:07:41]         _1 = const fal1:07:41] expected success, got: exit code: 101
[01:07:41] 
[01:07:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:41] Build completed unsuccessfully in 0:18:27
[01:07:41] Build completed unsuccessfully in 0:18:27
[01:07:41] Makefile:58: recipe for target 'check' failed
[01:07:41] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26a38a40
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
