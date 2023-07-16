rust
> #![feature(type_alias_impl_trait)]
> 
> type Opaque<'a> = impl Fn(&'a str) + 'static;
> 
> fn test<'a>() -> Opaque<'a> {
>     |_| {}
> }
> 