plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `i` in this scope
   --> compiler/rustc_lint/src/early.rs:431:23
    |
431 |         let is_last = i == (pass_count - 1);
    |
   ::: /checkout/compiler/rustc_ast/src/ptr.rs:39:1
    |
    |
39  | pub fn P<T: 'static>(value: T) -> P<T> {
    | -------------------------------------- similarly named function `P` defined here
help: a function with a similar name exists
    |
    |
431 |         let is_last = P == (pass_count - 1);
help: consider importing this constant
    |
17  | use rustc_span::sym::i;
    |
