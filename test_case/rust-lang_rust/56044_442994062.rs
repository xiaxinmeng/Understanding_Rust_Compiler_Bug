rust
> fn foo(a: (LogDrop, LogDrop), b: (LogDrop, LogDrop)) {
>     let (_x, _) = a;
>     let (_, _y) = b;
>     // ...
> }
> 