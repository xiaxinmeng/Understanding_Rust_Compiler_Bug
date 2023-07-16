plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 219 tests
i......................................................i................................ 88/219
............................i.................i................................F......F. 176/219
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
failures:

---- [mir-opt] tests/mir-opt/issue_99325.rs stdout ----
1 // MIR for `main` after built
1 // MIR for `main` after built
2 
3 | User Type Annotations
- | 0: user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_99325[8f58]::function_with_bytes), UserSubsts { substs: [Const { ty: Ref(ReStatic, Array(Uint(U8), Const { ty: Uint(Usize), kind: Value(Leaf(0x0000000000000004)) }), Not), kind: Value(Branch([Leaf(0x41), Leaf(0x41), Leaf(0x41), Leaf(0x41)])) }], user_self_ty: None }) }, span: $DIR/issue_99325.rs:10:16: 10:46, inferred_ty: fn() -> &'static [u8] {function_with_bytes::<&*b"AAAA">}
- | 1: user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_99325[8f58]::function_with_bytes), UserSubsts { substs: [Const { ty: Ref(ReStatic, Array(Uint(U8), Const { ty: Uint(Usize), kind: Value(Leaf(0x0000000000000004)) }), Not), kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:8 ~ issue_99325[8f58]::main::{constant#1}), const_param_did: Some(DefId(0:4 ~ issue_99325[8f58]::function_with_bytes::BYTES)) }, substs: [] }) }], user_self_ty: None }) }, span: $DIR/issue_99325.rs:11:16: 11:68, inferred_ty: fn() -> &'static [u8] {function_with_bytes::<&*b"AAAA">}
+ | 0: user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_99325[8f58]::function_with_bytes), UserSubsts { substs: [Const { ty: Ref(ReStatic, Array(Uint(U8), Const { ty: Uint(Usize), kind: Value(Leaf(0x00000004)) }), Not), kind: Value(Branch([Leaf(0x41), Leaf(0x41), Leaf(0x41), Leaf(0x41)])) }], user_self_ty: None }) }, span: $DIR/issue_99325.rs:10:16: 10:46, inferred_ty: fn() -> &'static [u8] {function_with_bytes::<&*b"AAAA">}
+ | 1: user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_99325[8f58]::function_with_bytes), UserSubsts { substs: [Const { ty: Ref(ReStatic, Array(Uint(U8), Const { ty: Uint(Usize), kind: Value(Leaf(0x00000004)) }), Not), kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:8 ~ issue_99325[8f58]::main::{constant#1}), const_param_did: Some(DefId(0:4 ~ issue_99325[8f58]::function_with_bytes::BYTES)) }, substs: [] }) }], user_self_ty: None }) }, span: $DIR/issue_99325.rs:11:16: 11:68, inferred_ty: fn() -> &'static [u8] {function_with_bytes::<&*b"AAAA">}
7 fn main() -> () {
7 fn main() -> () {
8     let mut _0: ();                      // return place in scope 0 at $DIR/issue_99325.rs:+0:15: +0:15

thread '[mir-opt] tests/mir-opt/issue_99325.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_99325.main.built.after.mir', src/tools/compiletest/src/runtest.rs:3463:21

---- [mir-opt] tests/mir-opt/nll/region_subtyping_basic.rs stdout ----
22 |
23 fn main() -> () {
23 fn main() -> () {
24     let mut _0: ();                      // return place in scope 0 at $DIR/region_subtyping_basic.rs:+0:11: +0:11
-     let mut _1: [usize; Const(Value(Leaf(0x00000003)): usize)]; // in scope 0 at $DIR/region_subtyping_basic.rs:+1:9: +1:14
+     let mut _1: [usize; Const(Value(Leaf(0x00000003)): Uint(Usize))]; // in scope 0 at $DIR/region_subtyping_basic.rs:+1:9: +1:14
26     let _3: usize;                       // in scope 0 at $DIR/region_subtyping_basic.rs:+2:16: +2:17
27     let mut _4: usize;                   // in scope 0 at $DIR/region_subtyping_basic.rs:+2:14: +2:18
28     let mut _5: bool;                    // in scope 0 at $DIR/region_subtyping_basic.rs:+2:14: +2:18

thread '[mir-opt] tests/mir-opt/nll/region_subtyping_basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3463:21

failures:
    [mir-opt] tests/mir-opt/issue_99325.rs
    [mir-opt] tests/mir-opt/nll/region_subtyping_basic.rs
