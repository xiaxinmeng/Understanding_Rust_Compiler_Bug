
> error: constant expression depends on a generic parameter
>   --> src/lib.rs:12:5
>    |
> 12 |     [(); my_cast({ SIXTEEN- IMM8 })]: ,           
>    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
>    |
>    = note: this may fail depending on what value the parameter takes
> 