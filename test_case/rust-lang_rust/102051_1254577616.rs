plain
test [mir-opt] src/test/mir-opt/inline/polymorphic-recursion.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/issue-101867.rs stdout ----
1 // MIR for `main` 0 mir_map
3 | User Type Annotations
3 | User Type Annotations
- | 0: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(std::option::Option<u8>) }, span: $DIR/issue-101867.rs:5:12: 5:22, inferred_ty: std::option::Option<u8>
- | 1: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(std::option::Option<u8>) }, span: $DIR/issue-101867.rs:5:12: 5:22, inferred_ty: std::option::Option<u8>
+ | 0: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(std::option::Option<u8>) }, span: $DIR/issue-101867.rs:3:12: 3:22, inferred_ty: std::option::Option<u8>
+ | 1: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(std::option::Option<u8>) }, span: $DIR/issue-101867.rs:3:12: 3:22, inferred_ty: std::option::Option<u8>
7 fn main() -> () {
7 fn main() -> () {
8     let mut _0: ();                      // return place in scope 0 at $DIR/issue-101867.rs:+0:11: +0:11

thread '[mir-opt] src/test/mir-opt/issue-101867.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_101867.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25


failures:
    [mir-opt] src/test/mir-opt/issue-101867.rs
