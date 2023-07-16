
A libstd for Miri is now available in `/Users/user/Library/Caches/org.rust-lang.miri/HOST`.

running 3 tests
test intptrcast::tests::test_align_addr ... ok
test range_map::tests::gaps ... ok
test range_map::tests::basic_insert ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 200 tests
normalized stdout:
2:rustcrustc_mir::interpret::eval_context::frame main
2:rustcrustc_mir::interpret::eval_context::frame main
2:rustcDEBUG rustc_middle::mir::interpret creating alloc Function(Instance { def: Item(WithOptConstParam { did: DefId(0:8 ~ align[317d]::main), const_param_did: None }), substs: [] }) with id alloc5
2:rustcrustc_mir::interpret::eval_context::frame std::rt::lang_start::<()>
2:rustc├─1395ms INFO rustc_mir::interpret::step StorageLive(_4)
2:rustc├─1395ms INFO rustc_mir::interpret::step StorageLive(_5)
2:rustc├─1395ms INFO rustc_mir::interpret::step StorageLive(_6)
2:rustc├─1395ms INFO rustc_mir::interpret::step StorageLive(_7)
2:rustc├─1395ms INFO rustc_mir::interpret::step (_7.0: fn() -> T) = _1
2:rustc├─1396ms INFO rustc_mir::interpret::step Retag(_7)
2:rustc├─1396ms INFO rustc_mir::interpret::step _6 = &_7
2:rustc├─1396ms INFO rustc_mir::interpret::step Retag(_6)
2:rustc├─1452ms INFO rustc_mir::interpret::step _5 = &(*_6)
2:rustc├─1452ms INFO rustc_mir::interpret::step Retag(_5)
2:rustc├─1452ms INFO rustc_mir::interpret::step _4 = move _5 as &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe (Pointer(Unsize))
2:rustc├─1454ms DEBUG rustc_mir::interpret::util ensure_monomorphic_enough: ty=[closure@std::rt::lang_start<()>::{closure#0}]
2:rustc├─1454ms DEBUG rustc_mir::interpret::util ensure_monomorphic_enough: ty=Some(Binder(std::ops::Fn<()>))
2:rustc├─1614ms INFO rustc_mir::interpret::step Retag(_4)
2:rustc├─1614ms INFO rustc_mir::interpret::step StorageDead(_5)
2:rustc├─1614ms INFO rustc_mir::interpret::step StorageLive(_8)
2:rustc├─1614ms INFO rustc_mir::interpret::step _8 = _2
2:rustc├─1614ms INFO rustc_mir::interpret::step StorageLive(_9)
2:rustc├─1614ms INFO rustc_mir::interpret::step _9 = _3
2:rustc├─1614ms INFO rustc_mir::interpret::step _0 = std::rt::lang_start_internal(move _4, move _8, move _9) -> bb1
2:rustc├┐rustc_mir::interpret::eval_context::frame std::rt::lang_start::<()>
