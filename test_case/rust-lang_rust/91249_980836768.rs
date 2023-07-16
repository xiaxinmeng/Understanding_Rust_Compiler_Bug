rust
> macro define_hello() {
>     const hello: u32 = 0;
> }
> 
> const hello: u32 = 1;
> 
> mod inner {
>     const use_hello: u32 = super::hello;
> }
> 