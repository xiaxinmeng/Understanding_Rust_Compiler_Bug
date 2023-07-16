rust
> struct Foo<T>(T);
> impl<T> Foo<T> {
>     const BAR: usize = std::mem::size_of::<T>();
> }
> 
> fn muh<T>() {
>     match 42 {
>         Foo::<T>::BAR => {}
>         _ => {}
>     }
> }
> 