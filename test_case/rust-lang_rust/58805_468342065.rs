
> error: redundant import                                                                                                     
>   --> src/libstd/rt.rs:24:9
>    |
> 24 |     use panic;
>    |         ^^^^^
>    |
> note: the other one
>   --> src/libstd/macros.rs:58:1
>    |
> 58 | / macro_rules! panic {
> 59 | |     () => ({
> 60 | |         panic!("explicit panic")
> 61 | |     });
> ...  |
> 71 | |     });
> 72 | | }
>    | |_^
> 
> error: aborting due to previous error
> 
> error: Could not compile `std`.
> 