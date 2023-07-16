 rust
> fn test(foo: Foo) {
>     foo[()]; // OK
>     foo[0];  // OK
>     foo.deref()[0]; // OK
> }
> 