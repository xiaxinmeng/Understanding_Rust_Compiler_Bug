plain
[00:47:17] ...........................................................................i........................
[00:47:22] ....................................................................................................
[00:47:28] ....................................................................................................
[00:47:35] ....................................................................................................
[00:47:40] ........i.................iiiiiiiii...................................................
[00:47:40] 
[00:47:40] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:48:35] ...........................................................................i........................
[00:48:40] ....................................................................................................
[00:48:45] ....................................................................................................
[00:48:51] ....................................................................................................
[00:48:56] ........i.................iiiiiiiii...................................................
[00:48:56] 
[00:48:56]  finished in 76.529
[00:48:56] travis_fold:end:test_ui_nll

---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:35] 
[01:01:35] running 50 tests
[01:01:40] ERROR 2018-06-03T06:03:47Z: compiletest::runtest: None
[01:01:40] ERROR 2018-06-03T06:03:47Z: compiletest::runtest: None
[01:01:41] ERROR 2018-06-03T06:03:48Z: compiletest::runtest: None
[01:01:42] ERROR 2018-06-03T06:03:49Z: compiletest::runtest: None
[01:01:42] ERROR 2018-06-03T06:03:49Z: compiletest::runtest: None
[01:01:53] ERROR 2018-06-03T06:04:01Z: compiletest::runtest: Some("    bb0: {")
  }
