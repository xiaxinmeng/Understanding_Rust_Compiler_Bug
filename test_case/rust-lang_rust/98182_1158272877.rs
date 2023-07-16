rust
> #[derive(Serialize)]
> struct Foo {
>     #[doc = include_str!("x.md")]
>     x: u32
> }
> 