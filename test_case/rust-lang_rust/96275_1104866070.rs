rust
> #![feature(core_intrinsics)]
> 
> use std::intrinsics::unlikely;
> 
> pub fn foo(x: bool) {
>     if unlikely(x) {
>         println!("foo: {x}");
>     }
>     println!("bar");
> }
> 