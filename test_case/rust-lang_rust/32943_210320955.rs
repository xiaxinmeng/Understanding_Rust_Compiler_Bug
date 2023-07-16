
<anon>:3:10: 3:11 error: the trait `core::marker::Sized` is not implemented for the type `str` [E0277]
<anon>:3     let &a = "a";
                  ^
<anon>:3:10: 3:11 help: see the detailed explanation for E0277
<anon>:3:10: 3:11 note: `str` does not have a constant size known at compile-time
<anon>:3:10: 3:11 note: all local variables must have a statically known size
