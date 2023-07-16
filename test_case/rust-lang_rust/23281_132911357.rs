
<anon>:4:5: 4:46 warning: the trait `core::marker::Sized` is not implemented for the type `core::ops::Fn() + 'static` [E0277]
<anon>:4     pub fn function(funs: Vec<Fn() -> ()>) {}
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:4:5: 4:46 help: see the detailed explanation for E0277
<anon>:4:5: 4:46 note: `core::ops::Fn() + 'static` does not have a constant size known at compile-time
<anon>:4     pub fn function(funs: Vec<Fn() -> ()>) {}
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:4:5: 4:46 note: this warning results from recent bug fixes and clarifications; it will become a HARD ERROR in the next release. See RFC 1214 for details.
<anon>:4     pub fn function(funs: Vec<Fn() -> ()>) {}
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
