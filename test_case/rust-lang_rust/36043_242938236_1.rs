
> impl ::std::clone::Clone for S {
>     #[inline]
>     fn clone(&self) -> S {
>         match *self {
> 
>             S { a: ref __self_0_0, b: ref __self_0_1 } => { // <== BAD
> 
>                 ::std::clone::assert_receiver_is_clone(&(*__self_0_0));
>                 ::std::clone::assert_receiver_is_clone(&(*__self_0_1));
>                 *self
>             }
>         }
>     }
> }
> impl ::std::marker::Copy for S { }
> struct S {
>     a: u8,
>     b: u8,
> }
> 