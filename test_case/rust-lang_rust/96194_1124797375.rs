rust
>type Assoc = impl Copy;
>fn f() -> impl for<'a> Tr<'a, Assoc = Assoc> {}
>