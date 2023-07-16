
A libstd for Miri is now available in `/Users/user/Library/Caches/org.rust-lang.miri/HOST`.

running 3 tests
test range_map::tests::basic_insert ... ok
test intptrcast::tests::test_align_addr ... ok
test range_map::tests::gaps ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 200 tests
normalized stdout:
2:rustc[2mrustc_mir::interpret::eval_context[0m::[1;32mframe[0m main
2:rustc[2mrustc_mir::interpret::eval_context[0m::[1;32mframe[0m main
2:rustc[1;34mDEBUG[0m [2mrustc_middle::mir::interpret[0m creating alloc Function(Instance { def: Item(WithOptConstParam { did: DefId(0:8 ~ align[317d]::main), const_param_did: None }), substs: [] }) with id alloc5
2:rustc[2mrustc_mir::interpret::eval_context[0m::[1;32mframe[0m std::rt::lang_start::<()>
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m StorageLive(_4)
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m StorageLive(_5)
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m StorageLive(_6)
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m StorageLive(_7)
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m (_7.0: fn() -> T) = _1
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m Retag(_7)
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m _6 = &_7
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m Retag(_6)
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m _5 = &(*_6)
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m Retag(_5)
2:rustc├─[2m12[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m _4 = move _5 as &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe (Pointer(Unsize))
2:rustc├─[2m12[0m[2mms[0m [1;34mDEBUG[0m [2mrustc_mir::interpret::util[0m ensure_monomorphic_enough: ty=[closure@std::rt::lang_start<()>::{closure#0}]
2:rustc├─[2m12[0m[2mms[0m [1;34mDEBUG[0m [2mrustc_mir::interpret::util[0m ensure_monomorphic_enough: ty=Some(Binder(std::ops::Fn<()>))
2:rustc├─[2m16[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m Retag(_4)
2:rustc├─[2m16[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m StorageDead(_5)
2:rustc├─[2m16[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m StorageLive(_8)
2:rustc├─[2m16[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m _8 = _2
2:rustc├─[2m16[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m StorageLive(_9)
2:rustc├─[2m16[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m _9 = _3
2:rustc├─[2m16[0m[2mms[0m [1;32m INFO[0m [2mrustc_mir::interpret::step[0m _0 = std::rt::lang_start_internal(move _4, move _8, move _9) -> bb1
2:rustc├┐[2mrustc_mir::interpret::eval_context[0m::[1;32mframe[0m std::rt::lang_start::<()>
2
