
<anon>:7:13: 7:25 error: the trait `core::marker::Sized` is not implemented for the type `str` [E0277]
<anon>:7     let s = S { s: *hi };
                     ^~~~~~~~~~~~
<anon>:7:13: 7:25 note: `str` does not have a constant size known at compile-time
<anon>:7     let s = S { s: *hi };
                     ^~~~~~~~~~~~
error: aborting due to previous error
playpen: application terminated with error code 101
