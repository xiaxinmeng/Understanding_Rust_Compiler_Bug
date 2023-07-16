rust
> error[E0308]: mismatched types
>   --> src/error.rs:24:11
>    |
> 24 |     name: b"RUSTMSG_TYPE_ERROR\0" as *const _ as *const libc::c_char,
>    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u8, found i8

> The issue is that the generated bindings use std::os::raw::c_char, and our code uses libc::c_char. In the past these types have been identical, and now they are not for some reason.