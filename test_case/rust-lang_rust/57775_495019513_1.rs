rust
> trait Foo: 'static {
>     fn singleton() -> &'static Self {
>         unimplemented!()
>     }
> }
> 
> fn x<T: Default + Foo>() {
>     let a = T::singleton();
> }
> 