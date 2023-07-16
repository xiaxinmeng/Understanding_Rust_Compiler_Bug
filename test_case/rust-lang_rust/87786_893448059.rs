rust
> macro_rules! foo {
>     ($bind:ident) => {
>         let $bind = &$bind;
>     }
> }
> 
> fn main() {
>     let bar = 42;
>     foo!(bar);
> }
> 