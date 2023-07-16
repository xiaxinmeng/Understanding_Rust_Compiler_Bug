rust
> fn foo() -> impl FnMut() -> i32 {
>     let mut bar = 0;
>     move || { bar += 1; bar }
> }
> 