[01:01:57]     bb1: {                              
[01:01:57]         falseUnwind -> [real: bb2, cleanup: bb3];
[01:01:57]     }
[01:01:57]     bb2: {                              
[01:01:57]         StorageLive(_2);
[01:01:57]         StorageLive(_3);
[01:01:57]         StorageLive(_4);
[01:01:57]         _4 = std::option::Option<&'36_0rs S<'36_0rs>>::None;
[01:01:57]         _3 = const <std::cell::Cell<T>>::new(move _4) -> [return: bb4, unwind: bb3];
[01:01:57]     bb3: {
[01:01:57]         resume;
[01:01:57]     }
[01:01:57]     }
[01:01:57]     bb4: {                              
[01:01:57]         StorageDead(_4);
[01:01:57]         _2 = S<'36_0rs> { r: move _3 };
[01:01:57]         StorageDead(_3);
[01:01:57]         StorageLive(_6);
[01:01:57]         _6 = &'18s (_2.0: std::cell::Cell<std::option::Option<&'36_0rs S<'36_0rs>>>);
[01:01:57]         StorageLive(_7);
[01:01:57]         StorageLive(_8);
[01:01:57]         StorageLive(_9);
[01:01:57]         _9 = &'36_0rs _2;
[01:01:57]         _8 = &'36_0rs (*_9);
[01:01:57]         _7 = std::option::Option<&'36_0rs S<'36_0rs>>::Some(move _8,);
[01:01:57]         StorageDead(_8);
[01:01:57]         _5 = const <std::cell::Cell<T>>::set(move _6, move _7) -> [return: bb5, unwind: bb3];
[01:01:57]     }
[01:01:57]     bb5: {                              
[01:01:57]         EndRegion('18s);
[01:01:57]         StorageDead(_7);
[01:01:57]         StorageDead(_6);
[01:01:57]         StorageDead(_9);
[01:01:57]         StorageLive(_11);
[01:01:57]         _11 = const query() -> [return: bb6, unwind: bb3];
[01:01:57]     }
[01:01:57]     bb6ased mut _1;
[01:01:57]         Validate(Acquire, [(*_6): i32/ReScope(Node(ItemLocalId(10)))]);
[01:01:57]         Validate(Suspend(ReScope(Node(ItemLocalId(10)))), [(*_6): i32/ReScope(Node(ItemLocalId(10)))]);
[01:01:57]         _5 = &ReErased mut (*_6);
[01:01:57]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(10)))]);
[01:01:57]         Validate(Release, [_2: (), _3: &ReScope(Node(ItemLocalId(10))) Test, _5: &ReScope(Node(ItemLocalId(10))) mut i32]);
[01:01:57]         _2 = const Test::foo(move _3, move _5) -> bb1;
[01:01:57]     bb1: {
[01:01:57]     bb1: {
[01:01:57]         Validate(Acquire, [_2: ()]);
[01:01:57]         EndRegion(ReScope(Node(ItemLocalId(10))));
[01:01:57] ... (elided)
[01:01:57]         return;
[01:01:57] }
[01:01:57] Actual:
[01:01:57] Actual:
[01:01:57] fn main() -> (){
[01:01:57]     let mut _0: ();
[01:01:57]     scope 1 {
[01:01:57]         scope 3 {
[01:01:57]         scope 4 {
[01:01:57]         scope 4 {
[01:01:57]             let _7: [closure@NodeId(50)];
[01:01:57]     }
[01:01:57]     scope 2 {
[01:01:57]     scope 2 {
[01:01:57]         let mut _1: i32;
[01:01:57]     }
[01:01:57]     let mut _2: ();
[01:01:57]     let mut _3: &ReErased Test;
[01:01:57]     let mut _4: Test;
[01:01:57]     let mut _5: &ReErased mut i32;
[01:01:57]     let mut _6: &ReErased mut i32;
[01:01:57]     let mut _8: i32;
[01:01:57]     let mut _9: &ReErased [closure@NodeId(50)];
[01:01:57]     let mut _10: (&ReErased mut i32,);
[01:01:57]     let mut _11: &ReErased mut i32;
[01:01:57]     let mut _12: &ReErased mut i32;
[01:01:57]     let mut _13: &ReErased Test;
[01:01:57]     (35)))), [_7: [closure@NodeId(50)]]);
[01:01:57]         _9 = &ReErased _7;
[01:01:57]         Validate(Acquire, [(*_9): [closure@NodeId(50)]/ReScope(Node(ItemLocalId(35))) (imm)]);
[01:01:57]         StorageLive(_10);
[01:01:57]         StorageLive(_11);
[01:01:57]         StorageLive(_12);
[01:01:57]         Validate(Suspend(ReScope(Node(ItemLocalId(35)))), [_1: i32]);
[01:01:57]         _12 = &ReErased mut _1;
[01:01:57]         Validate(Acquire, [(*_12): i32/ReScope(Node(ItemLocalId(35)))]);
[01:01:57]         Validate(Suspend(ReScope(Node(ItemLocalId(35)))), [(*_12): i32/ReScope(Node(ItemLocalId(35)))]);
[01:01:57]         _11 = &ReErased mut (*_12);
[01:01:57]         Validate(Acquire, [(*_11): i32/ReScope(Node(ItemLocalId(35)))]);
[01:01:57]         _10 = (move _11,);
[01:01:57]         Validate(Release, [_8: i32, _9: &ReScope(Node(ItemLocalId(35))) [closure@NodeId(50)], _10: (&ReScope(Node(ItemLocalId(35))) mut i32,)]);
[01:01:57]         _8 = const std::ops::Fn::call(move _9, move _10) -> bb2;
[01:01:57]     }
[01:01:57]     bb2: {                              
[01:01:57]         Validate(Acquire, [_8: i32]);
[01:01:57]         EndRegion(ReScope(Node(ItemLocalId(35))));
[01:01:57]         StorageDead(_10);
[01:01:57]         StorageDead(_11);
[01:01:57]         StorageDead(_9);
[01:01:57]         StorageDead(_12);
[01:01:57]         _0 = ();
[01:01:57]         StorageDead(_7);
[01:01:57]         StorageDead(_1);
[01:01:57]         return;
[01:01:57] }', tools/compiletest/src/runtest.rs:2812:13
[01:01:57] 
[01:01:57] ---- [mir-opt] mir-opt/validate_3.rs stdout ----
[01:0Sun, 03 Jun 2018 06:04:05 GMT
