
> error[E0582]: binding for associated type `Output` references lifetime `'r`, which does not appear in the trait input types
>   --> file.rs on line 5:35
>    |
> 10 | fn bug() -> impl for <'r> Fn() -> &'r () { || { &() } }
>    |                                   ^^^^^^
> 
> error: aborting due to previous error
> 
> For more information about this error, try `rustc --explain E0582`.
> 