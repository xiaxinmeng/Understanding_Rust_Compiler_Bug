rust
> #![feature(never_type_fallback)]
> fn main() {
>     let a = return;
>     let b = a + 1; // no implementation for `! + i32`
> }
> 