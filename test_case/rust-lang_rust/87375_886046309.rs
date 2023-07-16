plain
   Compiling miniz_oxide v0.4.0
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling object v0.22.0
   Compiling hashbrown v0.11.0
error: internal compiler error: broken MIR in DefId(0:1376 ~ alloc[7d3b]::collections::btree::map::{impl#13}::split_off) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(0:1340 ~ alloc[7d3b]::collections::btree::map::{impl#13}::new), UserSubsts { substs: [^0, ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:1337 ~ alloc[7d3b]::collections::btree::map::{impl#13}), self_ty: collections::btree::map::BTreeMap<K, V> }) }) }, span: library/alloc/src/collections/btree/map.rs:1166:20: 1166:29 (#0), inferred_ty: fn() -> collections::btree::map::BTreeMap<K, V> {collections::btree::map::BTreeMap::<K, V>::new} }): bad user type AscribeUserType(fn() -> collections::btree::map::BTreeMap<K, V> {collections::btree::map::BTreeMap::<K, V>::new}, DefId(0:1340 ~ alloc[7d3b]::collections::btree::map::{impl#13}::new) UserSubsts { substs: [_, _], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:1337 ~ alloc[7d3b]::collections::btree::map::{impl#13}), self_ty: collections::btree::map::BTreeMap<K, V> }) }): NoSolution
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


error: internal compiler error: broken MIR in DefId(0:1703 ~ alloc[7d3b]::collections::btree::map::{impl#68}::from_iter) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(0:1340 ~ alloc[7d3b]::collections::btree::map::{impl#13}::new), UserSubsts { substs: [^0, ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:1337 ~ alloc[7d3b]::collections::btree::map::{impl#13}), self_ty: collections::btree::map::BTreeMap<^2, ^3> }) }) }, span: library/alloc/src/collections/btree/map.rs:1913:23: 1913:36 (#0), inferred_ty: fn() -> collections::btree::map::BTreeMap<K, V> {collections::btree::map::BTreeMap::<K, V>::new} }): bad user type AscribeUserType(fn() -> collections::btree::map::BTreeMap<K, V> {collections::btree::map::BTreeMap::<K, V>::new}, DefId(0:1340 ~ alloc[7d3b]::collections::btree::map::{impl#13}::new) UserSubsts { substs: [_, _], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:1337 ~ alloc[7d3b]::collections::btree::map::{impl#13}), self_ty: collections::btree::map::BTreeMap<_, _> }) }): NoSolution
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


error: internal compiler error: broken MIR in DefId(0:1728 ~ alloc[7d3b]::collections::btree::map::{impl#72}::default) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(0:1340 ~ alloc[7d3b]::collections::btree::map::{impl#13}::new), UserSubsts { substs: [^0, ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:1337 ~ alloc[7d3b]::collections::btree::map::{impl#13}), self_ty: collections::btree::map::BTreeMap<^2, ^3> }) }) }, span: library/alloc/src/collections/btree/map.rs:1959:9: 1959:22 (#0), inferred_ty: fn() -> collections::btree::map::BTreeMap<K, V> {collections::btree::map::BTreeMap::<K, V>::new} }): bad user type AscribeUserType(fn() -> collections::btree::map::BTreeMap<K, V> {collections::btree::map::BTreeMap::<K, V>::new}, DefId(0:1340 ~ alloc[7d3b]::collections::btree::map::{impl#13}::new) UserSubsts { substs: [_, _], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:1337 ~ alloc[7d3b]::collections::btree::map::{impl#13}), self_ty: collections::btree::map::BTreeMap<_, _> }) }): NoSolution
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


error: internal compiler error: broken MIR in DefId(0:2861 ~ alloc[7d3b]::collections::btree::set::{impl#7}::from_iter) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(0:2812 ~ alloc[7d3b]::collections::btree::set::{impl#6}::new), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:2810 ~ alloc[7d3b]::collections::btree::set::{impl#6}), self_ty: collections::btree::set::BTreeSet<^1> }) }) }, span: library/alloc/src/collections/btree/set.rs:1054:23: 1054:36 (#0), inferred_ty: fn() -> collections::btree::set::BTreeSet<T> {collections::btree::set::BTreeSet::<T>::new} }): bad user type AscribeUserType(fn() -> collections::btree::set::BTreeSet<T> {collections::btree::set::BTreeSet::<T>::new}, DefId(0:2812 ~ alloc[7d3b]::collections::btree::set::{impl#6}::new) UserSubsts { substs: [_], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:2810 ~ alloc[7d3b]::collections::btree::set::{impl#6}), self_ty: collections::btree::set::BTreeSet<_> }) }): NoSolution
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


error: internal compiler error: broken MIR in DefId(0:2915 ~ alloc[7d3b]::collections::btree::set::{impl#16}::default) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(0:2812 ~ alloc[7d3b]::collections::btree::set::{impl#6}::new), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:2810 ~ alloc[7d3b]::collections::btree::set::{impl#6}), self_ty: collections::btree::set::BTreeSet<^1> }) }) }, span: library/alloc/src/collections/btree/set.rs:1176:9: 1176:22 (#0), inferred_ty: fn() -> collections::btree::set::BTreeSet<T> {collections::btree::set::BTreeSet::<T>::new} }): bad user type AscribeUserType(fn() -> collections::btree::set::BTreeSet<T> {collections::btree::set::BTreeSet::<T>::new}, DefId(0:2812 ~ alloc[7d3b]::collections::btree::set::{impl#6}::new) UserSubsts { substs: [_], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:2810 ~ alloc[7d3b]::collections::btree::set::{impl#6}), self_ty: collections::btree::set::BTreeSet<_> }) }): NoSolution
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1050:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (056af0f22 2021-07-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `alloc`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in DefId(0:8198 ~ core[b3ca]::pin::{impl#13}::deref_mut) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(0:8181 ~ core[b3ca]::pin::{impl#9}::get_mut), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0) }), ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:8177 ~ core[b3ca]::pin::{impl#9}), self_ty: pin::Pin<^2> }) }) }, span: library/core/src/pin.rs:830:9: 830:21 (#0), inferred_ty: fn(pin::Pin<&mut <P as ops::deref::Deref>::Target>) -> &mut <P as ops::deref::Deref>::Target {pin::Pin::<&mut <P as ops::deref::Deref>::Target>::get_mut} }): bad user type AscribeUserType(fn(pin::Pin<&mut <P as ops::deref::Deref>::Target>) -> &mut <P as ops::deref::Deref>::Target {pin::Pin::<&mut <P as ops::deref::Deref>::Target>::get_mut}, DefId(0:8181 ~ core[b3ca]::pin::{impl#9}::get_mut) UserSubsts { substs: ['_#16r, _], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(0:8177 ~ core[b3ca]::pin::{impl#9}), self_ty: pin::Pin<_> }) }): NoSolution
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1050:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (056af0f22 2021-07-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: internal compiler error: broken MIR in DefId(0:1169 ~ gimli[21e7]::read::abbrev::{impl#5}::empty) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(5:1340 ~ alloc[7d3b]::collections::btree::map::{impl#13}::new), UserSubsts { substs: [^0, ^1], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:1337 ~ alloc[7d3b]::collections::btree::map::{impl#13}), self_ty: alloc::collections::BTreeMap<^2, ^3> }) }) }, span: /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/read/abbrev.rs:119:18: 119:42 (#0), inferred_ty: fn() -> alloc::collections::BTreeMap<u64, read::abbrev::Abbreviation> {alloc::collections::BTreeMap::<u64, read::abbrev::Abbreviation>::new} }): bad user type AscribeUserType(fn() -> alloc::collections::BTreeMap<u64, read::abbrev::Abbreviation> {alloc::collections::BTreeMap::<u64, read::abbrev::Abbreviation>::new}, DefId(5:1340 ~ alloc[7d3b]::collections::btree::map::{impl#13}::new) UserSubsts { substs: [_, _], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:1337 ~ alloc[7d3b]::collections::btree::map::{impl#13}), self_ty: alloc::collections::BTreeMap<_, _> }) }): NoSolution
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1050:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (056af0f22 2021-07-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
