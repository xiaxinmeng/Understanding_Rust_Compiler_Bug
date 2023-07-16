rust
>     let buffer = Vec::<i8>::new().as_mut_ptr();
>     unsafe {
>         _ = CString::from_raw(buffer);
>     }
> 