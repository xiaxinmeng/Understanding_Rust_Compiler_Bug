
[00:19:55] error: variable does not need to be mutable
[00:19:55]    --> /checkout/src/librustc/infer/error_reporting/anon_anon_conflict.rs:261:21
[00:19:55]     |
[00:19:55] 261 |                 let mut subvisitor = &mut TyPathVisitor {
[00:19:55]     |                     ^^^^^^^^^^^^^^
[00:19:55]     |
[00:19:55] note: lint level defined here
[00:19:55]    --> /checkout/src/librustc/lib.rs:23:9
[00:19:55]     |
[00:19:55] 23  | #![deny(warnings)]
[00:19:55]     |         ^^^^^^^^
[00:19:55]     = note: #[deny(unused_mut)] implied by #[deny(warnings)]
[00:19:55] 
[00:19:56] error: aborting due to previous error
[00:19:56] 
[00:19:56] error: Could not compile `rustc`.
[00:19:56] 
