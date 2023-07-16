rust
> trait T {}
> struct S;
> impl S {
>     async fn f(_a: &S, _b:&S, _c: Box<dyn T>) {}
> }
> 