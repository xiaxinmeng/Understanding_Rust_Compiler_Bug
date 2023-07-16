plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..........
failures:

---- [mir-opt] src/test/mir-opt/check-maybe-uninit.rs stdout ----
2 + // MIR for `main` after CheckMaybeUninit
4   | User Type Annotations
4   | User Type Annotations
-   | 0: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(2:2022 ~ core[4f75]::mem::maybe_uninit::{impl#2}::uninit), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:2019 ~ core[4f75]::mem::maybe_uninit::{impl#2}), self_ty: std::mem::MaybeUninit<u8> }) }) }, span: $DIR/check-maybe-uninit.rs:6:17: 6:42, inferred_ty: fn() -> std::mem::MaybeUninit<u8> {std::mem::MaybeUninit::<u8>::uninit}
-   | 1: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(2:2022 ~ core[4f75]::mem::maybe_uninit::{impl#2}::uninit), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:2019 ~ core[4f75]::mem::maybe_uninit::{impl#2}), self_ty: std::mem::MaybeUninit<std::string::String> }) }) }, span: $DIR/check-maybe-uninit.rs:7:17: 7:46, inferred_ty: fn() -> std::mem::MaybeUninit<std::string::String> {std::mem::MaybeUninit::<std::string::String>::uninit}
+   | 0: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(2:2022 ~ core[8554]::mem::maybe_uninit::{impl#2}::uninit), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:2019 ~ core[8554]::mem::maybe_uninit::{impl#2}), self_ty: std::mem::MaybeUninit<u8> }) }) }, span: $DIR/check-maybe-uninit.rs:6:17: 6:42, inferred_ty: fn() -> std::mem::MaybeUninit<u8> {std::mem::MaybeUninit::<u8>::uninit}
+   | 1: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(2:2022 ~ core[8554]::mem::maybe_uninit::{impl#2}::uninit), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:2019 ~ core[8554]::mem::maybe_uninit::{impl#2}), self_ty: std::mem::MaybeUninit<std::string::String> }) }) }, span: $DIR/check-maybe-uninit.rs:7:17: 7:46, inferred_ty: fn() -> std::mem::MaybeUninit<std::string::String> {std::mem::MaybeUninit::<std::string::String>::uninit}
8   fn main() -> () {
8   fn main() -> () {
9       let mut _0: ();                      // return place in scope 0 at $DIR/check-maybe-uninit.rs:+0:11: +0:11

thread '[mir-opt] src/test/mir-opt/check-maybe-uninit.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/check_maybe_uninit.main.CheckMaybeUninit.diff', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/check-maybe-uninit.rs
