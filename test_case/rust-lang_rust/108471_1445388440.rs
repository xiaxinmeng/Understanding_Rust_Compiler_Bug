plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
---- [mir-opt] tests/mir-opt/building/uniform_array_move_out.rs stdout ----
1 // MIR for `move_out_from_end` after built
2 
3 | User Type Annotations
- | 0: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(5:296 ~ alloc[2c0a]::boxed::{impl#0}::new), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:294 ~ alloc[2c0a]::boxed::{impl#0}), self_ty: std::boxed::Box<^1, ^2> }) }) }, span: $DIR/uniform_array_move_out.rs:3:14: 3:22, inferred_ty: fn(i32) -> std::boxed::Box<i32> {std::boxed::Box::<i32>::new}
- | 1: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(5:296 ~ alloc[2c0a]::boxed::{impl#0}::new), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:294 ~ alloc[2c0a]::boxed::{impl#0}), self_ty: std::boxed::Box<^1, ^2> }) }) }, span: $DIR/uniform_array_move_out.rs:3:27: 3:35, inferred_ty: fn(i32) -> std::boxed::Box<i32> {std::boxed::Box::<i32>::new}
+ | 0: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(5:296 ~ alloc[e1cc]::boxed::{impl#0}::new), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:294 ~ alloc[e1cc]::boxed::{impl#0}), self_ty: std::boxed::Box<^1, ^2> }) }) }, span: $DIR/uniform_array_move_out.rs:3:14: 3:22, inferred_ty: fn(i32) -> std::boxed::Box<i32> {std::boxed::Box::<i32>::new}
+ | 1: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(5:296 ~ alloc[e1cc]::boxed::{impl#0}::new), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:294 ~ alloc[e1cc]::boxed::{impl#0}), self_ty: std::boxed::Box<^1, ^2> }) }) }, span: $DIR/uniform_array_move_out.rs:3:27: 3:35, inferred_ty: fn(i32) -> std::boxed::Box<i32> {std::boxed::Box::<i32>::new}
7 fn move_out_from_end() -> () {
7 fn move_out_from_end() -> () {
8     let mut _0: ();                      // return place in scope 0 at $DIR/uniform_array_move_out.rs:+0:24: +0:24

thread '[mir-opt] tests/mir-opt/building/uniform_array_move_out.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/building/uniform_array_move_out.move_out_from_end.built.after.mir', src/tools/compiletest/src/runtest.rs:3481:21


failures:
    [mir-opt] tests/mir-opt/building/uniform_array_move_out.rs
