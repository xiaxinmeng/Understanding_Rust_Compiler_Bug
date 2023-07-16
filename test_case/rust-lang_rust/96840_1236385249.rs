plain

---- [mir-opt] src/test/mir-opt/issue-99325.rs stdout ----
2 
3 | User Type Annotations
4 | 0: user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_99325[8f58]::function_with_bytes), UserSubsts { substs: [Const { ty: &'static [u8; 4], kind: Value(Branch([Leaf(0x41), Leaf(0x41), Leaf(0x41), Leaf(0x41)])) }], user_self_ty: None }) }, span: $DIR/issue-99325.rs:10:16: 10:46, inferred_ty: fn() -> &'static [u8] {function_with_bytes::<&*b"AAAA">}
- | 1: user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_99325[8f58]::function_with_bytes), UserSubsts { substs: [Const { ty: &'static [u8; 4], kind: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ issue_99325[8f58]::main::{constant#1}), const_param_did: Some(DefId(0:4 ~ issue_99325[8f58]::function_with_bytes::BYTES)) }, substs: [], promoted: None }) }], user_self_ty: None }) }, span: $DIR/issue-99325.rs:11:16: 11:68, inferred_ty: fn() -> &'static [u8] {function_with_bytes::<&*b"AAAA">}
+ | 1: user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_99325[8f58]::function_with_bytes), UserSubsts { substs: [Const { ty: &'static [u8; 4], kind: Unevaluated(Unevaluated { def: DefId(0:8 ~ issue_99325[8f58]::main::{constant#1}), substs: [], promoted: None }) }], user_self_ty: None }) }, span: $DIR/issue-99325.rs:11:16: 11:68, inferred_ty: fn() -> &'static [u8] {function_with_bytes::<&*b"AAAA">}
7 fn main() -> () {
7 fn main() -> () {
8     let mut _0: ();                      // return place in scope 0 at $DIR/issue-99325.rs:+0:15: +0:15

thread '[mir-opt] src/test/mir-opt/issue-99325.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_99325.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/issue-99325.rs
