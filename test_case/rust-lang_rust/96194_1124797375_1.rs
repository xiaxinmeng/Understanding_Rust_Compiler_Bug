rust
>type Assoc<'a> = impl Copy;
>fn f() -> impl for<'a> Tr<'a, Assoc = Assoc<'a>> {}
>