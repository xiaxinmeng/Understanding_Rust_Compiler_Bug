
[00:18:17] error: variable does not need to be mutable
[00:18:17]    --> /checkout/src/libsyntax/attr.rs:598:65
[00:18:17]     |
[00:18:17] 598 | pub fn eval_condition<F>(cfg: &ast::MetaItem, sess: &ParseSess, mut eval: &mut F)
[00:18:17]     |                                                                 ^^^^^^^^
[00:18:17]     |
[00:18:17] note: lint level defined here
[00:18:17]    --> /checkout/src/libsyntax/lib.rs:21:9
[00:18:17]     |
[00:18:17] 21  | #![deny(warnings)]
[00:18:17]     |         ^^^^^^^^
[00:18:17]     = note: #[deny(unused_mut)] implied by #[deny(warnings)]
[00:18:17] 
[00:18:17] error: aborting due to previous error
[00:18:17] 
[00:18:17] error: Could not compile `syntax`.
