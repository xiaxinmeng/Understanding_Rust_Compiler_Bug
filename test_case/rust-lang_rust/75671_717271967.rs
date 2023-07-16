
> stderr:
> ------------------------------------------
> error[E0308]: mismatched types
>   --> /checkout/src/test/ui/lint/lint-temporary-cstring-as-param.rs:8:19
>    |
> LL |     some_function(CString::new("").unwrap().as_ptr());
>    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i8`, found `u8`
>    |
>    = note: expected raw pointer `*const i8`
>               found raw pointer `*const u8`
> 
> error: aborting due to previous error
> 