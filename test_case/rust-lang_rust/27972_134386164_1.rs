
<anon>:2:1: 2:51 warning: the trait `ZZ` is not implemented for the type `core::fmt::Display + 'static` [E0277]
<anon>:2 fn _f<T:'static>() where std::fmt::Display : ZZ {}
         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:2:1: 2:51 help: run `rustc --explain E0277` to see a detailed explanation
<anon>:2:1: 2:51 note: this warning results from recent bug fixes and clarifications; it will become a HARD ERROR in the next release. See RFC 1214 for details.
<anon>:2 fn _f<T:'static>() where std::fmt::Display : ZZ {}
         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:2:1: 2:51 note: required by `ZZ`
<anon>:2 fn _f<T:'static>() where std::fmt::Display : ZZ {}
         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
