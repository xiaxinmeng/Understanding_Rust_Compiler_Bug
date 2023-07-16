rust
> type Type<T> = <T as Trait>::Type;
> 
> fn g<'a>(_: Type<&'a ()>) -> &'a str {
>     ""
> }
> 