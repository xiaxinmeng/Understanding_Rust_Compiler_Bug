
[00:47:16] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-cloudabi`
[00:47:16] 
[00:47:19] error[E0063]: missing field `inline_context` in initializer of `sys_common::backtrace::Frame`
[00:47:19]   --> libstd/sys/cloudabi/backtrace.rs:77:29
[00:47:19]    |
[00:47:19] 77 |         cx.frames[cx.idx] = Frame {
[00:47:19]    |                             ^^^^^ missing `inline_context`
[00:47:19] 
[00:47:21] error: aborting due to previous error
[00:47:21] 
[00:47:21] error: Could not compile `std`.
