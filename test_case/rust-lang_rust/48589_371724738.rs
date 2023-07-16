rust
> macro_rules! foo {
>     ($i:ident) => {
>         $i Foo {
>             x: u32,
>             y: i32,
>         }
>     }
> }
> 
> foo!(union);
> 