rust
[00:17:02] error: variable does not need to be mutable
[00:17:02]    --> /checkout/src/libcore/ops/function.rs:190:41
[00:17:02]     |
[00:17:02] 190 |         extern "rust-call" fn call_once(mut self, args: A) -> F::Output {
[00:17:02]     |                                         ^^^^^^^^
[00:17:02]     |
[00:17:02] note: lint level defined here
[00:17:02]    --> /checkout/src/libcore/lib.rs:68:9
[00:17:02]     |
[00:17:02] 68  | #![deny(warnings)]
[00:17:02]     |         ^^^^^^^^
[00:17:02]     = note: #[deny(unused_mut)] implied by #[deny(warnings)]
[00:17:02] 
[00:17:03] error: variable does not need to be mutable
[00:17:03]    --> /checkout/src/libcore/option.rs:875:18
[00:17:03]     |
[00:17:03] 875 |     fn into_iter(mut self) -> IterMut<'a, T> {
[00:17:03]     |                  ^^^^^^^^
[00:17:03] 
[00:17:03] error: variable does not need to be mutable
[00:17:03]    --> /checkout/src/libcore/result.rs:912:18
[00:17:03]     |
[00:17:03] 912 |     fn into_iter(mut self) -> IterMut<'a, T> {
[00:17:03]     |                  ^^^^^^^^
[00:17:03] 
[00:17:03] error: aborting due to 3 previous errors
[00:17:03] 
[00:17:03] error: Could not compile `core`.
